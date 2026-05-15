#ifndef _LIBKOREAN_KOREAN_H_
#define _LIBKOREAN_KOREAN_H_

#include <inttypes.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t ucschar;
typedef struct _KoreanInputContext KoreanInputContext;

enum { KOREAN_OUTPUT_SYLLABLE = 0, KOREAN_OUTPUT_JAMO = 1 };

enum {
  KOREAN_IC_OPTION_AUTO_REORDER = 0,
  KOREAN_IC_OPTION_COMBI_ON_DOUBLE_STROKE = 1,
  KOREAN_IC_OPTION_NON_CHOSEONG_COMBI = 2,
  KOREAN_IC_OPTION_OLD_JAMO = 3,
  KOREAN_IC_OPTION_NOBLE_NAME = 4,
  KOREAN_IC_OPTION_WORD_UNIT_COMMIT = 5
};

enum { KOREAN_첫소리_FILLER = 0x115F };
enum { KOREAN_가운데소리_FILLER = 0x1160 };

bool korean_첫소리인가(ucschar c);
bool korean_가운데소리인가(ucschar c);
bool korean_끝소리인가(ucschar c);
bool korean_첫소리인가_conjoinable(ucschar c);
bool korean_가운데소리인가_conjoinable(ucschar c);
bool korean_끝소리인가_conjoinable(ucschar c);
bool korean_첫소리인가_sound_conjoinable(ucschar c);
bool korean_소리마디인가(ucschar c);
bool korean_첫소리인가_sound(ucschar c);
bool korean_is_cjamo(ucschar c);
ucschar korean_initial_sound_to_compat_initial(ucschar ch);
ucschar korean_initial_sound_to_syllable(ucschar 첫소리, ucschar 가운데소리,
                                         ucschar 끝소리);
void korean_syllable_to_initial_sound(ucschar syllable, ucschar *첫소리,
                                      ucschar *가운데소리, ucschar *끝소리);

unsigned int korean_keyboard_list_get_count(void);
const char *korean_keyboard_list_get_keyboard_id(unsigned int index);
const char *korean_keyboard_list_get_keyboard_name(unsigned int index);

KoreanInputContext *korean_ic_new(const char *keyboard);
void korean_ic_delete(KoreanInputContext *hic);
bool korean_ic_process(KoreanInputContext *hic, int ascii);
bool korean_ic_backspace(KoreanInputContext *hic);
void korean_ic_reset(KoreanInputContext *hic);
void korean_ic_remove_preedit_prefix(KoreanInputContext *hic,
                                     const char *prefix);
const char *korean_ic_flush(KoreanInputContext *hic);
const char *korean_ic_get_preedit_string(KoreanInputContext *hic);
const char *korean_ic_get_commit_string(KoreanInputContext *hic);
bool korean_ic_is_empty(KoreanInputContext *hic);
bool korean_ic_has_initial(KoreanInputContext *hic);
bool korean_ic_has_medial(KoreanInputContext *hic);
bool korean_ic_has_final(KoreanInputContext *hic);
bool korean_ic_is_transliteration(KoreanInputContext *hic);
void korean_ic_set_option(KoreanInputContext *hic, int option, bool value);
bool korean_ic_get_option(KoreanInputContext *hic, int option);
void korean_ic_set_output_mode(KoreanInputContext *hic, int mode);
void korean_ic_select_keyboard(KoreanInputContext *hic, const char *id);

typedef struct _HanjaTable HanjaTable;
typedef struct _HanjaList HanjaList;
typedef struct _Hanja Hanja;

HanjaTable *hanja_table_load(const char *filename);
void hanja_table_delete(HanjaTable *table);
HanjaList *hanja_table_match_exact(const HanjaTable *table, const char *key);
HanjaList *hanja_table_match_prefix(const HanjaTable *table, const char *key);
HanjaList *hanja_table_match_suffix(const HanjaTable *table, const char *key);
int hanja_list_get_size(const HanjaList *list);
const char *hanja_list_get_key(const HanjaList *list);
const char *hanja_list_get_nth_key(const HanjaList *list, unsigned int n);
const char *hanja_list_get_nth_value(const HanjaList *list, unsigned int n);
const char *hanja_list_get_nth_comment(const HanjaList *list, unsigned int n);
const Hanja *hanja_list_get_nth(const HanjaList *list, unsigned int n);
void hanja_list_delete(HanjaList *list);
const char *hanja_get_key(const Hanja *hanja);
const char *hanja_get_value(const Hanja *hanja);
const char *hanja_get_comment(const Hanja *hanja);

#ifdef __cplusplus
}
#endif

#endif /* _LIBKOREAN_KOREAN_H_ */
