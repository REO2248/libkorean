use crate::char_utils::첫소리를_호환첫소리로_변환;
use crate::engine::{
    입력설정, 건값, 가운데소리, 끝소리, 끝소리To첫소리, 첫소리, 첫소리_끝소리_변환,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum 글자결과 {
    소모,
    새글자(글자상태),
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct 글자상태 {
    첫소리: Option<첫소리>,
    가운데소리: Option<가운데소리>,
    가운데소리조합중: bool,
    끝소리: Option<끝소리>,
    마지막건값: Option<건값>,
    기록: Vec<건값>,
}

impl 글자상태 {
    pub const fn new() -> Self {
        Self {
            첫소리: None,
            가운데소리: None,
            가운데소리조합중: false,
            끝소리: None,
            마지막건값: None,
            기록: Vec::new(),
        }
    }

    pub fn 초기화(&mut self) {
        self.첫소리 = None;
        self.가운데소리 = None;
        self.끝소리 = None;
        self.마지막건값 = None;
        self.가운데소리조합중 = false;
        self.기록.clear();
    }

    pub const fn 현시필요(&self) -> bool {
        self.첫소리.is_some() || self.가운데소리.is_some() || self.끝소리.is_some()
    }

    pub const fn 첫소리있는가(&self) -> bool {
        self.첫소리.is_some()
    }
    pub const fn 가운데소리있는가(&self) -> bool {
        self.가운데소리.is_some()
    }
    pub const fn 끝소리있는가(&self) -> bool {
        self.끝소리.is_some()
    }

    pub fn 기록(&self) -> &[건값] {
        &self.기록
    }

    pub fn preedit(&self, out: &mut String) {
        match (self.첫소리, self.가운데소리, self.끝소리) {
            (Some(c), Some(j), jo) => out.push_str(&c.조합(j, jo)),
            (Some(c), None, None) => out.push(첫소리를_호환첫소리로_변환(c.into())),
            (None, Some(j), None) => out.push(첫소리를_호환첫소리로_변환(j.into())),
            (None, None, Some(jo)) => out.push(첫소리를_호환첫소리로_변환(jo.into())),
            (None, Some(j), Some(jo)) => {
                out.push(j.자모());
                out.push(jo.자모());
            }
            (Some(c), None, Some(jo)) => {
                out.push(c.자모());
                out.push(jo.자모());
            }
            (None, None, None) => {}
        }
    }

    pub fn 자모(&self, out: &mut String) {
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
        self.초기화();
    }

    pub fn backspace(&mut self, opts: 입력설정) -> bool {
        if self.기록.is_empty() {
            return false;
        }
        self.기록.pop();

        let old_기록 = std::mem::take(&mut self.기록);
        self.초기화();
        for kv in old_기록 {
            let _ = self.key(kv, opts);
        }
        true
    }

    fn apply_kv(&mut self, kv: 건값, opts: 입력설정) -> 글자결과 {
        match kv {
            건값::첫소리 { 첫소리 } => self.첫소리(첫소리, opts),
            건값::가운데소리 {
                가운데소리,
                조합,
            } => self.가운데소리(가운데소리, 조합, opts),
            건값::끝소리 { 끝소리 } => self.끝소리(끝소리, Some(kv), opts),
            건값::둘다 {
                첫소리, 끝소리
            } => {
                if self.첫소리.is_some() && self.가운데소리.is_some() && self.끝소리.is_none()
                {
                    self.끝소리(끝소리, Some(kv), opts)
                } else {
                    self.첫소리(첫소리, opts)
                }
            }
            건값::첫소리끝소리 {
                첫소리,
                끝소리,
                첫번째,
            } => self.cho_jong(첫소리, 끝소리, 첫번째, opts),
            건값::첫소리가운데소리 {
                첫소리,
                가운데소리,
                첫번째,
                조합,
            } => self.cho_jung(첫소리, 가운데소리, 첫번째, 조합, opts),
            건값::가운데소리끝소리 {
                가운데소리,
                끝소리,
                첫번째,
                조합,
            } => self.jung_jong(가운데소리, 끝소리, 첫번째, 조합, opts),
            건값::통과(_) => 글자결과::소모,
        }
    }

    pub fn key(&mut self, kv: 건값, opts: 입력설정) -> 글자결과 {
        let ret = self.apply_kv(kv, opts);
        match ret {
            글자결과::소모 => {
                self.기록.push(kv);
                글자결과::소모
            }
            글자결과::새글자(mut next) => {
                next.기록.push(kv);
                글자결과::새글자(next)
            }
        }
    }

    pub fn 첫소리(&mut self, 첫소리: 첫소리, opts: 입력설정) -> 글자결과 {
        if let Some(prev_cho) = self.첫소리 {
            if let Some(prev_jong) = self.끝소리 {
                if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                    if let Some(new_jong) = prev_jong.try_add(j, opts) {
                        self.끝소리 = Some(new_jong);
                        self.마지막건값 = None;
                        return 글자결과::소모;
                    }
                }
                글자결과::새글자(Self {
                    첫소리: Some(첫소리),
                    ..Default::default()
                })
            } else if self.가운데소리.is_some() {
                if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                    self.끝소리 = Some(j);
                    self.마지막건값 = Some(건값::첫소리 { 첫소리 });
                    글자결과::소모
                } else {
                    글자결과::새글자(Self {
                        첫소리: Some(첫소리),
                        ..Default::default()
                    })
                }
            } else {
                if let Some(new_cho) = prev_cho.try_add(첫소리, opts) {
                    self.첫소리 = Some(new_cho);
                    return 글자결과::소모;
                }
                if opts.첫소리밖조합 {
                    if let (Some(j1), Some(j2)) =
                        (첫소리_끝소리_변환(prev_cho), 첫소리_끝소리_변환(첫소리))
                    {
                        if let Some(new_jong) = j1.try_add(j2, opts) {
                            self.첫소리 = None;
                            self.끝소리 = Some(new_jong);
                            self.마지막건값 = None;
                            return 글자결과::소모;
                        }
                    }
                }
                글자결과::새글자(Self {
                    첫소리: Some(첫소리),
                    ..Default::default()
                })
            }
        } else if self.가운데소리.is_none() && self.끝소리.is_some() {
            if let Some(j) = 첫소리_끝소리_변환(첫소리) {
                if let Some(new_jong) = self.끝소리.unwrap().try_add(j, opts) {
                    self.끝소리 = Some(new_jong);
                    self.마지막건값 = None;
                    return 글자결과::소모;
                }
            }
            글자결과::새글자(Self {
                첫소리: Some(첫소리),
                ..Default::default()
            })
        } else if opts.자동재배치 || (self.가운데소리.is_none() && self.끝소리.is_none())
        {
            self.첫소리 = Some(첫소리);
            글자결과::소모
        } else {
            글자결과::새글자(Self {
                첫소리: Some(첫소리),
                ..Default::default()
            })
        }
    }

    pub fn 가운데소리(
        &mut self,
        가운데소리: 가운데소리,
        조합: bool,
        opts: 입력설정,
    ) -> 글자결과 {
        if let Some(j) = self.끝소리 {
            if opts.끝소리를_첫소리로_처리 {
                let (new_jong, next_cho) = self.calculate_jongseong_move(j, opts);
                let next_기록 = vec![건값::첫소리 {
                    첫소리: next_cho
                }];
                let next = Self {
                    첫소리: Some(next_cho),
                    가운데소리: Some(가운데소리),
                    가운데소리조합중: 조합,
                    기록: next_기록,
                    ..Default::default()
                };
                self.끝소리 = new_jong;
                self.마지막건값 = None;
                return 글자결과::새글자(next);
            }
            글자결과::새글자(Self {
                가운데소리: Some(가운데소리),
                가운데소리조합중: 조합,
                ..Default::default()
            })
        } else if let Some(prev) = self.가운데소리 {
            if let Some(new) =
                Self::try_add_jungseong(prev, self.가운데소리조합중, 가운데소리, 조합, opts)
            {
                self.가운데소리 = Some(new);
                self.가운데소리조합중 = true;
                글자결과::소모
            } else {
                글자결과::새글자(Self {
                    가운데소리: Some(가운데소리),
                    가운데소리조합중: 조합,
                    ..Default::default()
                })
            }
        } else if self.첫소리.is_none() && !opts.자동재배치 {
            글자결과::새글자(Self {
                가운데소리: Some(가운데소리),
                가운데소리조합중: 조합,
                ..Default::default()
            })
        } else {
            self.가운데소리 = Some(가운데소리);
            self.가운데소리조합중 = 조합;
            글자결과::소모
        }
    }

    fn calculate_jongseong_move(
        &self,
        j: 끝소리,
        opts: 입력설정,
    ) -> (Option<끝소리>, 첫소리) {
        if let Some(건값::둘다 { 첫소리, .. }) = self.마지막건값 {
            return (None, 첫소리);
        }
        if let Some(건값::첫소리 { 첫소리 }) = self.마지막건값 {
            return (None, 첫소리);
        }

        match j.첫소리로() {
            끝소리To첫소리::Direct(c) => {
                if c == 첫소리::이응 {
                    return (Some(j), 첫소리::이응);
                }
                if opts.두번치기_조합 {
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
        self.기록.last().is_some_and(|kv| match kv {
            건값::둘다 { 첫소리, .. } => matches!(
                첫소리,
                첫소리::된기윽
                    | 첫소리::된시읏
                    | 첫소리::된디읃
                    | 첫소리::된비읍
                    | 첫소리::된지읒
            ),
            건값::첫소리 { 첫소리 } => matches!(
                첫소리,
                첫소리::된기윽
                    | 첫소리::된시읏
                    | 첫소리::된디읃
                    | 첫소리::된비읍
                    | 첫소리::된지읒
            ),
            건값::끝소리 { 끝소리 } => {
                matches!(끝소리, 끝소리::된기윽 | 끝소리::된시읏)
            }
            _ => false,
        })
    }

    pub fn 끝소리(
        &mut self,
        끝소리: 끝소리,
        kv: Option<건값>,
        opts: 입력설정,
    ) -> 글자결과 {
        if let Some(prev) = self.끝소리 {
            if let Some(new) = prev.try_add(끝소리, opts) {
                self.끝소리 = Some(new);
                self.마지막건값 = None;
                글자결과::소모
            } else {
                글자결과::새글자(Self {
                    끝소리: Some(끝소리),
                    마지막건값: kv,
                    ..Default::default()
                })
            }
        } else {
            self.끝소리 = Some(끝소리);
            self.마지막건값 = kv;
            글자결과::소모
        }
    }

    pub fn cho_jong(
        &mut self,
        첫소리: 첫소리,
        끝소리: 끝소리,
        _first: bool,
        opts: 입력설정,
    ) -> 글자결과 {
        let ret = self.첫소리(첫소리, opts);
        match ret {
            글자결과::소모 => self.끝소리(
                끝소리,
                Some(건값::첫소리끝소리 {
                    첫소리,
                    끝소리,
                    첫번째: _first,
                }),
                opts,
            ),
            글자결과::새글자(mut next) => {
                next.끝소리(
                    끝소리,
                    Some(건값::첫소리끝소리 {
                        첫소리,
                        끝소리,
                        첫번째: _first,
                    }),
                    opts,
                );
                글자결과::새글자(next)
            }
        }
    }

    pub fn cho_jung(
        &mut self,
        첫소리: 첫소리,
        가운데소리: 가운데소리,
        첫번째: bool,
        조합: bool,
        opts: 입력설정,
    ) -> 글자결과 {
        if 첫번째 {
            let ret = self.첫소리(첫소리, opts);
            match ret {
                글자결과::소모 => self.가운데소리(가운데소리, 조합, opts),
                글자결과::새글자(mut next) => {
                    next.가운데소리(가운데소리, 조합, opts);
                    글자결과::새글자(next)
                }
            }
        } else {
            self.가운데소리(가운데소리, 조합, opts)
        }
    }

    pub fn jung_jong(
        &mut self,
        가운데소리: 가운데소리,
        끝소리: 끝소리,
        첫번째: bool,
        조합: bool,
        opts: 입력설정,
    ) -> 글자결과 {
        if 첫번째 {
            let ret = self.가운데소리(가운데소리, 조합, opts);
            match ret {
                글자결과::소모 => self.끝소리(
                    끝소리,
                    Some(건값::가운데소리끝소리 {
                        가운데소리,
                        끝소리,
                        첫번째,
                        조합,
                    }),
                    opts,
                ),
                글자결과::새글자(mut next) => {
                    next.끝소리(
                        끝소리,
                        Some(건값::가운데소리끝소리 {
                            가운데소리,
                            끝소리,
                            첫번째,
                            조합,
                        }),
                        opts,
                    );
                    글자결과::새글자(next)
                }
            }
        } else {
            self.끝소리(
                끝소리,
                Some(건값::가운데소리끝소리 {
                    가운데소리,
                    끝소리,
                    첫번째,
                    조합,
                }),
                opts,
            )
        }
    }

    fn try_add_jungseong(
        ori_jung: 가운데소리,
        ori_compose: bool,
        가운데소리: 가운데소리,
        조합: bool,
        opts: 입력설정,
    ) -> Option<가운데소리> {
        if !opts.가운데소리조합 {
            return None;
        }
        if opts.자동재배치 {
            if ori_compose || 조합 {
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
        let mut state = 글자상태::new();
        let opts = 입력설정::default();

        let k = 건값::둘다 {
            첫소리: 첫소리::키읔,
            끝소리: 끝소리::기윽,
        };
        let a = 건값::가운데소리 {
            가운데소리: 가운데소리::아,
            조합: true,
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
        opts_move.끝소리를_첫소리로_처리 = true;

        let mut state = 글자상태::new();
        state.key(k, opts_move);
        state.key(a, opts_move);
        state.key(k, opts_move);
        let ret = state.key(a, opts_move);

        match ret {
            글자결과::새글자(next) => {
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
