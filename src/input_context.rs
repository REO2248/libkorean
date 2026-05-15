use crate::engine::layout::건반배렬;
use crate::engine::state::{글자결과, 글자상태};
use crate::engine::{건값, 입력설정};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum 입력사건 {
    결속(String),
    편집(String),
    없음,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum 출력방식 {
    #[default]
    소리마디,
    자모,
}

pub struct 입력문맥 {
    상태: 글자상태,
    배렬: 건반배렬,
    배렬식별자: String,
    설정: 입력설정,
    출력방식: 출력방식,
    결속문자렬: String,
    입력완충: String,
    존함기록: String,
    기록: Vec<건값>,
}

impl 입력문맥 {
    const 소리바디분리표식자: char = '🄵';

    pub fn new(배렬식별자: &str) -> Result<Self, 건반배렬에러> {
        let 배렬 = 건반배렬::new(배렬식별자)?;
        let mut 설정 = 입력설정::default();

        let is_trans = 배렬.다중문자건이_있는가();

        if is_trans {
            설정.끝소리를_첫소리로_처리 = true;
            설정.자동재배치 = false;
            설정.첫소리밖조합 = false;
            설정.가운데소리조합 = false;
        } else {
            설정.첫소리밖조합 = true;
        }

        if 배렬.옛글자가_있는가() {
            설정.옛글자방식 = true;
        }
        Ok(Self {
            상태: 글자상태::new(),
            배렬,
            배렬식별자: 배렬식별자.to_string(),
            설정,
            출력방식: 출력방식::소리마디,
            결속문자렬: String::new(),
            입력완충: String::new(),
            존함기록: String::new(),
            기록: Vec::new(),
        })
    }

    pub fn 항목설정(&mut self, option: 입력항목, value: bool) {
        match option {
            입력항목::자동재배치 => self.설정.자동재배치 = value,
            입력항목::두번타건조합 => self.설정.두번치기_조합 = value,
            입력항목::첫소리밖조합 => self.설정.첫소리밖조합 = value,
            입력항목::옛글자방식 => self.설정.옛글자방식 = value,
            입력항목::존함 => {
                if self.설정.존함 && !value {
                    self.존함비우기();
                }
                self.설정.존함 = value;
            }
            입력항목::단어단위확정 => self.설정.단어단위확정 = value,
        }
    }

    pub const fn 항목획득(&self, option: 입력항목) -> bool {
        match option {
            입력항목::자동재배치 => self.설정.자동재배치,
            입력항목::두번타건조합 => self.설정.두번치기_조합,
            입력항목::첫소리밖조합 => self.설정.첫소리밖조합,
            입력항목::옛글자방식 => self.설정.옛글자방식,
            입력항목::존함 => self.설정.존함,
            입력항목::단어단위확정 => self.설정.단어단위확정,
        }
    }

    pub const fn 출력방식_설정(&mut self, mode: 출력방식) {
        self.출력방식 = mode;
    }

    pub fn 처리(&mut self, key: char) -> bool {
        self.결속문자렬.clear();
        self.전사전처리(key);

        self.입력완충.push(key);
        self.완충처리(false)
    }

    fn 전사전처리(&mut self, key: char) {
        if !self.전사방식인가() || !key.is_ascii_uppercase() {
            return;
        }
        self.완충처리(true);
        self.결속으로비우기();
    }

    fn 완충처리(&mut self, force: bool) -> bool {
        while !self.입력완충.is_empty() {
            let mut match_len = 0;
            let mut matched_kv = None;

            let mut ends: Vec<usize> = self.입력완충.char_indices().map(|(i, _)| i).collect();
            ends.push(self.입력완충.len());

            for end in ends.into_iter().rev() {
                if end == 0 {
                    continue;
                }
                let sub = &self.입력완충[..end];
                if let Some(kv) = self.배렬.찾기(sub, self.전사방식인가()) {
                    matched_kv = Some(kv);
                    match_len = end;
                    break;
                }
            }

            if let Some(kv) = matched_kv {
                if !force
                    && self
                        .배렬
                        .앞붙이인가(&self.입력완충, self.전사방식인가())
                {
                    return true;
                }

                self.건값처리(kv);
                self.입력완충.drain(..match_len);
            } else {
                if !force
                    && self
                        .배렬
                        .앞붙이인가(&self.입력완충, self.전사방식인가())
                {
                    return true;
                }
                let first_char = self.입력완충.chars().next().unwrap();
                self.결속으로비우기();
                self.결속문자렬.push(first_char);
                let len = first_char.len_utf8();
                self.입력완충.drain(..len);
            }
        }
        true
    }

    fn 건값처리(&mut self, kv: 건값) {
        self.기록.push(kv);
        self.다시만들기();
    }

    fn 다시만들기(&mut self) {
        let keys = std::mem::take(&mut self.기록);
        self.상태.초기화();
        self.존함기록.clear();
        for mut kv in keys {
            if self.전사방식인가() {
                kv = self.apply_transliteration_rules(kv);
            }
            if let 건값::통과(ch) = kv {
                self.결속으로비우기();
                if ch != Self::소리바디분리표식자 {
                    self.결속문자렬.push(ch);
                }
                continue;
            }
            match self.상태.key(kv, self.설정) {
                글자결과::소모 => self.기록.push(kv),
                글자결과::새글자(mut next) => {
                    if self.전사방식인가() && !next.첫소리있는가() && next.가운데소리있는가() {
                        let mut final_next = 글자상태::new();
                        final_next.key(
                            건값::첫소리 {
                                첫소리: crate::engine::첫소리::이응,
                            },
                            self.설정,
                        );
                        for k in next.기록() {
                            final_next.key(*k, self.설정);
                        }
                        next = final_next;
                    }
                    self.commit_syllable();
                    self.상태 = next;
                    if self.기록.is_empty() {
                        self.기록.extend(self.상태.기록());
                    } else {
                        self.기록.push(kv);
                    }
                }
            }
        }
    }

    fn apply_transliteration_rules(&mut self, mut kv: 건값) -> 건값 {
        let is_consonant = matches!(
            kv,
            건값::첫소리 { .. } | 건값::둘다 { .. } | 건값::끝소리 { .. }
        );
        if is_consonant && self.상태.첫소리있는가() && !self.상태.가운데소리있는가() {
            self.상태.key(
                건값::가운데소리 {
                    가운데소리: crate::engine::가운데소리::으,
                    조합: true,
                },
                self.설정,
            );
            self.commit_syllable();
        }

        if let 건값::가운데소리 { .. } = kv {
            if !self.상태.첫소리있는가() && !self.상태.가운데소리있는가() && !self.상태.끝소리있는가() {
                self.상태.key(
                    건값::첫소리 {
                        첫소리: crate::engine::첫소리::이응,
                    },
                    self.설정,
                );
            }
        }

        if let 건값::끝소리 { 끝소리 } = kv {
            if !self.상태.첫소리있는가() && !self.상태.가운데소리있는가() && !self.상태.끝소리있는가() {
                if let crate::engine::끝소리To첫소리::Direct(next_cho) = 끝소리.첫소리로()
                {
                    kv = 건값::첫소리 {
                        첫소리: next_cho
                    };
                }
            }
        }
        kv
    }

    pub fn 지우기(&mut self) -> 입력사건 {
        self.결속문자렬.clear();
        if !self.입력완충.is_empty() {
            self.입력완충.pop();
            return 입력사건::편집(self.편집문자렬());
        }

        if self.기록.is_empty() {
            return 입력사건::없음;
        }

        self.기록.pop();
        self.다시만들기();

        입력사건::편집(self.편집문자렬())
    }

    pub fn 비우기(&mut self) -> String {
        self.결속문자렬.clear();
        self.완충처리(true);
        self.결속으로비우기();
        let out = self.결속문자렬.clone();
        self.결속문자렬.clear();
        out
    }

    pub fn 초기화(&mut self) {
        self.상태.초기화();
        self.입력완충.clear();
        self.결속문자렬.clear();
        self.기록.clear();
    }

    pub const fn is_empty(&self) -> bool {
        !self.상태.현시필요() && self.존함기록.is_empty()
    }
    pub const fn 첫소리있는가(&self) -> bool {
        self.상태.첫소리있는가()
    }
    pub const fn 가운데소리있는가(&self) -> bool {
        self.상태.가운데소리있는가()
    }
    pub const fn 끝소리있는가(&self) -> bool {
        self.상태.끝소리있는가()
    }

    pub fn 배렬식별자(&self) -> &str {
        &self.배렬식별자
    }

    pub fn 전사방식인가(&self) -> bool {
        self.배렬.다중문자건이_있는가()
    }

    pub fn 편집문자렬(&self) -> String {
        let mut out = String::new();

        match self.출력방식 {
            출력방식::소리마디 => self.상태.preedit(&mut out),
            출력방식::자모 => self.상태.자모(&mut out),
        }

        if !self.입력완충.is_empty() {
            if let Some(mut kv) = self
                .배렬
                .찾기(&self.입력완충, self.전사방식인가())
            {
                let mut temp_state = self.상태.clone();
                if self.전사방식인가()
                    && !temp_state.첫소리있는가()
                    && !temp_state.가운데소리있는가()
                    && !temp_state.끝소리있는가()
                {
                    if let 건값::가운데소리 { .. } = kv {
                        temp_state.key(
                            건값::첫소리 {
                                첫소리: crate::engine::첫소리::이응,
                            },
                            self.설정,
                        );
                    } else if let 건값::끝소리 { 끝소리 } = kv {
                        if let crate::engine::끝소리To첫소리::Direct(next_cho) = 끝소리.첫소리로()
                        {
                            kv = 건값::첫소리 {
                                첫소리: next_cho
                            };
                        }
                    }
                }
                match temp_state.key(kv, self.설정) {
                    글자결과::소모 => {
                        out.clear();
                        match self.출력방식 {
                            출력방식::소리마디 => temp_state.preedit(&mut out),
                            출력방식::자모 => temp_state.자모(&mut out),
                        }
                    }
                    글자결과::새글자(mut next) => {
                        out.clear();
                        if self.전사방식인가() && !next.첫소리있는가() && next.가운데소리있는가() {
                            let mut final_next = 글자상태::new();
                            final_next.key(
                                건값::첫소리 {
                                    첫소리: crate::engine::첫소리::이응,
                                },
                                self.설정,
                            );
                            for k in next.기록() {
                                final_next.key(*k, self.설정);
                            }
                            next = final_next;
                        }
                        match self.출력방식 {
                            출력방식::소리마디 => {
                                temp_state.preedit(&mut out);
                                next.preedit(&mut out);
                            }
                            출력방식::자모 => {
                                temp_state.자모(&mut out);
                                next.자모(&mut out);
                            }
                        }
                    }
                }
            } else {
                out.push_str(&self.입력완충);
            }
        }

        let mut final_out = self.존함기록.clone();

        if self.설정.존함 {
            let mut combined = final_out.clone();
            combined.push_str(&out);
            if combined.contains("김일성") {
                combined = combined.replace("김일성", "\u{F113}\u{F114}\u{F115}");
                return combined;
            } else if combined.contains("김정일") {
                combined = combined.replace("김정일", "\u{F116}\u{F117}\u{F118}");
                return combined;
            } else if combined.contains("김정은") {
                combined = combined.replace("김정은", "\u{F120}\u{F121}\u{F122}");
                return combined;
            }
        }

        final_out.push_str(&out);
        final_out
    }

    pub fn 결속문자렬(&self) -> &str {
        &self.결속문자렬
    }

    pub fn 결속문자렬_비우기(&mut self) {
        self.결속문자렬.clear();
    }

    fn 결속으로비우기(&mut self) {
        match self.출력방식 {
            출력방식::소리마디 => {
                self.commit_syllable();
                self.존함비우기();
            }
            출력방식::자모 => {
                self.상태.자모(&mut self.결속문자렬);
                self.상태.초기화();
                self.존함기록.clear();
            }
        }
    }

    fn commit_syllable(&mut self) {
        let mut syl = String::new();
        self.상태.commit(&mut syl);
        if syl.is_empty() {
            return;
        }

        if !self.설정.존함 && !self.설정.단어단위확정 {
            self.결속문자렬.push_str(&syl);
            self.기록.clear();
            return;
        }

        self.존함기록.push_str(&syl);
        if self.설정.단어단위확정 {
            return;
        }

        let mut replaced = false;
        if self.존함기록.ends_with("김일성") {
            self.존함기록.truncate(self.존함기록.len() - "김일성".len());
            self.존함비우기();
            self.결속문자렬.push_str("\u{F113}\u{F114}\u{F115}");
            replaced = true;
        } else if self.존함기록.ends_with("김정일") {
            self.존함기록.truncate(self.존함기록.len() - "김정일".len());
            self.존함비우기();
            self.결속문자렬.push_str("\u{F116}\u{F117}\u{F118}");
            replaced = true;
        } else if self.존함기록.ends_with("김정은") {
            self.존함기록.truncate(self.존함기록.len() - "김정은".len());
            self.존함비우기();
            self.결속문자렬.push_str("\u{F120}\u{F121}\u{F122}");
            replaced = true;
        }

        if replaced {
            self.존함기록.clear();
        } else {
            let h = &self.존함기록;
            if h != "김" && h != "김일" && h != "김정" {
                self.존함비우기();
            }
        }
    }

    fn 존함비우기(&mut self) {
        self.결속문자렬.push_str(&self.존함기록);
        self.존함기록.clear();
        self.기록.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum 입력항목 {
    자동재배치,
    두번타건조합,
    첫소리밖조합,
    옛글자방식,
    존함,
    단어단위확정,
}

#[derive(Debug)]
pub enum 건반배렬에러 {
    알수없음(String),
}

impl std::fmt::Display for 건반배렬에러 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::알수없음(id) => write!(f, "unknown 배렬: {id}"),
        }
    }
}

impl std::error::Error for 건반배렬에러 {}
