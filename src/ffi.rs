#![cfg(feature = "c-api")]
#![allow(clippy::missing_safety_doc)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::ptr::NonNull;

use crate::char_utils;
use crate::input_context::{InputContext, InputEvent, InputOption};
use crate::keyboard::KeyboardRegistry;

#[allow(non_camel_case_types)]
pub type ucschar = u32;

pub const KOREAN_OUTPUT_SYLLABLE: c_int = 0;
pub const KOREAN_OUTPUT_JAMO: c_int = 1;

pub const KOREAN_IC_OPTION_AUTO_REORDER: c_int = 0;
pub const KOREAN_IC_OPTION_COMBI_ON_DOUBLE_STROKE: c_int = 1;
pub const KOREAN_IC_OPTION_NON_CHOSEONG_COMBI: c_int = 2;
pub const KOREAN_IC_OPTION_OLD_JAMO: c_int = 3;
pub const KOREAN_IC_OPTION_NOBLE_NAME: c_int = 4;
pub const KOREAN_IC_OPTION_WORD_UNIT_COMMIT: c_int = 5;

pub const KOREAN_INITIAL_FILLER: ucschar = 0x115F;
pub const KOREAN_MEDIAL_FILLER: ucschar = 0x1160;

#[repr(C)]
pub struct KoreanInputContext {
    _private: [u8; 0],
}

#[derive(Default)]
struct FfiState {
    preedit: CString,
    commit: CString,
    flush: CString,
}

struct ManagedContext {
    ic: InputContext,
    ffi: FfiState,
}

#[no_mangle]
pub extern "C" fn korean_is_initial(c: ucschar) -> bool {
    char::from_u32(c).is_some_and(char_utils::첫소리인가)
}

#[no_mangle]
pub extern "C" fn korean_is_medial(c: ucschar) -> bool {
    char::from_u32(c).is_some_and(char_utils::가운데소리인가)
}

#[no_mangle]
pub extern "C" fn korean_is_final(c: ucschar) -> bool {
    char::from_u32(c).is_some_and(char_utils::끝소리인가)
}

#[no_mangle]
pub extern "C" fn korean_is_initial_conjoinable(c: ucschar) -> bool {
    korean_is_initial(c)
}

#[no_mangle]
pub extern "C" fn korean_is_medial_conjoinable(c: ucschar) -> bool {
    korean_is_medial(c)
}

#[no_mangle]
pub extern "C" fn korean_is_final_conjoinable(c: ucschar) -> bool {
    korean_is_final(c)
}

#[no_mangle]
pub extern "C" fn korean_is_initial_sound_conjoinable(c: ucschar) -> bool {
    korean_is_initial(c) || korean_is_medial(c) || korean_is_final(c)
}

#[no_mangle]
pub extern "C" fn korean_is_syllable(c: ucschar) -> bool {
    char::from_u32(c).is_some_and(char_utils::소리마디인가)
}

#[no_mangle]
pub extern "C" fn korean_is_initial_sound(c: ucschar) -> bool {
    korean_is_initial_sound_conjoinable(c) || korean_is_cjamo(c)
}

#[no_mangle]
pub extern "C" fn korean_is_cjamo(c: ucschar) -> bool {
    char::from_u32(c).is_some_and(char_utils::is_cjamo)
}

#[no_mangle]
pub extern "C" fn korean_initial_sound_to_compat_initial(c: ucschar) -> ucschar {
    char::from_u32(c)
        .map(char_utils::첫소리를_호환첫소리로_변환)
        .map_or(c, |ch| ch as u32)
}

#[no_mangle]
pub extern "C" fn korean_initial_sound_to_syllable(
    첫소리: ucschar,
    가운데소리: ucschar,
    끝소리: ucschar,
) -> ucschar {
    let cho_c = char::from_u32(첫소리);
    let jung_c = char::from_u32(가운데소리);
    let jong_c = if 끝소리 == 0 {
        None
    } else {
        char::from_u32(끝소리)
    };

    match (cho_c, jung_c) {
        (Some(c), Some(j)) => {
            if let Some(syl_str) = char_utils::첫소리_소리마디로_변환(c, j, jong_c) {
                if syl_str.chars().count() == 1 {
                    return syl_str.chars().next().unwrap() as ucschar;
                }
            }
            0
        }
        _ => 0,
    }
}

///
/// Pointer arguments may be null. Non-null pointers must point to valid `ucschar` memory.
#[no_mangle]
pub unsafe extern "C" fn korean_syllable_to_initial_sound(
    syl: ucschar,
    첫소리: *mut ucschar,
    가운데소리: *mut ucschar,
    끝소리: *mut ucschar,
) {
    if let Some(c) = char::from_u32(syl) {
        if let Some((c2, j, jo)) = char_utils::소리마디를_첫소리로_변환(c) {
            if !첫소리.is_null() {
                *첫소리 = c2 as u32;
            }
            if !가운데소리.is_null() {
                *가운데소리 = j as u32;
            }
            if !끝소리.is_null() {
                *끝소리 = jo.map_or(0, |j| j as u32);
            }
        }
    }
}
#[no_mangle]
pub extern "C" fn korean_keyboard_list_get_count() -> c_uint {
    KeyboardRegistry::list().count() as c_uint
}

#[no_mangle]
pub extern "C" fn korean_keyboard_list_get_keyboard_id(index: c_uint) -> *const c_char {
    KeyboardRegistry::list()
        .nth(index as usize)
        .map_or(std::ptr::null(), |kb| kb.id_cstr.as_ptr())
}

#[no_mangle]
pub extern "C" fn korean_keyboard_list_get_keyboard_name(index: c_uint) -> *const c_char {
    KeyboardRegistry::list()
        .nth(index as usize)
        .map_or(std::ptr::null(), |kb| kb.name_cstr.as_ptr())
}

///
/// `keyboard` must be null or a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_new(keyboard: *const c_char) -> *mut KoreanInputContext {
    let id = if keyboard.is_null() {
        "2"
    } else {
        let c_str = unsafe { CStr::from_ptr(keyboard) };
        c_str.to_str().unwrap_or("2")
    };
    InputContext::new(id).map_or(std::ptr::null_mut(), |ic| {
        Box::into_raw(Box::new(ManagedContext {
            ic,
            ffi: FfiState::default(),
        }))
        .cast::<KoreanInputContext>()
    })
}

///
/// `hic` must be null or a pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_delete(hic: *mut KoreanInputContext) {
    if !hic.is_null() {
        drop(Box::from_raw(hic.cast::<ManagedContext>()));
    }
}
fn get_ctx(hic: *mut KoreanInputContext) -> Option<&'static mut ManagedContext> {
    NonNull::new(hic).map(|ptr| unsafe { ptr.cast::<ManagedContext>().as_mut() })
}

///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_process(hic: *mut KoreanInputContext, ascii: c_int) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    ctx.ic.clear_commit_string();
    if !(0..=0x7f).contains(&ascii) {
        return false;
    }
    let key = ascii as u8 as char;
    ctx.ic.process(key)
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_backspace(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    !matches!(ctx.ic.backspace(), InputEvent::None)
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
/// The returned pointer is valid until the next call on this context or deletion.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_flush(hic: *mut KoreanInputContext) -> *const c_char {
    let Some(ctx) = get_ctx(hic) else {
        return std::ptr::null();
    };
    let result = ctx.ic.flush();
    ctx.ffi.flush = CString::new(result).unwrap_or_default();
    ctx.ffi.flush.as_ptr()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_reset(hic: *mut KoreanInputContext) {
    let Some(ctx) = get_ctx(hic) else {
        return;
    };
    ctx.ic.reset();
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
/// The returned pointer is valid until the next call on this context or deletion.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_get_preedit_string(
    hic: *mut KoreanInputContext,
) -> *const c_char {
    let Some(ctx) = get_ctx(hic) else {
        return std::ptr::null();
    };
    let result = ctx.ic.preedit_string();
    ctx.ffi.preedit = CString::new(result).unwrap_or_default();
    ctx.ffi.preedit.as_ptr()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
/// The returned pointer is valid until the next call on this context or deletion.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_get_commit_string(
    hic: *mut KoreanInputContext,
) -> *const c_char {
    let Some(ctx) = get_ctx(hic) else {
        return std::ptr::null();
    };
    let result = ctx.ic.get_commit_string();
    ctx.ffi.commit = CString::new(result).unwrap_or_default();
    ctx.ffi.commit.as_ptr()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_set_option(
    hic: *mut KoreanInputContext,
    option: c_int,
    value: bool,
) {
    let Some(ctx) = get_ctx(hic) else {
        return;
    };
    let opt = match option {
        0 => InputOption::AutoReorder,
        1 => InputOption::CombiOnDoubleStroke,
        2 => InputOption::NonChoseongCombi,
        3 => InputOption::OldJamo,
        4 => InputOption::존함,
        5 => InputOption::WordUnitCommit,
        _ => return,
    };
    ctx.ic.set_option(opt, value);
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_get_option(hic: *mut KoreanInputContext, option: c_int) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    let opt = match option {
        0 => InputOption::AutoReorder,
        1 => InputOption::CombiOnDoubleStroke,
        2 => InputOption::NonChoseongCombi,
        3 => InputOption::OldJamo,
        4 => InputOption::존함,
        5 => InputOption::WordUnitCommit,
        _ => return false,
    };
    ctx.ic.get_option(opt)
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
/// `id` must be null or a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_select_keyboard(
    hic: *mut KoreanInputContext,
    id: *const c_char,
) {
    let Some(ctx) = get_ctx(hic) else {
        return;
    };
    if id.is_null() {
        return;
    }
    let c_str = unsafe { CStr::from_ptr(id) };
    let new_id = c_str.to_str().unwrap_or("2");
    if let Ok(new_ic) = InputContext::new(new_id) {
        ctx.ic = new_ic;
    }
}

///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_is_empty(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return true;
    };
    ctx.ic.is_empty()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_has_initial(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    ctx.ic.has_initial()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_has_medial(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    ctx.ic.has_medial()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_has_final(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    ctx.ic.has_final()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_is_transliteration(hic: *mut KoreanInputContext) -> bool {
    let Some(ctx) = get_ctx(hic) else {
        return false;
    };
    ctx.ic.is_transliteration()
}
///
/// `hic` must be null or a valid pointer returned by `korean_ic_new`.
#[no_mangle]
pub unsafe extern "C" fn korean_ic_set_output_mode(hic: *mut KoreanInputContext, mode: c_int) {
    let Some(ctx) = get_ctx(hic) else {
        return;
    };
    if mode == KOREAN_OUTPUT_JAMO {
        ctx.ic
            .set_output_mode(crate::input_context::OutputMode::자모);
    } else {
        ctx.ic
            .set_output_mode(crate::input_context::OutputMode::소리마디);
    }
}
use crate::hanja::{한자사전, 한자};

#[repr(C)]
pub struct 한자방식 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct 한자목록 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct 한자Ffi {
    _private: [u8; 0],
}

struct Managed한자목록 {
    key: CString,
    entries: Vec<Managed한자Ffi>,
}

struct Managed한자Ffi {
    key: CString,
    value: CString,
    comment: CString,
}

///
/// `filename` must be null or a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn hanja_table_load(filename: *const c_char) -> *mut 한자방식 {
    let path = if filename.is_null() {
        let paths = [
            format!("{}/data/hanja/hanja.txt", env!("CARGO_MANIFEST_DIR")),
            "data/hanja/hanja.txt".into(),
            "/usr/share/libkorean/hanja/hanja.txt".into(),
        ];
        paths
            .into_iter()
            .find(|p| std::path::Path::new(p).exists())
            .unwrap_or_default()
    } else {
        let c_str = unsafe { CStr::from_ptr(filename) };
        c_str.to_str().unwrap_or_default().to_string()
    };
    if path.is_empty() {
        return std::ptr::null_mut();
    }

    한자사전::load(&path)
        .map(|dict| Box::into_raw(Box::new(dict)).cast::<한자방식>())
        .unwrap_or(std::ptr::null_mut())
}

///
/// `table` must be null or a pointer returned by `hanja_table_load`.
#[no_mangle]
pub unsafe extern "C" fn hanja_table_delete(table: *mut 한자방식) {
    if !table.is_null() {
        drop(Box::from_raw(table.cast::<한자사전>()));
    }
}
fn make_hanja_list(key: &str, entries: Vec<한자>) -> *mut 한자목록 {
    let managed_key = CString::new(key).unwrap_or_default();
    let managed_entries: Vec<Managed한자Ffi> = entries
        .into_iter()
        .map(|e| Managed한자Ffi {
            key: CString::new(e.key).unwrap_or_default(),
            value: CString::new(e.value).unwrap_or_default(),
            comment: CString::new(e.comment.unwrap_or_default()).unwrap_or_default(),
        })
        .collect();

    let list = Managed한자목록 {
        key: managed_key,
        entries: managed_entries,
    };

    Box::into_raw(Box::new(list)).cast::<한자목록>()
}

///
/// `table` must be null or a pointer returned by `hanja_table_load`.
/// `key` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn hanja_table_match_exact(
    table: *const 한자방식,
    key: *const c_char,
) -> *mut 한자목록 {
    if table.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    let dict = unsafe { &*table.cast::<한자사전>() };
    let c_str = unsafe { CStr::from_ptr(key) };
    let Ok(key_str) = c_str.to_str() else {
        return std::ptr::null_mut();
    };
    dict.match_exact(key_str)
        .map_or(std::ptr::null_mut(), |entries| {
            make_hanja_list(key_str, entries)
        })
}

///
/// `table` must be null or a pointer returned by `hanja_table_load`.
/// `key` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn hanja_table_match_prefix(
    table: *const 한자방식,
    key: *const c_char,
) -> *mut 한자목록 {
    if table.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    let dict = unsafe { &*table.cast::<한자사전>() };
    let c_str = unsafe { CStr::from_ptr(key) };
    let Ok(key_str) = c_str.to_str() else {
        return std::ptr::null_mut();
    };
    let entries = dict.match_prefix(key_str);
    if entries.is_empty() {
        return std::ptr::null_mut();
    }
    make_hanja_list(key_str, entries)
}

///
/// `table` must be null or a pointer returned by `hanja_table_load`.
/// `key` must be a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn hanja_table_match_suffix(
    table: *const 한자방식,
    key: *const c_char,
) -> *mut 한자목록 {
    if table.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    let dict = unsafe { &*table.cast::<한자사전>() };
    let c_str = unsafe { CStr::from_ptr(key) };
    let Ok(key_str) = c_str.to_str() else {
        return std::ptr::null_mut();
    };
    let entries = dict.match_suffix(key_str);
    if entries.is_empty() {
        return std::ptr::null_mut();
    }
    make_hanja_list(key_str, entries)
}

///
/// `list` must be null or a pointer returned by hanja_table_match_*.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_size(list: *const 한자목록) -> c_int {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return 0;
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed.entries.len() as c_int
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// The returned pointer is valid as long as `list` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_key(list: *const 한자목록) -> *const c_char {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed.key.as_ptr()
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// The returned pointer is valid as long as `list` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_nth_key(
    list: *const 한자목록,
    n: c_uint,
) -> *const c_char {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed
        .entries
        .get(n as usize)
        .map_or(std::ptr::null(), |e| e.key.as_ptr())
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// The returned pointer is valid as long as `list` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_nth_value(
    list: *const 한자목록,
    n: c_uint,
) -> *const c_char {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed
        .entries
        .get(n as usize)
        .map_or(std::ptr::null(), |e| e.value.as_ptr())
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// The returned pointer is valid as long as `list` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_nth_comment(
    list: *const 한자목록,
    n: c_uint,
) -> *const c_char {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed
        .entries
        .get(n as usize)
        .map_or(std::ptr::null(), |e| e.comment.as_ptr())
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// The returned pointer is valid as long as `list` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_get_nth(list: *const 한자목록, n: c_uint) -> *const 한자Ffi {
    let Some(ptr) = NonNull::new(list as *mut 한자목록) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자목록>().as_ref() };
    managed
        .entries
        .get(n as usize)
        .map_or(std::ptr::null(), |e| {
            std::ptr::from_ref::<Managed한자Ffi>(e).cast::<한자Ffi>()
        })
}
///
/// `list` must be null or a pointer returned by hanja_table_match_*.
/// After calling this function, `list` and all pointers derived from it become invalid.
#[no_mangle]
pub unsafe extern "C" fn hanja_list_delete(list: *mut 한자목록) {
    if !list.is_null() {
        drop(Box::from_raw(list.cast::<Managed한자목록>()));
    }
}
///
/// `hanja` must be null or a pointer returned by `hanja_list_get_nth`.
/// The returned pointer is valid as long as the parent `한자목록` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_get_key(hanja: *const 한자Ffi) -> *const c_char {
    let Some(ptr) = NonNull::new(hanja as *mut 한자Ffi) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자Ffi>().as_ref() };
    managed.key.as_ptr()
}
///
/// `hanja` must be null or a pointer returned by `hanja_list_get_nth`.
/// The returned pointer is valid as long as the parent `한자목록` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_get_value(hanja: *const 한자Ffi) -> *const c_char {
    let Some(ptr) = NonNull::new(hanja as *mut 한자Ffi) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자Ffi>().as_ref() };
    managed.value.as_ptr()
}
///
/// `hanja` must be null or a pointer returned by `hanja_list_get_nth`.
/// The returned pointer is valid as long as the parent `한자목록` is not deleted.
#[no_mangle]
pub unsafe extern "C" fn hanja_get_comment(hanja: *const 한자Ffi) -> *const c_char {
    let Some(ptr) = NonNull::new(hanja as *mut 한자Ffi) else {
        return std::ptr::null();
    };
    let managed = unsafe { ptr.cast::<Managed한자Ffi>().as_ref() };
    managed.comment.as_ptr()
}
