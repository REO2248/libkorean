use crate::char_utils::initial_sound_to_compat_initial;
use crate::engine::{
    initial_to_final, Final, FinalToInitial, Initial, InputOptions, KeyValue, Medial,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterResult {
    Consume,
    NewCharacter(CharacterState),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CharacterState {
    initial_sound: Option<Initial>,
    medial_sound: Option<Medial>,
    compose_jung: bool,
    final_sound: Option<Final>,
    final_kv: Option<KeyValue>,
    history: Vec<KeyValue>,
}

impl CharacterState {
    pub const fn new() -> Self {
        Self {
            initial_sound: None,
            medial_sound: None,
            compose_jung: false,
            final_sound: None,
            final_kv: None,
            history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.initial_sound = None;
        self.medial_sound = None;
        self.final_sound = None;
        self.final_kv = None;
        self.compose_jung = false;
        self.history.clear();
    }

    pub const fn need_display(&self) -> bool {
        self.initial_sound.is_some() || self.medial_sound.is_some() || self.final_sound.is_some()
    }

    pub const fn has_initial(&self) -> bool {
        self.initial_sound.is_some()
    }
    pub const fn has_medial(&self) -> bool {
        self.medial_sound.is_some()
    }
    pub const fn has_final(&self) -> bool {
        self.final_sound.is_some()
    }

    pub fn history(&self) -> &[KeyValue] {
        &self.history
    }

    pub fn preedit(&self, out: &mut String) {
        match (self.initial_sound, self.medial_sound, self.final_sound) {
            (Some(c), Some(j), jo) => out.push_str(&c.compose(j, jo)),
            (Some(c), None, None) => out.push(initial_sound_to_compat_initial(c.into())),
            (None, Some(j), None) => out.push(initial_sound_to_compat_initial(j.into())),
            (None, None, Some(jo)) => out.push(initial_sound_to_compat_initial(jo.into())),
            (None, Some(j), Some(jo)) => {
                out.push(j.jamo());
                out.push(jo.jamo());
            }
            (Some(c), None, Some(jo)) => {
                out.push(c.jamo());
                out.push(jo.jamo());
            }
            (None, None, None) => {}
        }
    }

    pub fn jamo(&self, out: &mut String) {
        if let Some(c) = self.initial_sound {
            out.push(c.into());
        }
        if let Some(j) = self.medial_sound {
            out.push(j.into());
        }
        if let Some(jo) = self.final_sound {
            out.push(jo.into());
        }
    }

    pub fn commit(&mut self, out: &mut String) {
        self.preedit(out);
        self.reset();
    }

    pub fn backspace(&mut self, opts: InputOptions) -> bool {
        if self.history.is_empty() {
            return false;
        }
        self.history.pop();

        let old_history = std::mem::take(&mut self.history);
        self.reset();
        for kv in old_history {
            let _ = self.key(kv, opts);
        }
        true
    }

    fn apply_kv(&mut self, kv: KeyValue, opts: InputOptions) -> CharacterResult {
        match kv {
            KeyValue::Initial { initial_sound } => self.initial_sound(initial_sound, opts),
            KeyValue::Medial {
                medial_sound,
                compose,
            } => self.medial_sound(medial_sound, compose, opts),
            KeyValue::Final { final_sound } => self.final_sound(final_sound, Some(kv), opts),
            KeyValue::Both {
                initial_sound,
                final_sound,
            } => {
                if self.initial_sound.is_some()
                    && self.medial_sound.is_some()
                    && self.final_sound.is_none()
                {
                    self.final_sound(final_sound, Some(kv), opts)
                } else {
                    self.initial_sound(initial_sound, opts)
                }
            }
            KeyValue::ChoJong {
                initial_sound,
                final_sound,
                first,
            } => self.cho_jong(initial_sound, final_sound, first, opts),
            KeyValue::ChoJung {
                initial_sound,
                medial_sound,
                first,
                compose,
            } => self.cho_jung(initial_sound, medial_sound, first, compose, opts),
            KeyValue::JungJong {
                medial_sound,
                final_sound,
                first,
                compose,
            } => self.jung_jong(medial_sound, final_sound, first, compose, opts),
            KeyValue::Pass(_) => CharacterResult::Consume,
        }
    }

    pub fn key(&mut self, kv: KeyValue, opts: InputOptions) -> CharacterResult {
        let ret = self.apply_kv(kv, opts);
        match ret {
            CharacterResult::Consume => {
                self.history.push(kv);
                CharacterResult::Consume
            }
            CharacterResult::NewCharacter(mut next) => {
                next.history.push(kv);
                CharacterResult::NewCharacter(next)
            }
        }
    }

    pub fn initial_sound(&mut self, initial_sound: Initial, opts: InputOptions) -> CharacterResult {
        if let Some(prev_cho) = self.initial_sound {
            if let Some(prev_jong) = self.final_sound {
                if let Some(j) = initial_to_final(initial_sound) {
                    if let Some(new_jong) = prev_jong.try_add(j, opts) {
                        self.final_sound = Some(new_jong);
                        self.final_kv = None;
                        return CharacterResult::Consume;
                    }
                }
                CharacterResult::NewCharacter(Self {
                    initial_sound: Some(initial_sound),
                    ..Default::default()
                })
            } else if self.medial_sound.is_some() {
                if let Some(j) = initial_to_final(initial_sound) {
                    self.final_sound = Some(j);
                    self.final_kv = Some(KeyValue::Initial { initial_sound });
                    CharacterResult::Consume
                } else {
                    CharacterResult::NewCharacter(Self {
                        initial_sound: Some(initial_sound),
                        ..Default::default()
                    })
                }
            } else {
                if let Some(new_cho) = prev_cho.try_add(initial_sound, opts) {
                    self.initial_sound = Some(new_cho);
                    return CharacterResult::Consume;
                }
                if opts.non_initial_combi {
                    if let (Some(j1), Some(j2)) =
                        (initial_to_final(prev_cho), initial_to_final(initial_sound))
                    {
                        if let Some(new_jong) = j1.try_add(j2, opts) {
                            self.initial_sound = None;
                            self.final_sound = Some(new_jong);
                            self.final_kv = None;
                            return CharacterResult::Consume;
                        }
                    }
                }
                CharacterResult::NewCharacter(Self {
                    initial_sound: Some(initial_sound),
                    ..Default::default()
                })
            }
        } else if self.medial_sound.is_none() && self.final_sound.is_some() {
            if let Some(j) = initial_to_final(initial_sound) {
                if let Some(new_jong) = self.final_sound.unwrap().try_add(j, opts) {
                    self.final_sound = Some(new_jong);
                    self.final_kv = None;
                    return CharacterResult::Consume;
                }
            }
            CharacterResult::NewCharacter(Self {
                initial_sound: Some(initial_sound),
                ..Default::default()
            })
        } else if opts.auto_reorder || (self.medial_sound.is_none() && self.final_sound.is_none()) {
            self.initial_sound = Some(initial_sound);
            CharacterResult::Consume
        } else {
            CharacterResult::NewCharacter(Self {
                initial_sound: Some(initial_sound),
                ..Default::default()
            })
        }
    }

    pub fn medial_sound(
        &mut self,
        medial_sound: Medial,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if let Some(j) = self.final_sound {
            if opts.treat_final_as_initial {
                let (new_jong, next_cho) = self.calculate_jongseong_move(j, opts);
                let next_history = vec![KeyValue::Initial {
                    initial_sound: next_cho,
                }];
                let next = Self {
                    initial_sound: Some(next_cho),
                    medial_sound: Some(medial_sound),
                    compose_jung: compose,
                    history: next_history,
                    ..Default::default()
                };
                self.final_sound = new_jong;
                self.final_kv = None;
                return CharacterResult::NewCharacter(next);
            }
            CharacterResult::NewCharacter(Self {
                medial_sound: Some(medial_sound),
                compose_jung: compose,
                ..Default::default()
            })
        } else if let Some(prev) = self.medial_sound {
            if let Some(new) =
                Self::try_add_jungseong(prev, self.compose_jung, medial_sound, compose, opts)
            {
                self.medial_sound = Some(new);
                self.compose_jung = true;
                CharacterResult::Consume
            } else {
                CharacterResult::NewCharacter(Self {
                    medial_sound: Some(medial_sound),
                    compose_jung: compose,
                    ..Default::default()
                })
            }
        } else if self.initial_sound.is_none() && !opts.auto_reorder {
            CharacterResult::NewCharacter(Self {
                medial_sound: Some(medial_sound),
                compose_jung: compose,
                ..Default::default()
            })
        } else {
            self.medial_sound = Some(medial_sound);
            self.compose_jung = compose;
            CharacterResult::Consume
        }
    }

    fn calculate_jongseong_move(&self, j: Final, opts: InputOptions) -> (Option<Final>, Initial) {
        if let Some(KeyValue::Both { initial_sound, .. }) = self.final_kv {
            return (None, initial_sound);
        }
        if let Some(KeyValue::Initial { initial_sound }) = self.final_kv {
            return (None, initial_sound);
        }

        match j.to_initial() {
            FinalToInitial::Direct(c) => {
                if c == Initial::이응 {
                    return (Some(j), Initial::이응);
                }
                if opts.combi_on_double_stroke {
                    let is_ssang = matches!(c, Initial::된기윽 | Initial::된시읏);
                    if is_ssang && !self.is_last_kv_ssang() {
                        match c {
                            Initial::된기윽 => return (Some(Final::기윽), Initial::기윽),
                            Initial::된시읏 => return (Some(Final::시읏), Initial::시읏),
                            _ => {}
                        }
                    }
                }
                (None, c)
            }
            FinalToInitial::Compose(jo, c) => (Some(jo), c),
        }
    }

    fn is_last_kv_ssang(&self) -> bool {
        self.history.last().is_some_and(|kv| match kv {
            KeyValue::Both { initial_sound, .. } => matches!(
                initial_sound,
                Initial::된기윽
                    | Initial::된시읏
                    | Initial::된디읃
                    | Initial::된비읍
                    | Initial::된지읒
            ),
            KeyValue::Initial { initial_sound } => matches!(
                initial_sound,
                Initial::된기윽
                    | Initial::된시읏
                    | Initial::된디읃
                    | Initial::된비읍
                    | Initial::된지읒
            ),
            KeyValue::Final { final_sound } => {
                matches!(final_sound, Final::된기윽 | Final::된시읏)
            }
            _ => false,
        })
    }

    pub fn final_sound(
        &mut self,
        final_sound: Final,
        kv: Option<KeyValue>,
        opts: InputOptions,
    ) -> CharacterResult {
        if let Some(prev) = self.final_sound {
            if let Some(new) = prev.try_add(final_sound, opts) {
                self.final_sound = Some(new);
                self.final_kv = None;
                CharacterResult::Consume
            } else {
                CharacterResult::NewCharacter(Self {
                    final_sound: Some(final_sound),
                    final_kv: kv,
                    ..Default::default()
                })
            }
        } else {
            self.final_sound = Some(final_sound);
            self.final_kv = kv;
            CharacterResult::Consume
        }
    }

    pub fn cho_jong(
        &mut self,
        initial_sound: Initial,
        final_sound: Final,
        _first: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        let ret = self.initial_sound(initial_sound, opts);
        match ret {
            CharacterResult::Consume => self.final_sound(
                final_sound,
                Some(KeyValue::ChoJong {
                    initial_sound,
                    final_sound,
                    first: _first,
                }),
                opts,
            ),
            CharacterResult::NewCharacter(mut next) => {
                next.final_sound(
                    final_sound,
                    Some(KeyValue::ChoJong {
                        initial_sound,
                        final_sound,
                        first: _first,
                    }),
                    opts,
                );
                CharacterResult::NewCharacter(next)
            }
        }
    }

    pub fn cho_jung(
        &mut self,
        initial_sound: Initial,
        medial_sound: Medial,
        first: bool,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if first {
            let ret = self.initial_sound(initial_sound, opts);
            match ret {
                CharacterResult::Consume => self.medial_sound(medial_sound, compose, opts),
                CharacterResult::NewCharacter(mut next) => {
                    next.medial_sound(medial_sound, compose, opts);
                    CharacterResult::NewCharacter(next)
                }
            }
        } else {
            self.medial_sound(medial_sound, compose, opts)
        }
    }

    pub fn jung_jong(
        &mut self,
        medial_sound: Medial,
        final_sound: Final,
        first: bool,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if first {
            let ret = self.medial_sound(medial_sound, compose, opts);
            match ret {
                CharacterResult::Consume => self.final_sound(
                    final_sound,
                    Some(KeyValue::JungJong {
                        medial_sound,
                        final_sound,
                        first,
                        compose,
                    }),
                    opts,
                ),
                CharacterResult::NewCharacter(mut next) => {
                    next.final_sound(
                        final_sound,
                        Some(KeyValue::JungJong {
                            medial_sound,
                            final_sound,
                            first,
                            compose,
                        }),
                        opts,
                    );
                    CharacterResult::NewCharacter(next)
                }
            }
        } else {
            self.final_sound(
                final_sound,
                Some(KeyValue::JungJong {
                    medial_sound,
                    final_sound,
                    first,
                    compose,
                }),
                opts,
            )
        }
    }

    fn try_add_jungseong(
        ori_jung: Medial,
        ori_compose: bool,
        medial_sound: Medial,
        compose: bool,
        opts: InputOptions,
    ) -> Option<Medial> {
        if !opts.medial_combi {
            return None;
        }
        if opts.auto_reorder {
            if ori_compose || compose {
                ori_jung
                    .try_add(medial_sound, opts)
                    .or_else(|| medial_sound.try_add(ori_jung, opts))
            } else {
                None
            }
        } else if ori_compose {
            ori_jung.try_add(medial_sound, opts)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asymmetric_both() {
        let mut state = CharacterState::new();
        let opts = InputOptions::default();

        let k = KeyValue::Both {
            initial_sound: Initial::키읔,
            final_sound: Final::기윽,
        };
        let a = KeyValue::Medial {
            medial_sound: Medial::아,
            compose: true,
        };

        state.key(k, opts);
        let mut out = String::new();
        state.preedit(&mut out);
        assert_eq!(out, "ㅋ");

        state.key(a, opts);
        let mut out = String::new();
        state.preedit(&mut out);
        assert_eq!(out, "카");

        state.key(k, opts);
        let mut out = String::new();
        state.preedit(&mut out);
        assert_eq!(out, "칵");

        let mut opts_move = opts;
        opts_move.treat_final_as_initial = true;

        let mut state = CharacterState::new();
        state.key(k, opts_move);
        state.key(a, opts_move);
        state.key(k, opts_move);
        let ret = state.key(a, opts_move);

        match ret {
            CharacterResult::NewCharacter(next) => {
                let mut out = String::new();
                state.preedit(&mut out);
                assert_eq!(out, "카");

                let mut out_next = String::new();
                next.preedit(&mut out_next);
                assert_eq!(out_next, "카");
            }
            _ => panic!("Should move"),
        }
    }
}
