use crate::char_utils::첫소리를_호환첫소리로_변환;
use crate::engine::{
    InputOptions, KeyValue, 가운데소리, 끝소리, 끝소리To첫소리, 첫소리, 첫소리_끝소리_변환,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterResult {
    Consume,
    NewCharacter(CharacterState),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct CharacterState {
    첫소리: Option<첫소리>,
    가운데소리: Option<가운데소리>,
    compose_jung: bool,
    끝소리: Option<끝소리>,
    final_kv: Option<KeyValue>,
    history: Vec<KeyValue>,
}

impl CharacterState {
    pub const fn new() -> Self {
        Self {
            첫소리: None,
            가운데소리: None,
            compose_jung: false,
            끝소리: None,
            final_kv: None,
            history: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.첫소리 = None;
        self.가운데소리 = None;
        self.끝소리 = None;
        self.final_kv = None;
        self.compose_jung = false;
        self.history.clear();
    }

    pub const fn need_display(&self) -> bool {
        self.첫소리.is_some() || self.가운데소리.is_some() || self.끝소리.is_some()
    }

    pub const fn has_initial(&self) -> bool {
        self.첫소리.is_some()
    }
    pub const fn has_medial(&self) -> bool {
        self.가운데소리.is_some()
    }
    pub const fn has_final(&self) -> bool {
        self.끝소리.is_some()
    }

    pub fn history(&self) -> &[KeyValue] {
        &self.history
    }

    pub fn preedit(&self, out: &mut String) {
        match (self.첫소리, self.가운데소리, self.끝소리) {
            (Some(c), Some(j), jo) => out.push_str(&c.compose(j, jo)),
            (Some(c), None, None) => out.push(첫소리를_호환첫소리로_변환(c.into())),
            (None, Some(j), None) => out.push(첫소리를_호환첫소리로_변환(j.into())),
            (None, None, Some(jo)) => out.push(첫소리를_호환첫소리로_변환(jo.into())),
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
        if let Some(c) = self.첫소리 {
            out.push(c.into());
        }
        if let Some(j) = self.가운데소리 {
            out.push(j.into());
        }
        if let Some(jo) = self.끝소리 {
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
            KeyValue::첫소리 { 첫소리 } => self.첫소리(첫소리, opts),
            KeyValue::가운데소리 {
                가운데소리,
                compose,
            } => self.가운데소리(가운데소리, compose, opts),
            KeyValue::끝소리 { 끝소리 } => self.끝소리(끝소리, Some(kv), opts),
            KeyValue::Both {
                첫소리, 끝소리
            } => {
                if self.첫소리.is_some() && self.가운데소리.is_some() && self.끝소리.is_none()
                {
                    self.끝소리(끝소리, Some(kv), opts)
                } else {
                    self.첫소리(첫소리, opts)
                }
            }
            KeyValue::ChoJong {
                첫소리,
                끝소리,
                first,
            } => self.cho_jong(첫소리, 끝소리, first, opts),
            KeyValue::ChoJung {
                첫소리,
                가운데소리,
                first,
                compose,
            } => self.cho_jung(첫소리, 가운데소리, first, compose, opts),
            KeyValue::JungJong {
                가운데소리,
                끝소리,
                first,
                compose,
            } => self.jung_jong(가운데소리, 끝소리, first, compose, opts),
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

    pub fn 첫소리(&mut self, 첫소리: 첫소리, opts: InputOptions) -> CharacterResult {
        if let Some(prev_cho) = self.첫소리 {
            if let Some(prev_jong) = self.끝소리 {
                if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                    if let Some(new_jong) = prev_jong.try_add(j, opts) {
                        self.끝소리 = Some(new_jong);
                        self.final_kv = None;
                        return CharacterResult::Consume;
                    }
                }
                CharacterResult::NewCharacter(Self {
                    첫소리: Some(첫소리),
                    ..Default::default()
                })
            } else if self.가운데소리.is_some() {
                if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                    self.끝소리 = Some(j);
                    self.final_kv = Some(KeyValue::첫소리 { 첫소리 });
                    CharacterResult::Consume
                } else {
                    CharacterResult::NewCharacter(Self {
                        첫소리: Some(첫소리),
                        ..Default::default()
                    })
                }
            } else {
                if let Some(new_cho) = prev_cho.try_add(첫소리, opts) {
                    self.첫소리 = Some(new_cho);
                    return CharacterResult::Consume;
                }
                if opts.non_initial_combi {
                    if let (Some(j1), Some(j2)) =
                        (첫소리_끝소리_변환(prev_cho), 첫소리_끝소리_변환(첫소리))
                    {
                        if let Some(new_jong) = j1.try_add(j2, opts) {
                            self.첫소리 = None;
                            self.끝소리 = Some(new_jong);
                            self.final_kv = None;
                            return CharacterResult::Consume;
                        }
                    }
                }
                CharacterResult::NewCharacter(Self {
                    첫소리: Some(첫소리),
                    ..Default::default()
                })
            }
        } else if self.가운데소리.is_none() && self.끝소리.is_some() {
            if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                if let Some(new_jong) = self.끝소리.unwrap().try_add(j, opts) {
                    self.끝소리 = Some(new_jong);
                    self.final_kv = None;
                    return CharacterResult::Consume;
                }
            }
            CharacterResult::NewCharacter(Self {
                첫소리: Some(첫소리),
                ..Default::default()
            })
        } else if opts.auto_reorder || (self.가운데소리.is_none() && self.끝소리.is_none())
        {
            self.첫소리 = Some(첫소리);
            CharacterResult::Consume
        } else {
            CharacterResult::NewCharacter(Self {
                첫소리: Some(첫소리),
                ..Default::default()
            })
        }
    }

    pub fn 가운데소리(
        &mut self,
        가운데소리: 가운데소리,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if let Some(j) = self.끝소리 {
            if opts.treat_final_as_initial {
                let (new_jong, next_cho) = self.calculate_jongseong_move(j, opts);
                let next_history = vec![KeyValue::첫소리 {
                    첫소리: next_cho
                }];
                let next = Self {
                    첫소리: Some(next_cho),
                    가운데소리: Some(가운데소리),
                    compose_jung: compose,
                    history: next_history,
                    ..Default::default()
                };
                self.끝소리 = new_jong;
                self.final_kv = None;
                return CharacterResult::NewCharacter(next);
            }
            CharacterResult::NewCharacter(Self {
                가운데소리: Some(가운데소리),
                compose_jung: compose,
                ..Default::default()
            })
        } else if let Some(prev) = self.가운데소리 {
            if let Some(new) =
                Self::try_add_jungseong(prev, self.compose_jung, 가운데소리, compose, opts)
            {
                self.가운데소리 = Some(new);
                self.compose_jung = true;
                CharacterResult::Consume
            } else {
                CharacterResult::NewCharacter(Self {
                    가운데소리: Some(가운데소리),
                    compose_jung: compose,
                    ..Default::default()
                })
            }
        } else if self.첫소리.is_none() && !opts.auto_reorder {
            CharacterResult::NewCharacter(Self {
                가운데소리: Some(가운데소리),
                compose_jung: compose,
                ..Default::default()
            })
        } else {
            self.가운데소리 = Some(가운데소리);
            self.compose_jung = compose;
            CharacterResult::Consume
        }
    }

    fn calculate_jongseong_move(
        &self,
        j: 끝소리,
        opts: InputOptions,
    ) -> (Option<끝소리>, 첫소리) {
        if let Some(KeyValue::Both { 첫소리, .. }) = self.final_kv {
            return (None, 첫소리);
        }
        if let Some(KeyValue::첫소리 { 첫소리 }) = self.final_kv {
            return (None, 첫소리);
        }

        match j.to_initial() {
            끝소리To첫소리::Direct(c) => {
                if c == 첫소리::이응 {
                    return (Some(j), 첫소리::이응);
                }
                if opts.combi_on_double_stroke {
                    let is_ssang = matches!(c, 첫소리::된기윽 | 첫소리::된시읏);
                    if is_ssang && !self.is_last_kv_ssang() {
                        match c {
                            첫소리::된기윽 => return (Some(끝소리::기윽), 첫소리::기윽),
                            첫소리::된시읏 => return (Some(끝소리::시읏), 첫소리::시읏),
                            _ => {}
                        }
                    }
                }
                (None, c)
            }
            끝소리To첫소리::Compose(jo, c) => (Some(jo), c),
        }
    }

    fn is_last_kv_ssang(&self) -> bool {
        self.history.last().is_some_and(|kv| match kv {
            KeyValue::Both { 첫소리, .. } => matches!(
                첫소리,
                첫소리::된기윽
                    | 첫소리::된시읏
                    | 첫소리::된디읃
                    | 첫소리::된비읍
                    | 첫소리::된지읒
            ),
            KeyValue::첫소리 { 첫소리 } => matches!(
                첫소리,
                첫소리::된기윽
                    | 첫소리::된시읏
                    | 첫소리::된디읃
                    | 첫소리::된비읍
                    | 첫소리::된지읒
            ),
            KeyValue::끝소리 { 끝소리 } => {
                matches!(끝소리, 끝소리::된기윽 | 끝소리::된시읏)
            }
            _ => false,
        })
    }

    pub fn 끝소리(
        &mut self,
        끝소리: 끝소리,
        kv: Option<KeyValue>,
        opts: InputOptions,
    ) -> CharacterResult {
        if let Some(prev) = self.끝소리 {
            if let Some(new) = prev.try_add(끝소리, opts) {
                self.끝소리 = Some(new);
                self.final_kv = None;
                CharacterResult::Consume
            } else {
                CharacterResult::NewCharacter(Self {
                    끝소리: Some(끝소리),
                    final_kv: kv,
                    ..Default::default()
                })
            }
        } else {
            self.끝소리 = Some(끝소리);
            self.final_kv = kv;
            CharacterResult::Consume
        }
    }

    pub fn cho_jong(
        &mut self,
        첫소리: 첫소리,
        끝소리: 끝소리,
        _first: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        let ret = self.첫소리(첫소리, opts);
        match ret {
            CharacterResult::Consume => self.끝소리(
                끝소리,
                Some(KeyValue::ChoJong {
                    첫소리,
                    끝소리,
                    first: _first,
                }),
                opts,
            ),
            CharacterResult::NewCharacter(mut next) => {
                next.끝소리(
                    끝소리,
                    Some(KeyValue::ChoJong {
                        첫소리,
                        끝소리,
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
        첫소리: 첫소리,
        가운데소리: 가운데소리,
        first: bool,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if first {
            let ret = self.첫소리(첫소리, opts);
            match ret {
                CharacterResult::Consume => self.가운데소리(가운데소리, compose, opts),
                CharacterResult::NewCharacter(mut next) => {
                    next.가운데소리(가운데소리, compose, opts);
                    CharacterResult::NewCharacter(next)
                }
            }
        } else {
            self.가운데소리(가운데소리, compose, opts)
        }
    }

    pub fn jung_jong(
        &mut self,
        가운데소리: 가운데소리,
        끝소리: 끝소리,
        first: bool,
        compose: bool,
        opts: InputOptions,
    ) -> CharacterResult {
        if first {
            let ret = self.가운데소리(가운데소리, compose, opts);
            match ret {
                CharacterResult::Consume => self.끝소리(
                    끝소리,
                    Some(KeyValue::JungJong {
                        가운데소리,
                        끝소리,
                        first,
                        compose,
                    }),
                    opts,
                ),
                CharacterResult::NewCharacter(mut next) => {
                    next.끝소리(
                        끝소리,
                        Some(KeyValue::JungJong {
                            가운데소리,
                            끝소리,
                            first,
                            compose,
                        }),
                        opts,
                    );
                    CharacterResult::NewCharacter(next)
                }
            }
        } else {
            self.끝소리(
                끝소리,
                Some(KeyValue::JungJong {
                    가운데소리,
                    끝소리,
                    first,
                    compose,
                }),
                opts,
            )
        }
    }

    fn try_add_jungseong(
        ori_jung: 가운데소리,
        ori_compose: bool,
        가운데소리: 가운데소리,
        compose: bool,
        opts: InputOptions,
    ) -> Option<가운데소리> {
        if !opts.medial_combi {
            return None;
        }
        if opts.auto_reorder {
            if ori_compose || compose {
                ori_jung
                    .try_add(가운데소리, opts)
                    .or_else(|| 가운데소리.try_add(ori_jung, opts))
            } else {
                None
            }
        } else if ori_compose {
            ori_jung.try_add(가운데소리, opts)
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
            첫소리: 첫소리::키읔,
            끝소리: 끝소리::기윽,
        };
        let a = KeyValue::가운데소리 {
            가운데소리: 가운데소리::아,
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
