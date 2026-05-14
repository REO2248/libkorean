use super::jamo::{initial_to_final, Final, Initial, InputOptions, Medial};
use num_traits::FromPrimitive;
use std::str::FromStr;
impl Initial {
    pub const FILLER: char = '\u{115F}';

    pub const fn is_old(self) -> bool {
        self as u32 > 18
    }

    #[must_use]
    pub fn compose(self, medial_sound: Medial, final_sound: Option<Final>) -> String {
        let mut s = String::new();
        if (self as u32) < 19
            && (medial_sound as u32) < 21
            && final_sound.is_none_or(|j| (j as u32) < 27)
        {
            s.push(unsafe {
                std::char::from_u32_unchecked(
                    0xAC00
                        + self as u32 * 588
                        + medial_sound as u32 * 28
                        + final_sound.map_or(0, |j| j as u32 + 1),
                )
            });
        } else {
            s.push(self.into());
            s.push(medial_sound.into());
            if let Some(j) = final_sound {
                s.push(j.into());
            }
        }
        s
    }

    pub fn decompose(ch: char) -> Option<(Self, Medial, Option<Final>)> {
        let n = ch as u32;
        let offset = n.checked_sub(0xAC00)?;
        let initial_sound = FromPrimitive::from_u32(offset / 588)?;
        let offset = offset % 588;
        let medial_sound = FromPrimitive::from_u32(offset / 28)?;
        let offset = offset % 28;
        let final_sound = match offset.checked_sub(1) {
            Some(o) => Some(FromPrimitive::from_u32(o)?),
            None => None,
        };
        Some((initial_sound, medial_sound, final_sound))
    }

    pub const fn try_add(self, other: Self, opts: InputOptions) -> Option<Self> {
        if opts.combi_on_double_stroke {
            match (self, other) {
                (Self::기윽, Self::기윽) => return Some(Self::된기윽),
                (Self::비읍, Self::비읍) => return Some(Self::된비읍),
                (Self::시읏, Self::시읏) => return Some(Self::된시읏),
                (Self::지읒, Self::지읒) => return Some(Self::된지읒),
                (Self::디읃, Self::디읃) => return Some(Self::된디읃),
                _ => {}
            }
        }

        if opts.old_jamo_mode {
            if let result @ Some(_) = match (self, other) {
                (Self::기윽, Self::기윽) => Some(Self::된기윽),
                (Self::기윽, Self::디읃) => Some(Self::기윽디읃),
                (Self::니은, Self::기윽) => Some(Self::니은기윽),
                (Self::니은, Self::니은) => Some(Self::두니은),
                (Self::니은, Self::디읃) => Some(Self::니은디읃),
                (Self::니은, Self::비읍) => Some(Self::니은비읍),
                (Self::니은, Self::시읏) => Some(Self::니은시읏),
                (Self::니은, Self::지읒) => Some(Self::니은지읒),
                (Self::니은, Self::히읗) => Some(Self::니은히읗),
                (Self::디읃, Self::기윽) => Some(Self::디읃기윽),
                (Self::디읃, Self::디읃) => Some(Self::된디읃),
                (Self::디읃, Self::리을) => Some(Self::디읃리을),
                (Self::디읃, Self::미음) => Some(Self::디읃미음),
                (Self::디읃, Self::비읍) => Some(Self::디읃비읍),
                (Self::디읃, Self::시읏) => Some(Self::디읃시읏),
                (Self::디읃, Self::지읒) => Some(Self::디읃지읒),
                (Self::리을, Self::기윽) => Some(Self::리을기윽),
                (Self::리을, Self::된기윽) => Some(Self::리을두기윽),
                (Self::리을, Self::니은) => Some(Self::리을니은),
                (Self::리을, Self::디읃) => Some(Self::리을디읃),
                (Self::리을, Self::된디읃) => Some(Self::리을두디읃),
                (Self::리을, Self::리을) => Some(Self::두리을),
                (Self::리을, Self::미음) => Some(Self::리을미음),
                (Self::리을, Self::비읍) => Some(Self::리을비읍),
                (Self::리을, Self::된비읍) => Some(Self::리을두비읍),
                (Self::리을, Self::시읏) => Some(Self::리을시읏),
                (Self::리을, Self::이응) => Some(Self::가벼운리을),
                (Self::리을, Self::지읒) => Some(Self::리을지읒),
                (Self::리을, Self::키읔) => Some(Self::리을키읔),
                (Self::리을, Self::히읗) => Some(Self::리을히읗),
                (Self::리을, Self::가벼운비읍) => Some(Self::리을가벼운비읍),
                (Self::미음, Self::기윽) => Some(Self::미음기윽),
                (Self::미음, Self::디읃) => Some(Self::미음디읃),
                (Self::미음, Self::비읍) => Some(Self::미음비읍),
                (Self::미음, Self::시읏) => Some(Self::미음시읏),
                (Self::미음, Self::이응) => Some(Self::가벼운미음),
                (Self::비읍, Self::기윽) => Some(Self::비읍기윽),
                (Self::비읍, Self::니은) => Some(Self::비읍니은),
                (Self::비읍, Self::디읃) => Some(Self::비읍디읃),
                (Self::비읍, Self::비읍) => Some(Self::된비읍),
                (Self::비읍, Self::시읏) => Some(Self::비읍시읏),
                (Self::비읍, Self::된시읏) => Some(Self::비읍두시읏),
                (Self::비읍, Self::이응) => Some(Self::가벼운비읍),
                (Self::비읍, Self::지읒) => Some(Self::비읍지읒),
                (Self::비읍, Self::치읓) => Some(Self::비읍치읓),
                (Self::비읍, Self::키읔) => Some(Self::비읍키읔),
                (Self::비읍, Self::티읕) => Some(Self::비읍티읕),
                (Self::비읍, Self::피읖) => Some(Self::비읍피읖),
                (Self::비읍, Self::히읗) => Some(Self::비읍히읗),
                (Self::비읍, Self::가벼운비읍) => Some(Self::가벼운두비읍),
                (Self::비읍, Self::시읏기윽) => Some(Self::비읍시읏기윽),
                (Self::비읍, Self::시읏디읃) => Some(Self::비읍시읏디읃),
                (Self::비읍, Self::시읏비읍) => Some(Self::비읍시읏비읍),
                (Self::비읍, Self::시읏지읒) => Some(Self::비읍시읏지읒),
                (Self::비읍, Self::시읏티읕) => Some(Self::비읍시읏티읕),
                (Self::된비읍, Self::이응) => Some(Self::가벼운두비읍),
                (Self::시읏, Self::기윽) => Some(Self::시읏기윽),
                (Self::시읏, Self::니은) => Some(Self::시읏니은),
                (Self::시읏, Self::디읃) => Some(Self::시읏디읃),
                (Self::시읏, Self::리을) => Some(Self::시읏리을),
                (Self::시읏, Self::미음) => Some(Self::시읏미음),
                (Self::시읏, Self::비읍) => Some(Self::시읏비읍),
                (Self::시읏, Self::시읏) => Some(Self::된시읏),
                (Self::시읏, Self::된시읏) => Some(Self::시읏두시읏),
                (Self::시읏, Self::이응) => Some(Self::시읏이응),
                (Self::시읏, Self::지읒) => Some(Self::시읏지읒),
                (Self::시읏, Self::치읓) => Some(Self::시읏치읓),
                (Self::시읏, Self::키읔) => Some(Self::시읏키읔),
                (Self::시읏, Self::티읕) => Some(Self::시읏티읕),
                (Self::시읏, Self::피읖) => Some(Self::시읏피읖),
                (Self::시읏, Self::히읗) => Some(Self::시읏히읗),
                (Self::시읏, Self::비읍기윽) => Some(Self::시읏비읍기윽),
                (Self::시읏, Self::시읏비읍) => Some(Self::두시읏비읍),
                (Self::된시읏, Self::비읍) => Some(Self::두시읏비읍),
                (Self::된시읏, Self::시읏) => Some(Self::시읏두시읏),
                (Self::이응, Self::기윽) => Some(Self::이응기윽),
                (Self::이응, Self::디읃) => Some(Self::이응디읃),
                (Self::이응, Self::리을) => Some(Self::이응리을),
                (Self::이응, Self::미음) => Some(Self::이응미음),
                (Self::이응, Self::비읍) => Some(Self::이응비읍),
                (Self::이응, Self::시읏) => Some(Self::이응시읏),
                (Self::이응, Self::이응) => Some(Self::두이응),
                (Self::이응, Self::지읒) => Some(Self::이응지읒),
                (Self::이응, Self::치읓) => Some(Self::이응치읓),
                (Self::이응, Self::티읕) => Some(Self::이응티읕),
                (Self::이응, Self::피읖) => Some(Self::이응피읖),
                (Self::이응, Self::히읗) => Some(Self::이응히읗),
                (Self::이응, Self::반이소리) => Some(Self::이응반이소리),
                (Self::지읒, Self::이응) => Some(Self::지읒이응),
                (Self::지읒, Self::지읒) => Some(Self::된지읒),
                (Self::된지읒, Self::히읗) => Some(Self::두지읒히읗),
                (Self::치읓, Self::키읔) => Some(Self::치읓키읔),
                (Self::치읓, Self::히읗) => Some(Self::치읓히읗),
                (Self::티읕, Self::티읕) => Some(Self::두티읕),
                (Self::피읖, Self::비읍) => Some(Self::피읖비읍),
                (Self::피읖, Self::이응) => Some(Self::가벼운피읖),
                (Self::피읖, Self::히읗) => Some(Self::피읖히읗),
                (Self::히읗, Self::시읏) => Some(Self::히읗시읏),
                (Self::히읗, Self::히읗) => Some(Self::두히읗),
                (Self::비읍시읏, Self::기윽) => Some(Self::비읍시읏기윽),
                (Self::비읍시읏, Self::디읃) => Some(Self::비읍시읏디읃),
                (Self::비읍시읏, Self::비읍) => Some(Self::비읍시읏비읍),
                (Self::비읍시읏, Self::시읏) => Some(Self::비읍두시읏),
                (Self::비읍시읏, Self::지읒) => Some(Self::비읍시읏지읒),
                (Self::비읍시읏, Self::티읕) => Some(Self::비읍시읏티읕),
                (Self::시읏비읍, Self::기윽) => Some(Self::시읏비읍기윽),
                (Self::이머리소리시읏, Self::이머리소리시읏) => {
                    Some(Self::이머리소리두시읏)
                }
                (Self::이몸소리시읏, Self::이몸소리시읏) => Some(Self::이몸소리두시읏),
                (Self::이머리소리지읒, Self::이머리소리지읒) => {
                    Some(Self::이머리소리두지읒)
                }
                (Self::이몸소리지읒, Self::이몸소리지읒) => Some(Self::이몸소리두지읒),
                (Self::목구멍터집소리, Self::목구멍터집소리) => {
                    Some(Self::두목구멍터집소리)
                }
                (Self::리을기윽, Self::기윽) => Some(Self::리을두기윽),
                (Self::리을디읃, Self::디읃) => Some(Self::리을두디읃),
                (Self::리을비읍, Self::비읍) => Some(Self::리을두비읍),
                (Self::리을비읍, Self::이응) => Some(Self::리을가벼운비읍),
                _ => None,
            } {
                return result;
            }
        }
        match (self, other) {
            (Self::치읓, Self::히읗) => Some(Self::치읓),
            _ => None,
        }
    }
}

impl Medial {
    pub const FILLER: char = '\u{1160}';

    pub const fn is_old(self) -> bool {
        self as u32 > 20
    }

    pub const fn try_add(self, other: Self, opts: InputOptions) -> Option<Self> {
        match (self, other) {
            (Self::아, Self::이) => Some(Self::애),
            (Self::야, Self::이) => Some(Self::얘),
            (Self::어, Self::이) => Some(Self::에),
            (Self::여, Self::이) => Some(Self::예),
            (Self::오, Self::아) => Some(Self::와),
            (Self::오, Self::이) => Some(Self::외),
            (Self::오, Self::애) | (Self::와, Self::이) => Some(Self::왜),
            (Self::우, Self::어) => Some(Self::워),
            (Self::우, Self::이) => Some(Self::위),
            (Self::우, Self::에) | (Self::워, Self::이) => Some(Self::웨),
            (Self::으, Self::이) => Some(Self::의),
            (Self::의, Self::우) if opts.old_jamo_mode => Some(Self::의우),
            (Self::이, Self::아) if opts.old_jamo_mode => Some(Self::이아),
            (Self::이, Self::야) if opts.old_jamo_mode => Some(Self::이야),
            (Self::이, Self::얘) if opts.old_jamo_mode => Some(Self::이얘),
            (Self::이, Self::여) if opts.old_jamo_mode => Some(Self::이여),
            (Self::이, Self::예) if opts.old_jamo_mode => Some(Self::이예),
            (Self::이, Self::오) if opts.old_jamo_mode => Some(Self::이오),
            (Self::이, Self::요) if opts.old_jamo_mode => Some(Self::이요),
            (Self::이, Self::우) if opts.old_jamo_mode => Some(Self::이우),
            (Self::이, Self::유) if opts.old_jamo_mode => Some(Self::이유),
            (Self::이, Self::으) if opts.old_jamo_mode => Some(Self::이으),
            (Self::이, Self::이) if opts.old_jamo_mode => Some(Self::이이),
            (Self::이, Self::이야오) if opts.old_jamo_mode => Some(Self::이야오),
            (Self::이, Self::아래아) if opts.old_jamo_mode => Some(Self::이아래아),
            (Self::오오, Self::이) if opts.old_jamo_mode => Some(Self::오오이),
            (Self::이야, Self::오) if opts.old_jamo_mode => Some(Self::이야오),
            (Self::이오, Self::이) if opts.old_jamo_mode => Some(Self::이오이),
            (Self::아래아, Self::아) if opts.old_jamo_mode => Some(Self::아래아아),
            (Self::아래아, Self::어) if opts.old_jamo_mode => Some(Self::아래아어),
            (Self::아래아, Self::에) if opts.old_jamo_mode => Some(Self::아래아에),
            (Self::아래아, Self::우) if opts.old_jamo_mode => Some(Self::아래아우),
            (Self::아래아, Self::이) if opts.old_jamo_mode => Some(Self::아래아이),
            (Self::아래아, Self::아래아) if opts.old_jamo_mode => Some(Self::두아래아),
            (Self::아래아아, Self::아) if opts.old_jamo_mode => Some(Self::두아래아),

            (Self::아, Self::아) if opts.old_jamo_mode => Some(Self::아래아),
            (Self::아, Self::오) if opts.old_jamo_mode => Some(Self::아오),
            (Self::아, Self::우) if opts.old_jamo_mode => Some(Self::아우),
            (Self::아, Self::으) if opts.old_jamo_mode => Some(Self::아으),

            (Self::야, Self::오) if opts.old_jamo_mode => Some(Self::야오),
            (Self::야, Self::요) if opts.old_jamo_mode => Some(Self::야요),
            (Self::야, Self::우) if opts.old_jamo_mode => Some(Self::야우),

            (Self::어, Self::오) if opts.old_jamo_mode => Some(Self::어오),
            (Self::어, Self::우) if opts.old_jamo_mode => Some(Self::어우),
            (Self::어, Self::으) if opts.old_jamo_mode => Some(Self::어으),

            (Self::여, Self::야) if opts.old_jamo_mode => Some(Self::여야),
            (Self::여, Self::오) if opts.old_jamo_mode => Some(Self::여오),
            (Self::여, Self::우) if opts.old_jamo_mode => Some(Self::여우),

            (Self::오, Self::야) if opts.old_jamo_mode => Some(Self::오야),
            (Self::오, Self::얘) if opts.old_jamo_mode => Some(Self::오얘),
            (Self::오, Self::어) if opts.old_jamo_mode => Some(Self::오어),
            (Self::오, Self::에) if opts.old_jamo_mode => Some(Self::오에),
            (Self::오, Self::여) if opts.old_jamo_mode => Some(Self::오여),
            (Self::오, Self::예) if opts.old_jamo_mode => Some(Self::오예),
            (Self::오, Self::오) if opts.old_jamo_mode => Some(Self::오오),
            (Self::오, Self::우) if opts.old_jamo_mode => Some(Self::오우),

            (Self::요, Self::아) if opts.old_jamo_mode => Some(Self::요아),
            (Self::요, Self::애) if opts.old_jamo_mode => Some(Self::요애),
            (Self::요, Self::야) if opts.old_jamo_mode => Some(Self::요야),
            (Self::요, Self::얘) if opts.old_jamo_mode => Some(Self::요얘),
            (Self::요, Self::어) if opts.old_jamo_mode => Some(Self::요어),
            (Self::요, Self::여) if opts.old_jamo_mode => Some(Self::요여),
            (Self::요, Self::오) if opts.old_jamo_mode => Some(Self::요오),
            (Self::요, Self::이) if opts.old_jamo_mode => Some(Self::요이),
            (Self::우, Self::아) if opts.old_jamo_mode => Some(Self::우아),
            (Self::우, Self::애) if opts.old_jamo_mode => Some(Self::우애),

            (Self::우, Self::여) if opts.old_jamo_mode => Some(Self::우여),
            (Self::우, Self::예) if opts.old_jamo_mode => Some(Self::우예),
            (Self::우, Self::우) if opts.old_jamo_mode => Some(Self::우우),

            (Self::우, Self::어으) if opts.old_jamo_mode => Some(Self::우어으),
            (Self::우, Self::이이) if opts.old_jamo_mode => Some(Self::우이이),
            (Self::워, Self::으) if opts.old_jamo_mode => Some(Self::우어으),

            (Self::위, Self::이) if opts.old_jamo_mode => Some(Self::우이이),
            (Self::유, Self::아) if opts.old_jamo_mode => Some(Self::유아),
            (Self::유, Self::애) if opts.old_jamo_mode => Some(Self::유애),
            (Self::유, Self::어) if opts.old_jamo_mode => Some(Self::유어),
            (Self::유, Self::에) if opts.old_jamo_mode => Some(Self::유에),
            (Self::유, Self::여) if opts.old_jamo_mode => Some(Self::유여),
            (Self::유, Self::예) if opts.old_jamo_mode => Some(Self::유예),
            (Self::유, Self::오) if opts.old_jamo_mode => Some(Self::유오),
            (Self::유, Self::우) if opts.old_jamo_mode => Some(Self::유우),
            (Self::유, Self::이) if opts.old_jamo_mode => Some(Self::유이),
            (Self::으, Self::아) if opts.old_jamo_mode => Some(Self::으아),
            (Self::으, Self::어) if opts.old_jamo_mode => Some(Self::으어),
            (Self::으, Self::에) if opts.old_jamo_mode => Some(Self::으에),
            (Self::으, Self::오) if opts.old_jamo_mode => Some(Self::으오),
            (Self::으, Self::우) if opts.old_jamo_mode => Some(Self::으우),
            (Self::으, Self::으) if opts.old_jamo_mode => Some(Self::으으),

            _ => None,
        }
    }
}
impl Final {
    pub const fn is_old(self) -> bool {
        self as u32 > 26
    }

    pub const fn try_add(self, other: Self, opts: InputOptions) -> Option<Self> {
        let compose_ssang = opts.combi_on_double_stroke;

        match (self, other) {
            (Self::기윽, Self::기윽) if compose_ssang => Some(Self::된기윽),
            (Self::시읏, Self::시읏) if compose_ssang => Some(Self::된시읏),
            (Self::기윽, Self::시읏) => Some(Self::기윽시읏),
            (Self::니은, Self::히읗) => Some(Self::니은히읗),
            (Self::니은, Self::지읒) => Some(Self::니은지읒),
            (Self::리을, Self::기윽) => Some(Self::리을기윽),
            (Self::리을, Self::미음) => Some(Self::리을미음),
            (Self::리을, Self::비읍) => Some(Self::리을비읍),
            (Self::리을, Self::시읏) => Some(Self::리을시읏),
            (Self::리을, Self::티읕) => Some(Self::리을티읕),
            (Self::리을, Self::피읖) => Some(Self::리을피읖),
            (Self::리을, Self::히읗) => Some(Self::리을히읗),
            (Self::비읍, Self::시읏) => Some(Self::비읍시읏),
            _ if opts.old_jamo_mode => match (self, other) {
                (Self::기윽, Self::기윽) => Some(Self::된기윽),
                (Self::기윽, Self::니은) => Some(Self::기윽니은),
                (Self::기윽, Self::리을) => Some(Self::기윽리을),
                (Self::기윽, Self::비읍) => Some(Self::기윽비읍),
                (Self::기윽, Self::시읏) => Some(Self::기윽시읏),
                (Self::기윽, Self::치읓) => Some(Self::기윽치읓),
                (Self::기윽, Self::키읔) => Some(Self::기윽키읔),
                (Self::기윽, Self::히읗) => Some(Self::기윽히읗),
                (Self::기윽, Self::시읏기윽) => Some(Self::기윽시읏기윽),
                (Self::기윽시읏, Self::기윽) => Some(Self::기윽시읏기윽),
                (Self::니은, Self::기윽) => Some(Self::니은기윽),
                (Self::니은, Self::니은) => Some(Self::두니은),
                (Self::니은, Self::디읃) => Some(Self::니은디읃),
                (Self::니은, Self::리을) => Some(Self::니은리을),
                (Self::니은, Self::시읏) => Some(Self::니은시읏),
                (Self::니은, Self::지읒) => Some(Self::니은지읒),
                (Self::니은, Self::치읓) => Some(Self::니은치읓),
                (Self::니은, Self::티읕) => Some(Self::니은티읕),
                (Self::니은, Self::히읗) => Some(Self::니은히읗),
                (Self::니은, Self::반이소리) => Some(Self::니은반이소리),
                (Self::디읃, Self::기윽) => Some(Self::디읃기윽),
                (Self::디읃, Self::디읃) => Some(Self::두디읃),
                (Self::디읃, Self::리을) => Some(Self::디읃리을),
                (Self::디읃, Self::비읍) => Some(Self::디읃비읍),
                (Self::디읃, Self::시읏) => Some(Self::디읃시읏),
                (Self::디읃, Self::지읒) => Some(Self::디읃지읒),
                (Self::디읃, Self::치읓) => Some(Self::디읃치읓),
                (Self::디읃, Self::티읕) => Some(Self::디읃티읕),
                (Self::디읃, Self::시읏기윽) => Some(Self::디읃시읏기윽),
                (Self::디읃, Self::디읃비읍) => Some(Self::두디읃비읍),
                (Self::리을, Self::기윽) => Some(Self::리을기윽),
                (Self::리을, Self::된기윽) => Some(Self::리을두기윽),
                (Self::리을, Self::기윽시읏) => Some(Self::리을기윽시읏),
                (Self::리을, Self::니은) => Some(Self::리을니은),
                (Self::리을, Self::디읃) => Some(Self::리을디읃),
                (Self::리을, Self::리을) => Some(Self::두리을),
                (Self::리을, Self::미음) => Some(Self::리을미음),
                (Self::리을, Self::비읍) => Some(Self::리을비읍),
                (Self::리을, Self::비읍시읏) => Some(Self::리을비읍시읏),
                (Self::리을, Self::시읏) => Some(Self::리을시읏),
                (Self::리을, Self::된시읏) => Some(Self::리을두시읏),
                (Self::리을, Self::이응) => Some(Self::가벼운리을),
                (Self::리을, Self::키읔) => Some(Self::리을키읔),
                (Self::리을, Self::티읕) => Some(Self::리을티읕),
                (Self::리을, Self::피읖) => Some(Self::리을피읖),
                (Self::리을, Self::히읗) => Some(Self::리을히읗),
                (Self::리을, Self::리을키읔) => Some(Self::두리을키읔),
                (Self::리을, Self::미음기윽) => Some(Self::리을미음기윽),
                (Self::리을, Self::미음시읏) => Some(Self::리을미음시읏),
                (Self::리을, Self::미음히읗) => Some(Self::리을미음히읗),
                (Self::리을, Self::비읍피읖) => Some(Self::리을비읍피읖),
                (Self::리을, Self::비읍히읗) => Some(Self::리을비읍히읗),
                (Self::리을, Self::가벼운비읍) => Some(Self::리을가벼운비읍),
                (Self::리을, Self::반이소리) => Some(Self::리을반이소리),
                (Self::리을, Self::어금이소리) => Some(Self::리을어금이소리),
                (Self::리을, Self::목구멍터집소리) => Some(Self::리을목구멍터집소리),
                (Self::리을, Self::기윽히읗) => Some(Self::리을기윽히읗),
                (Self::리을, Self::비읍디읃) => Some(Self::리을비읍디읃),
                (Self::리을기윽, Self::기윽) => Some(Self::리을두기윽),
                (Self::리을기윽, Self::시읏) => Some(Self::리을기윽시읏),
                (Self::리을기윽, Self::히읗) => Some(Self::리을기윽히읗),
                (Self::리을미음, Self::기윽) => Some(Self::리을미음기윽),
                (Self::리을미음, Self::시읏) => Some(Self::리을미음시읏),
                (Self::리을미음, Self::히읗) => Some(Self::리을미음히읗),
                (Self::리을비읍, Self::디읃) => Some(Self::리을비읍디읃),
                (Self::리을비읍, Self::시읏) => Some(Self::리을비읍시읏),
                (Self::리을비읍, Self::이응) => Some(Self::리을가벼운비읍),
                (Self::리을비읍, Self::피읖) => Some(Self::리을비읍피읖),
                (Self::리을비읍, Self::히읗) => Some(Self::리을비읍히읗),
                (Self::리을시읏, Self::시읏) => Some(Self::리을두시읏),
                (Self::미음, Self::기윽) => Some(Self::미음기윽),
                (Self::미음, Self::니은) => Some(Self::미음니은),
                (Self::미음, Self::리을) => Some(Self::미음리을),
                (Self::미음, Self::미음) => Some(Self::두미음),
                (Self::미음, Self::비읍) => Some(Self::미음비읍),
                (Self::미음, Self::비읍시읏) => Some(Self::미음비읍시읏),
                (Self::미음, Self::시읏) => Some(Self::미음시읏),
                (Self::미음, Self::된시읏) => Some(Self::미음두시읏),
                (Self::미음, Self::이응) => Some(Self::가벼운미음),
                (Self::미음, Self::지읒) => Some(Self::미음지읒),
                (Self::미음, Self::치읓) => Some(Self::미음치읓),
                (Self::미음, Self::히읗) => Some(Self::미음히읗),
                (Self::미음, Self::반이소리) => Some(Self::미음반이소리),
                (Self::미음, Self::두니은) => Some(Self::미음두니은),
                (Self::비읍, Self::디읃) => Some(Self::비읍디읃),
                (Self::비읍, Self::리을) => Some(Self::비읍리을),
                (Self::비읍, Self::리을피읖) => Some(Self::비읍리을피읖),
                (Self::비읍, Self::미음) => Some(Self::비읍미음),
                (Self::비읍, Self::비읍) => Some(Self::두비읍),
                (Self::비읍, Self::시읏) => Some(Self::비읍시읏),
                (Self::비읍, Self::이응) => Some(Self::가벼운비읍),
                (Self::비읍, Self::지읒) => Some(Self::비읍지읒),
                (Self::비읍, Self::치읓) => Some(Self::비읍치읓),
                (Self::비읍, Self::피읖) => Some(Self::비읍피읖),
                (Self::비읍, Self::히읗) => Some(Self::비읍히읗),
                (Self::비읍, Self::시읏디읃) => Some(Self::비읍시읏디읃),
                (Self::비읍시읏, Self::디읃) => Some(Self::비읍시읏디읃),
                (Self::시읏, Self::기윽) => Some(Self::시읏기윽),
                (Self::시읏, Self::디읃) => Some(Self::시읏디읃),
                (Self::시읏, Self::리을) => Some(Self::시읏리을),
                (Self::시읏, Self::미음) => Some(Self::시읏미음),
                (Self::시읏, Self::비읍) => Some(Self::시읏비읍),
                (Self::시읏, Self::시읏) => Some(Self::된시읏),
                (Self::시읏, Self::지읒) => Some(Self::시읏지읒),
                (Self::시읏, Self::치읓) => Some(Self::시읏치읓),
                (Self::시읏, Self::티읕) => Some(Self::시읏티읕),
                (Self::시읏, Self::히읗) => Some(Self::시읏히읗),
                (Self::시읏, Self::가벼운비읍) => Some(Self::시읏가벼운비읍),
                (Self::시읏, Self::시읏기윽) => Some(Self::두시읏기윽),
                (Self::시읏, Self::시읏디읃) => Some(Self::두시읏디읃),
                (Self::시읏, Self::반이소리) => Some(Self::시읏반이소리),
                (Self::된시읏, Self::기윽) => Some(Self::두시읏기윽),
                (Self::된시읏, Self::디읃) => Some(Self::두시읏디읃),
                (Self::지읒, Self::비읍) => Some(Self::지읒비읍),
                (Self::지읒, Self::지읒) => Some(Self::두지읒),
                (Self::지읒, Self::두비읍) => Some(Self::지읒두비읍),
                (Self::피읖, Self::비읍) => Some(Self::피읖비읍),
                (Self::피읖, Self::시읏) => Some(Self::피읖시읏),
                (Self::피읖, Self::이응) => Some(Self::가벼운피읖),
                (Self::피읖, Self::티읕) => Some(Self::피읖티읕),
                (Self::히읗, Self::니은) => Some(Self::히읗니은),
                (Self::히읗, Self::리을) => Some(Self::히읗리을),
                (Self::히읗, Self::미음) => Some(Self::히읗미음),
                (Self::히읗, Self::비읍) => Some(Self::히읗비읍),
                (Self::리을디읃, Self::히읗) => Some(Self::리을디읃히읗),
                (Self::두리을, Self::키읔) => Some(Self::두리을키읔),
                (Self::리을목구멍터집소리, Self::히읗) => {
                    Some(Self::리을목구멍터집소리히읗)
                }
                (Self::미음비읍, Self::시읏) => Some(Self::미음비읍시읏),
                (Self::미음시읏, Self::시읏) => Some(Self::미음두시읏),
                (Self::비읍리을, Self::피읖) => Some(Self::비읍리을피읖),
                (Self::시읏비읍, Self::이응) => Some(Self::시읏가벼운비읍),
                (Self::반이소리, Self::비읍) => Some(Self::반이소리비읍),
                (Self::반이소리, Self::가벼운비읍) => Some(Self::반이소리가벼운비읍),
                (Self::이응기윽, Self::기윽) => Some(Self::이응두기윽),
                (Self::어금이소리, Self::기윽) => Some(Self::이응기윽),
                (Self::어금이소리, Self::된기윽) => Some(Self::이응두기윽),
                (Self::어금이소리, Self::미음) => Some(Self::어금이소리미음),
                (Self::어금이소리, Self::시읏) => Some(Self::어금이소리시읏),
                (Self::어금이소리, Self::키읔) => Some(Self::이응키읔),
                (Self::어금이소리, Self::히읗) => Some(Self::어금이소리히읗),
                (Self::어금이소리, Self::반이소리) => Some(Self::어금이소리반이소리),
                (Self::어금이소리, Self::어금이소리) => Some(Self::두이응),
                (Self::두디읃, Self::비읍) => Some(Self::두디읃비읍),
                (Self::디읃시읏, Self::기윽) => Some(Self::디읃시읏기윽),
                (Self::미음니은, Self::니은) => Some(Self::미음두니은),
                (Self::반이소리비읍, Self::이응) => Some(Self::반이소리가벼운비읍),
                (Self::지읒비읍, Self::비읍) => Some(Self::지읒두비읍),
                _ => None,
            },
            _ => None,
        }
    }

    pub const fn to_initial(self) -> FinalToInitial {
        use FinalToInitial::{Compose, Direct};
        match self {
            Self::기윽 => Direct(Initial::기윽),
            Self::된기윽 => Direct(Initial::된기윽),
            Self::기윽시읏 => Compose(Self::기윽, Initial::시읏),
            Self::니은 => Direct(Initial::니은),
            Self::니은지읒 => Compose(Self::니은, Initial::지읒),
            Self::니은히읗 => Compose(Self::니은, Initial::히읗),
            Self::디읃 => Direct(Initial::디읃),
            Self::리을 => Direct(Initial::리을),
            Self::리을기윽 => Compose(Self::리을, Initial::기윽),
            Self::리을미음 => Compose(Self::리을, Initial::미음),
            Self::리을비읍 => Compose(Self::리을, Initial::비읍),
            Self::리을시읏 => Compose(Self::리을, Initial::시읏),
            Self::리을티읕 => Compose(Self::리을, Initial::티읕),
            Self::리을피읖 => Compose(Self::리을, Initial::피읖),
            Self::리을히읗 => Compose(Self::리을, Initial::히읗),
            Self::미음 => Direct(Initial::미음),
            Self::비읍 => Direct(Initial::비읍),
            Self::비읍시읏 => Compose(Self::비읍, Initial::시읏),
            Self::시읏 => Direct(Initial::시읏),
            Self::된시읏 => Direct(Initial::된시읏),
            Self::이응 => Direct(Initial::이응),
            Self::지읒 => Direct(Initial::지읒),
            Self::치읓 => Direct(Initial::치읓),
            Self::키읔 => Direct(Initial::키읔),
            Self::티읕 => Direct(Initial::티읕),
            Self::피읖 => Direct(Initial::피읖),
            Self::히읗 => Direct(Initial::히읗),
            Self::기윽리을 => Compose(Self::기윽, Initial::리을),
            Self::기윽시읏기윽 => Compose(Self::기윽시읏, Initial::기윽),
            Self::니은기윽 => Compose(Self::니은, Initial::기윽),
            Self::니은디읃 => Compose(Self::니은, Initial::디읃),
            Self::니은시읏 => Compose(Self::니은, Initial::시읏),
            Self::니은반이소리 => Compose(Self::니은, Initial::반이소리),
            Self::니은티읕 => Compose(Self::니은, Initial::티읕),
            Self::디읃기윽 => Compose(Self::디읃, Initial::기윽),
            Self::디읃리을 => Compose(Self::디읃, Initial::리을),
            Self::리을기윽시읏 => Compose(Self::리을기윽, Initial::시읏),
            Self::리을니은 => Compose(Self::리을, Initial::니은),
            Self::리을디읃 => Compose(Self::리을, Initial::디읃),
            Self::리을디읃히읗 => Compose(Self::리을디읃, Initial::히읗),
            Self::두리을 => Compose(Self::리을, Initial::리을),
            Self::리을미음기윽 => Compose(Self::리을미음, Initial::기윽),
            Self::리을미음시읏 => Compose(Self::리을미음, Initial::시읏),
            Self::리을비읍시읏 => Compose(Self::리을비읍, Initial::시읏),
            Self::리을비읍히읗 => Compose(Self::리을비읍, Initial::히읗),
            Self::리을가벼운비읍 => Compose(Self::리을비읍, Initial::이응),
            Self::리을두시읏 => Compose(Self::리을시읏, Initial::시읏),
            Self::리을반이소리 => Compose(Self::리을, Initial::반이소리),
            Self::리을키읔 => Compose(Self::리을, Initial::키읔),
            Self::리을목구멍터집소리 => Compose(Self::리을, Initial::목구멍터집소리),
            Self::미음기윽 => Compose(Self::미음, Initial::기윽),
            Self::미음리을 => Compose(Self::미음, Initial::리을),
            Self::미음비읍 => Compose(Self::미음, Initial::비읍),
            Self::미음시읏 => Compose(Self::미음, Initial::시읏),
            Self::미음두시읏 => Compose(Self::미음시읏, Initial::시읏),
            Self::미음반이소리 => Compose(Self::미음, Initial::반이소리),
            Self::미음치읓 => Compose(Self::미음, Initial::치읓),
            Self::미음히읗 => Compose(Self::미음, Initial::히읗),
            Self::가벼운미음 => Compose(Self::미음, Initial::이응),
            Self::비읍리을 => Compose(Self::비읍, Initial::리을),
            Self::비읍피읖 => Compose(Self::비읍, Initial::피읖),
            Self::비읍히읗 => Compose(Self::비읍, Initial::히읗),
            Self::가벼운비읍 => Compose(Self::비읍, Initial::이응),
            Self::시읏기윽 => Compose(Self::시읏, Initial::기윽),
            Self::시읏디읃 => Compose(Self::시읏, Initial::디읃),
            Self::시읏리을 => Compose(Self::시읏, Initial::리을),
            Self::시읏비읍 => Compose(Self::시읏, Initial::비읍),
            Self::반이소리 => Direct(Initial::반이소리),
            Self::이응기윽 => Compose(Self::어금이소리, Initial::기윽),
            Self::이응두기윽 => Compose(Self::어금이소리, Initial::된기윽),
            Self::두이응 => Compose(Self::어금이소리, Initial::어금이소리),
            Self::이응키읔 => Compose(Self::어금이소리, Initial::키읔),
            Self::어금이소리 => Direct(Initial::어금이소리),
            Self::어금이소리시읏 => Compose(Self::어금이소리, Initial::시읏),
            Self::어금이소리반이소리 => Compose(Self::어금이소리, Initial::반이소리),
            Self::피읖비읍 => Compose(Self::피읖, Initial::비읍),
            Self::가벼운피읖 => Compose(Self::피읖, Initial::이응),
            Self::히읗니은 => Compose(Self::히읗, Initial::니은),
            Self::히읗리을 => Compose(Self::히읗, Initial::리을),
            Self::히읗미음 => Compose(Self::히읗, Initial::미음),
            Self::히읗비읍 => Compose(Self::히읗, Initial::비읍),
            Self::목구멍터집소리 => Direct(Initial::목구멍터집소리),
            Self::기윽니은 => Compose(Self::기윽, Initial::니은),
            Self::기윽비읍 => Compose(Self::기윽, Initial::비읍),
            Self::기윽치읓 => Compose(Self::기윽, Initial::치읓),
            Self::기윽키읔 => Compose(Self::기윽, Initial::키읔),
            Self::기윽히읗 => Compose(Self::기윽, Initial::히읗),
            Self::두니은 => Compose(Self::니은, Initial::니은),
            Self::니은리을 => Compose(Self::니은, Initial::리을),
            Self::니은치읓 => Compose(Self::니은, Initial::치읓),
            Self::두디읃 => Compose(Self::디읃, Initial::디읃),
            Self::두디읃비읍 => Compose(Self::두디읃, Initial::비읍),
            Self::디읃비읍 => Compose(Self::디읃, Initial::비읍),
            Self::디읃시읏 => Compose(Self::디읃, Initial::시읏),
            Self::디읃시읏기윽 => Compose(Self::디읃시읏, Initial::기윽),
            Self::디읃지읒 => Compose(Self::디읃, Initial::지읒),
            Self::디읃치읓 => Compose(Self::디읃, Initial::치읓),
            Self::디읃티읕 => Compose(Self::디읃, Initial::티읕),
            Self::리을두기윽 => Compose(Self::리을기윽, Initial::기윽),
            Self::리을기윽히읗 => Compose(Self::리을기윽, Initial::히읗),
            Self::두리을키읔 => Compose(Self::두리을, Initial::키읔),
            Self::리을미음히읗 => Compose(Self::리을미음, Initial::히읗),
            Self::리을비읍디읃 => Compose(Self::리을비읍, Initial::디읃),
            Self::리을비읍피읖 => Compose(Self::리을비읍, Initial::피읖),
            Self::리을어금이소리 => Compose(Self::리을, Initial::어금이소리),
            Self::리을목구멍터집소리히읗 => {
                Compose(Self::리을목구멍터집소리, Initial::히읗)
            }
            Self::가벼운리을 => Compose(Self::리을, Initial::이응),
            Self::미음니은 => Compose(Self::미음, Initial::니은),
            Self::미음두니은 => Compose(Self::미음니은, Initial::니은),
            Self::두미음 => Compose(Self::미음, Initial::미음),
            Self::미음비읍시읏 => Compose(Self::미음비읍, Initial::시읏),
            Self::미음지읒 => Compose(Self::미음, Initial::지읒),
            Self::비읍디읃 => Compose(Self::비읍, Initial::디읃),
            Self::비읍리을피읖 => Compose(Self::비읍리을, Initial::피읖),
            Self::비읍미음 => Compose(Self::비읍, Initial::미음),
            Self::두비읍 => Compose(Self::비읍, Initial::비읍),
            Self::비읍시읏디읃 => Compose(Self::비읍시읏, Initial::디읃),
            Self::비읍지읒 => Compose(Self::비읍, Initial::지읒),
            Self::비읍치읓 => Compose(Self::비읍, Initial::치읓),
            Self::시읏미음 => Compose(Self::시읏, Initial::미음),
            Self::시읏가벼운비읍 => Compose(Self::시읏비읍, Initial::이응),
            Self::두시읏기윽 => Compose(Self::된시읏, Initial::기윽),
            Self::두시읏디읃 => Compose(Self::된시읏, Initial::디읃),
            Self::시읏반이소리 => Compose(Self::시읏, Initial::반이소리),
            Self::시읏지읒 => Compose(Self::시읏, Initial::지읒),
            Self::시읏치읓 => Compose(Self::시읏, Initial::치읓),
            Self::시읏티읕 => Compose(Self::시읏, Initial::티읕),
            Self::시읏히읗 => Compose(Self::시읏, Initial::히읗),
            Self::반이소리비읍 => Compose(Self::반이소리, Initial::비읍),
            Self::반이소리가벼운비읍 => Compose(Self::반이소리비읍, Initial::이응),
            Self::어금이소리미음 => Compose(Self::어금이소리, Initial::미음),
            Self::어금이소리히읗 => Compose(Self::어금이소리, Initial::히읗),
            Self::지읒비읍 => Compose(Self::지읒, Initial::비읍),
            Self::지읒두비읍 => Compose(Self::지읒비읍, Initial::비읍),
            Self::두지읒 => Compose(Self::지읒, Initial::지읒),
            Self::피읖시읏 => Compose(Self::피읖, Initial::시읏),
            Self::피읖티읕 => Compose(Self::피읖, Initial::티읕),
        }
    }
}

pub enum FinalToInitial {
    Direct(Initial),
    Compose(Final, Initial),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyValue {
    Initial {
        initial_sound: Initial,
    },
    Medial {
        medial_sound: Medial,
        compose: bool,
    },
    Final {
        final_sound: Final,
    },
    Both {
        initial_sound: Initial,
        final_sound: Final,
    },
    ChoJong {
        initial_sound: Initial,
        final_sound: Final,
        first: bool,
    },
    ChoJung {
        initial_sound: Initial,
        medial_sound: Medial,
        first: bool,
        compose: bool,
    },
    JungJong {
        medial_sound: Medial,
        final_sound: Final,
        first: bool,
        compose: bool,
    },
    Pass(char),
}

impl KeyValue {
    pub const fn has_old_jamo(&self) -> bool {
        match self {
            Self::Initial { initial_sound } => initial_sound.is_old(),
            Self::Medial { medial_sound, .. } => medial_sound.is_old(),
            Self::Final { final_sound } => final_sound.is_old(),
            Self::Both {
                initial_sound,
                final_sound,
            } => initial_sound.is_old() || final_sound.is_old(),
            Self::ChoJong {
                initial_sound,
                final_sound,
                ..
            } => initial_sound.is_old() || final_sound.is_old(),
            Self::ChoJung {
                initial_sound,
                medial_sound,
                ..
            } => initial_sound.is_old() || medial_sound.is_old(),
            Self::JungJong {
                medial_sound,
                final_sound,
                ..
            } => medial_sound.is_old() || final_sound.is_old(),
            Self::Pass(_) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyValuePart {
    Cho { initial_sound: Initial },
    Jung { medial_sound: Medial, compose: bool },
    Jong { final_sound: Final },
}

impl KeyValuePart {
    fn parse(chars: &mut std::str::Chars) -> Option<Self> {
        use crate::char_utils::compat_to_conjoining;

        let to_initial = |c: char| {
            Initial::from_initial_sound(c)
                .or_else(|| Initial::from_initial_sound(compat_to_conjoining(c)))
        };
        let to_medial = |c: char| {
            Medial::from_initial_sound(c)
                .or_else(|| Medial::from_initial_sound(compat_to_conjoining(c)))
        };
        let to_final = |c: char| {
            Final::from_initial_sound(c)
                .or_else(|| Final::from_initial_sound(compat_to_conjoining(c)))
        };

        match chars.next()? {
            '$' => {
                let next = chars.next()?;
                if let Some(medial_sound) = to_medial(next) {
                    Some(Self::Jung {
                        medial_sound,
                        compose: false,
                    })
                } else {
                    Some(Self::Jong {
                        final_sound: to_final(next)?,
                    })
                }
            }
            c => {
                if let Some(initial_sound) = to_initial(c) {
                    Some(Self::Cho { initial_sound })
                } else {
                    Some(Self::Jung {
                        medial_sound: to_medial(c)?,
                        compose: true,
                    })
                }
            }
        }
    }
}

impl FromStr for KeyValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut next = || KeyValuePart::parse(&mut chars);

        match next() {
            None => Ok(Self::Pass(s.chars().next().ok_or(())?)),
            Some(first) => match first {
                KeyValuePart::Cho { initial_sound } => match next() {
                    Some(KeyValuePart::Jong { final_sound }) => Ok(Self::Both {
                        initial_sound,
                        final_sound,
                    }),
                    Some(KeyValuePart::Jung {
                        medial_sound,
                        compose,
                    }) => Ok(Self::ChoJung {
                        initial_sound,
                        medial_sound,
                        first: true,
                        compose,
                    }),
                    None => {
                        // Check if it's also a valid final_sound
                        initial_to_final(initial_sound).map_or(
                            Ok(Self::Initial { initial_sound }),
                            |final_sound| {
                                Ok(Self::Both {
                                    initial_sound,
                                    final_sound,
                                })
                            },
                        )
                    }
                    _ => Err(()),
                },
                KeyValuePart::Jung {
                    medial_sound,
                    compose,
                } => match next() {
                    Some(KeyValuePart::Cho { initial_sound }) => Ok(Self::ChoJung {
                        initial_sound,
                        medial_sound,
                        first: false,
                        compose,
                    }),
                    Some(KeyValuePart::Jong { final_sound }) => Ok(Self::JungJong {
                        medial_sound,
                        final_sound,
                        first: true,
                        compose,
                    }),
                    None => Ok(Self::Medial {
                        medial_sound,
                        compose,
                    }),
                    _ => Err(()),
                },
                KeyValuePart::Jong { final_sound } => match next() {
                    Some(KeyValuePart::Cho { initial_sound }) => Ok(Self::Both {
                        initial_sound,
                        final_sound,
                    }),
                    Some(KeyValuePart::Jung {
                        medial_sound,
                        compose,
                    }) => Ok(Self::JungJong {
                        medial_sound,
                        final_sound,
                        first: false,
                        compose,
                    }),
                    None => Ok(Self::Final { final_sound }),
                    _ => Err(()),
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compose_decompose() {
        let (initial_sound, medial_sound, final_sound) = Initial::decompose('앙').unwrap();
        assert_eq!(initial_sound, Initial::이응);
        assert_eq!(medial_sound, Medial::아);
        assert_eq!(final_sound, Some(Final::이응));
        assert_eq!(initial_sound.compose(medial_sound, final_sound), "앙");
    }

    #[test]
    fn test_parse_keyvalue() {
        assert_eq!(
            "ㅇ".parse::<KeyValue>().unwrap(),
            KeyValue::Both {
                initial_sound: Initial::이응,
                final_sound: Final::이응
            }
        );
        assert_eq!(
            "ㅏ".parse::<KeyValue>().unwrap(),
            KeyValue::Medial {
                medial_sound: Medial::아,
                compose: true
            }
        );
        assert_eq!(
            "ㅋ$ㄱ".parse::<KeyValue>().unwrap(),
            KeyValue::Both {
                initial_sound: Initial::키읔,
                final_sound: Final::기윽,
            }
        );
    }

    #[test]
    fn test_old_jamo_initial() {
        assert_eq!(char::from(Initial::목구멍터집소리), 'ᅙ');
        assert_eq!(char::from(Initial::니은히읗), 'ᅝ');
        assert_eq!(char::from(Initial::이머리소리시읏), 'ᄼ');
    }

    #[test]
    fn test_old_jamo_medial() {
        // Araea is U+119E (Conjoining Jamo), not U+3152 (Compatibility Jamo)
        assert_eq!(char::from(Medial::아래아), 'ᆞ');
    }

    #[test]
    fn test_old_jamo_final() {
        assert_eq!(char::from(Final::목구멍터집소리), 'ᇹ');
    }

    #[test]
    fn test_old_jamo_mode_combinations() {
        let mut opts = InputOptions {
            old_jamo_mode: false,
            combi_on_double_stroke: true,
            ..Default::default()
        };
        assert!(Initial::기윽.try_add(Initial::기윽, opts).is_some());

        opts.old_jamo_mode = true;
        // Old initial combination: ᄂ + ᄒ → ᅝ
        assert_eq!(
            Initial::니은.try_add(Initial::히읗, opts),
            Some(Initial::니은히읗)
        );
    }
}
