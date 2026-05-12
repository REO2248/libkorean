use crate::engine::{InputOptions, KeyValue};
use crate::engine::layout::Layout;
use crate::engine::state::{CharacterResult, CharacterState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    Commit(String),
    Preedit(String),
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputMode {
    #[default]
    Syllable,
    Jamo,
}

pub struct InputContext {
    state: CharacterState,
    layout: Layout,
    layout_id: String,
    options: InputOptions,
    output_mode: OutputMode,
    commit_string: String,
    input_buffer: String,
    noble_history: String,
}

impl InputContext {
    const SYLLABLE_BREAK_MARKER: char = '🄵';

    pub fn new(layout_id: &str) -> Result<Self, LayoutError> {
        let layout = Layout::new(layout_id)?;
        let mut options = InputOptions::default();

        let is_trans = layout.has_multi_char_keys();

        if is_trans {
            options.treat_final_as_initial = true;
            options.auto_reorder = false;
            options.non_initial_combi = false;
            options.medial_combi = false;
        } else {
            options.non_initial_combi = true;
        }

        if layout.has_old_jamo() {
            options.old_jamo_mode = true;
        }
        Ok(Self {
            state: CharacterState::new(),
            layout,
            layout_id: layout_id.to_string(),
            options,
            output_mode: OutputMode::Syllable,
            commit_string: String::new(),
            input_buffer: String::new(),
            noble_history: String::new(),
        })
    }

    pub fn set_option(&mut self, option: InputOption, value: bool) {
        match option {
            InputOption::AutoReorder => self.options.auto_reorder = value,
            InputOption::CombiOnDoubleStroke => self.options.combi_on_double_stroke = value,
            InputOption::NonChoseongCombi => self.options.non_initial_combi = value,
            InputOption::OldJamo => self.options.old_jamo_mode = value,
            InputOption::NobleName => {
                if self.options.noble_name && !value {
                    self.flush_noble_name();
                }
                self.options.noble_name = value;
            }
        }
    }

    pub const fn get_option(&self, option: InputOption) -> bool {
        match option {
            InputOption::AutoReorder => self.options.auto_reorder,
            InputOption::CombiOnDoubleStroke => self.options.combi_on_double_stroke,
            InputOption::NonChoseongCombi => self.options.non_initial_combi,
            InputOption::OldJamo => self.options.old_jamo_mode,
            InputOption::NobleName => self.options.noble_name,
        }
    }

    pub const fn set_output_mode(&mut self, mode: OutputMode) {
        self.output_mode = mode;
    }

    pub fn process(&mut self, key: char) -> bool {
        self.commit_string.clear();

        if self.is_transliteration() && key.is_ascii_uppercase() {
            self.process_buffer(true);
            self.flush_to_commit();
        }

        self.input_buffer.push(key);
        self.process_buffer(false)
    }

    fn process_buffer(&mut self, force: bool) -> bool {
        while !self.input_buffer.is_empty() {
            let mut match_len = 0;
            let mut matched_kv = None;

            let mut ends: Vec<usize> = self
                .input_buffer
                .char_indices()
                .map(|(i, _)| i)
                .collect();
            ends.push(self.input_buffer.len());

            for end in ends.into_iter().rev() {
                if end == 0 {
                    continue;
                }
                let sub = &self.input_buffer[..end];
                if let Some(kv) = self.layout.lookup(sub, self.is_transliteration()) {
                    matched_kv = Some(kv);
                    match_len = end;
                    break;
                }
            }

            if let Some(kv) = matched_kv {
                if !force && self.layout.is_prefix(&self.input_buffer, self.is_transliteration()) {
                    return true;
                }

                self.process_kv(kv);
                self.input_buffer.drain(..match_len);
            } else {
                if !force && self.layout.is_prefix(&self.input_buffer, self.is_transliteration()) {
                    return true;
                }
                let first_char = self.input_buffer.chars().next().unwrap();
                self.flush_to_commit();
                self.commit_string.push(first_char);
                let len = first_char.len_utf8();
                self.input_buffer.drain(..len);
            }
        }
        true
    }

    fn process_kv(&mut self, mut kv: KeyValue) {
        if self.is_transliteration() {
            let is_consonant = matches!(kv, KeyValue::Initial { .. } | KeyValue::Both { .. } | KeyValue::Final { .. });
            if is_consonant && self.state.has_initial() && !self.state.has_medial() {
                self.state
                    .medial_sound(crate::engine::Medial::Eu, true, self.options);
                self.commit_syllable();
            }

            if let KeyValue::Medial { .. } = kv {
                if !self.state.has_initial()
                    && !self.state.has_medial()
                    && !self.state.has_final()
                {
                    self.state.key(
                        KeyValue::Initial {
                            initial_sound: crate::engine::Initial::Iung,
                        },
                        self.options,
                    );
                }
            }

            if let KeyValue::Final { final_sound } = kv {
                if !self.state.has_initial()
                    && !self.state.has_medial()
                    && !self.state.has_final()
                {
                    if let crate::engine::FinalToInitial::Direct(next_cho) = final_sound.to_initial() {
                        kv = KeyValue::Initial { initial_sound: next_cho };
                    }
                }
            }
        }

        if let KeyValue::Pass(ch) = kv {
            self.flush_to_commit();
            if ch != Self::SYLLABLE_BREAK_MARKER {
                self.commit_string.push(ch);
            }
            return;
        }

        let ret = self.state.key(kv, self.options);
        self.handle_result(ret);
    }

    pub fn backspace(&mut self) -> InputEvent {
        self.commit_string.clear();
        if !self.input_buffer.is_empty() {
            self.input_buffer.pop();
            return InputEvent::Preedit(self.preedit_string());
        }

        let old_state_empty = self.is_empty();
        if self.state.backspace(self.options) || (!old_state_empty && self.is_empty()) {
            InputEvent::Preedit(self.preedit_string())
        } else if !self.noble_history.is_empty() {
            if let Some(last_syl) = self.noble_history.pop() {
                if let Some((cho_c, jung_c, jong_c)) = crate::char_utils::syllable_to_initial_sound(last_syl) {
                    use std::convert::TryFrom;
                    use crate::engine::{Initial, Medial, Final};
                    if let (Ok(cho), Ok(jung)) = (Initial::try_from(cho_c), Medial::try_from(jung_c)) {
                        self.state.initial_sound(cho, self.options);
                        self.state.medial_sound(jung, true, self.options);
                        if let Some(j_c) = jong_c {
                            if let Ok(jo) = Final::try_from(j_c) {
                                self.state.final_sound(jo, None, self.options);
                            }
                        }
                    }
                    InputEvent::Preedit(self.preedit_string())
                } else {
                    InputEvent::Preedit(self.preedit_string())
                }
            } else {
                InputEvent::None
            }
        } else {
            InputEvent::None
        }
    }

    pub fn flush(&mut self) -> String {
        self.process_buffer(true);
        self.commit_string.clear();
        self.flush_to_commit();
        let out = self.commit_string.clone();
        self.commit_string.clear();
        out
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.input_buffer.clear();
        self.commit_string.clear();
        self.noble_history.clear();
    }

    pub const fn is_empty(&self) -> bool {
        !self.state.need_display() && self.noble_history.is_empty()
    }
    pub const fn has_initial(&self) -> bool {
        self.state.has_initial()
    }
    pub const fn has_medial(&self) -> bool {
        self.state.has_medial()
    }
    pub const fn has_final(&self) -> bool {
        self.state.has_final()
    }

    pub fn layout_id(&self) -> &str {
        &self.layout_id
    }

    pub fn is_transliteration(&self) -> bool {
        self.layout.has_multi_char_keys()
    }

    pub fn preedit_string(&self) -> String {
        let mut out = String::new();
        let history = self.noble_history.clone();

        match self.output_mode {
            OutputMode::Syllable => self.state.preedit(&mut out),
            OutputMode::Jamo => self.state.jamo(&mut out),
        }

        let mut combined = history.clone();
        combined.push_str(&out);

        if self.options.noble_name {
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

        let mut final_out = history;
        final_out.push_str(&out);
        let mut out = final_out;

        if !self.input_buffer.is_empty() {
            if let Some(mut kv) = self.layout.lookup(&self.input_buffer, self.is_transliteration()) {
                let mut temp_state = self.state.clone();
                if self.is_transliteration()
                    && !temp_state.has_initial()
                    && !temp_state.has_medial()
                    && !temp_state.has_final()
                {
                    if let KeyValue::Medial { .. } = kv {
                        temp_state.key(
                            KeyValue::Initial {
                                initial_sound: crate::engine::Initial::Iung,
                            },
                            self.options,
                        );
                    } else if let KeyValue::Final { final_sound } = kv {
                        if let crate::engine::FinalToInitial::Direct(next_cho) = final_sound.to_initial() {
                            kv = KeyValue::Initial { initial_sound: next_cho };
                        }
                    }
                }
                match temp_state.key(kv, self.options) {
                    CharacterResult::Consume => {
                        out.clear();
                        match self.output_mode {
                            OutputMode::Syllable => temp_state.preedit(&mut out),
                            OutputMode::Jamo => temp_state.jamo(&mut out),
                        }
                    }
                    CharacterResult::NewCharacter(mut next) => {
                        out.clear();
                        if self.is_transliteration()
                            && !next.has_initial()
                            && next.has_medial()
                        {
                            let mut final_next = CharacterState::new();
                            final_next.key(
                                KeyValue::Initial {
                                    initial_sound: crate::engine::Initial::Iung,
                                },
                                self.options,
                            );
                            for kv in next.history() {
                                final_next.key(*kv, self.options);
                            }
                            next = final_next;
                        }
                        match self.output_mode {
                            OutputMode::Syllable => {
                                temp_state.preedit(&mut out);
                                next.preedit(&mut out);
                            }
                            OutputMode::Jamo => {
                                temp_state.jamo(&mut out);
                                next.jamo(&mut out);
                            }
                        }
                    }
                }
            } else {
                out.push_str(&self.input_buffer);
            }
        }
        out
    }

    pub fn get_commit_string(&self) -> &str {
        &self.commit_string
    }

    pub fn clear_commit_string(&mut self) {
        self.commit_string.clear();
    }

    fn handle_result(&mut self, ret: CharacterResult) {
        match ret {
            CharacterResult::Consume => {}
            CharacterResult::NewCharacter(mut new_state) => {
                if self.is_transliteration()
                    && !new_state.has_initial()
                    && new_state.has_medial()
                {
                    let mut final_new_state = CharacterState::new();
                    final_new_state.key(
                        KeyValue::Initial {
                            initial_sound: crate::engine::Initial::Iung,
                        },
                        self.options,
                    );
                    for kv in new_state.history() {
                        final_new_state.key(*kv, self.options);
                    }
                    new_state = final_new_state;
                }
                self.commit_syllable();
                self.state = new_state;
            }
        }
    }

    fn flush_to_commit(&mut self) {
        match self.output_mode {
            OutputMode::Syllable => {
                self.commit_syllable();
                self.flush_noble_name();
            }
            OutputMode::Jamo => {
                self.state.jamo(&mut self.commit_string);
                self.state.reset();
                self.noble_history.clear();
            }
        }
    }

    fn commit_syllable(&mut self) {
        let mut syl = String::new();
        self.state.commit(&mut syl);
        if syl.is_empty() {
            return;
        }

        if !self.options.noble_name {
            self.commit_string.push_str(&syl);
            return;
        }

        self.noble_history.push_str(&syl);

        let mut replaced = false;
        if self.noble_history.ends_with("김일성") {
            self.noble_history.truncate(self.noble_history.len() - "김일성".len());
            self.flush_noble_name();
            self.commit_string.push_str("\u{F113}\u{F114}\u{F115}");
            replaced = true;
        } else if self.noble_history.ends_with("김정일") {
            self.noble_history.truncate(self.noble_history.len() - "김정일".len());
            self.flush_noble_name();
            self.commit_string.push_str("\u{F116}\u{F117}\u{F118}");
            replaced = true;
        } else if self.noble_history.ends_with("김정은") {
            self.noble_history.truncate(self.noble_history.len() - "김정은".len());
            self.flush_noble_name();
            self.commit_string.push_str("\u{F120}\u{F121}\u{F122}");
            replaced = true;
        }

        if replaced {
            self.noble_history.clear();
        } else {
            let h = &self.noble_history;
            if h != "김" && h != "김일" && h != "김정" {
                self.flush_noble_name();
            }
        }
    }

    fn flush_noble_name(&mut self) {
        self.commit_string.push_str(&self.noble_history);
        self.noble_history.clear();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InputOption {
    AutoReorder,
    CombiOnDoubleStroke,
    NonChoseongCombi,
    OldJamo,
    NobleName,
}

#[derive(Debug)]
pub enum LayoutError {
    Unknown(String),
}

impl std::fmt::Display for LayoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(id) => write!(f, "unknown layout: {id}"),
        }
    }
}

impl std::error::Error for LayoutError {}
