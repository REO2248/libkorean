use super::jamo::{
    입력설정, 가운데소리, 끝소리, 첫소리, 첫소리_끝소리_변환
};
use num_traits::FromPrimitive;
use std::str::FromStr;
impl 첫소리 {
    pub const 채움문자: char = '\u{115F}';

    pub const fn 옛글자인가(self) -> bool {
        self as u32 > 18
    }

    #[must_use]
    pub fn 조합(self, 가운데소리: 가운데소리, 끝소리: Option<끝소리>) -> String {
        let mut s = String::new();
        if (self as u32) < 19 && (가운데소리 as u32) < 21 && 끝소리.is_none_or(|j| (j as u32) < 27)
        {
            s.push(unsafe {
                std::char::from_u32_unchecked(
                    0xAC00
                        + self as u32 * 588
                        + 가운데소리 as u32 * 28
                        + 끝소리.map_or(0, |j| j as u32 + 1),
                )
            });
        } else {
            s.push(self.into());
            s.push(가운데소리.into());
            if let Some(j) = 끝소리 {
                s.push(j.into());
            }
        }
        s
    }

    pub fn 분해(ch: char) -> Option<(Self, 가운데소리, Option<끝소리>)> {
        let n = ch as u32;
        let offset = n.checked_sub(0xAC00)?;
        let 첫소리 = FromPrimitive::from_u32(offset / 588)?;
        let offset = offset % 588;
        let 가운데소리 = FromPrimitive::from_u32(offset / 28)?;
        let offset = offset % 28;
        let 끝소리 = match offset.checked_sub(1) {
            Some(o) => Some(FromPrimitive::from_u32(o)?),
            None => None,
        };
        Some((첫소리, 가운데소리, 끝소리))
    }

    pub const fn try_add(self, other: Self, opts: 입력설정) -> Option<Self> {
        if opts.두번치기_조합 {
            match (self, other) {
                (Self::기윽, Self::기윽) => return Some(Self::된기윽),
                (Self::비읍, Self::비읍) => return Some(Self::된비읍),
                (Self::시읏, Self::시읏) => return Some(Self::된시읏),
                (Self::지읒, Self::지읒) => return Some(Self::된지읒),
                (Self::디읃, Self::디읃) => return Some(Self::된디읃),
                _ => {}
            }
        }

        if opts.옛글자방식 {
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

impl 가운데소리 {
    pub const 채움문자: char = '\u{1160}';

    pub const fn 옛글자인가(self) -> bool {
        self as u32 > 20
    }

    pub const fn try_add(self, other: Self, opts: 입력설정) -> Option<Self> {
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
            (Self::의, Self::우) if opts.옛글자방식 => Some(Self::의우),
            (Self::이, Self::아) if opts.옛글자방식 => Some(Self::이아),
            (Self::이, Self::야) if opts.옛글자방식 => Some(Self::이야),
            (Self::이, Self::얘) if opts.옛글자방식 => Some(Self::이얘),
            (Self::이, Self::여) if opts.옛글자방식 => Some(Self::이여),
            (Self::이, Self::예) if opts.옛글자방식 => Some(Self::이예),
            (Self::이, Self::오) if opts.옛글자방식 => Some(Self::이오),
            (Self::이, Self::요) if opts.옛글자방식 => Some(Self::이요),
            (Self::이, Self::우) if opts.옛글자방식 => Some(Self::이우),
            (Self::이, Self::유) if opts.옛글자방식 => Some(Self::이유),
            (Self::이, Self::으) if opts.옛글자방식 => Some(Self::이으),
            (Self::이, Self::이) if opts.옛글자방식 => Some(Self::이이),
            (Self::이, Self::이야오) if opts.옛글자방식 => Some(Self::이야오),
            (Self::이, Self::아래아) if opts.옛글자방식 => Some(Self::이아래아),
            (Self::오오, Self::이) if opts.옛글자방식 => Some(Self::오오이),
            (Self::이야, Self::오) if opts.옛글자방식 => Some(Self::이야오),
            (Self::이오, Self::이) if opts.옛글자방식 => Some(Self::이오이),
            (Self::아래아, Self::아) if opts.옛글자방식 => Some(Self::아래아아),
            (Self::아래아, Self::어) if opts.옛글자방식 => Some(Self::아래아어),
            (Self::아래아, Self::에) if opts.옛글자방식 => Some(Self::아래아에),
            (Self::아래아, Self::우) if opts.옛글자방식 => Some(Self::아래아우),
            (Self::아래아, Self::이) if opts.옛글자방식 => Some(Self::아래아이),
            (Self::아래아, Self::아래아) if opts.옛글자방식 => Some(Self::두아래아),
            (Self::아래아아, Self::아) if opts.옛글자방식 => Some(Self::두아래아),

            (Self::아, Self::아) if opts.옛글자방식 => Some(Self::아래아),
            (Self::아, Self::오) if opts.옛글자방식 => Some(Self::아오),
            (Self::아, Self::우) if opts.옛글자방식 => Some(Self::아우),
            (Self::아, Self::으) if opts.옛글자방식 => Some(Self::아으),

            (Self::야, Self::오) if opts.옛글자방식 => Some(Self::야오),
            (Self::야, Self::요) if opts.옛글자방식 => Some(Self::야요),
            (Self::야, Self::우) if opts.옛글자방식 => Some(Self::야우),

            (Self::어, Self::오) if opts.옛글자방식 => Some(Self::어오),
            (Self::어, Self::우) if opts.옛글자방식 => Some(Self::어우),
            (Self::어, Self::으) if opts.옛글자방식 => Some(Self::어으),

            (Self::여, Self::야) if opts.옛글자방식 => Some(Self::여야),
            (Self::여, Self::오) if opts.옛글자방식 => Some(Self::여오),
            (Self::여, Self::우) if opts.옛글자방식 => Some(Self::여우),

            (Self::오, Self::야) if opts.옛글자방식 => Some(Self::오야),
            (Self::오, Self::얘) if opts.옛글자방식 => Some(Self::오얘),
            (Self::오, Self::어) if opts.옛글자방식 => Some(Self::오어),
            (Self::오, Self::에) if opts.옛글자방식 => Some(Self::오에),
            (Self::오, Self::여) if opts.옛글자방식 => Some(Self::오여),
            (Self::오, Self::예) if opts.옛글자방식 => Some(Self::오예),
            (Self::오, Self::오) if opts.옛글자방식 => Some(Self::오오),
            (Self::오, Self::우) if opts.옛글자방식 => Some(Self::오우),

            (Self::요, Self::아) if opts.옛글자방식 => Some(Self::요아),
            (Self::요, Self::애) if opts.옛글자방식 => Some(Self::요애),
            (Self::요, Self::야) if opts.옛글자방식 => Some(Self::요야),
            (Self::요, Self::얘) if opts.옛글자방식 => Some(Self::요얘),
            (Self::요, Self::어) if opts.옛글자방식 => Some(Self::요어),
            (Self::요, Self::여) if opts.옛글자방식 => Some(Self::요여),
            (Self::요, Self::오) if opts.옛글자방식 => Some(Self::요오),
            (Self::요, Self::이) if opts.옛글자방식 => Some(Self::요이),
            (Self::우, Self::아) if opts.옛글자방식 => Some(Self::우아),
            (Self::우, Self::애) if opts.옛글자방식 => Some(Self::우애),

            (Self::우, Self::여) if opts.옛글자방식 => Some(Self::우여),
            (Self::우, Self::예) if opts.옛글자방식 => Some(Self::우예),
            (Self::우, Self::우) if opts.옛글자방식 => Some(Self::우우),

            (Self::우, Self::어으) if opts.옛글자방식 => Some(Self::우어으),
            (Self::우, Self::이이) if opts.옛글자방식 => Some(Self::우이이),
            (Self::워, Self::으) if opts.옛글자방식 => Some(Self::우어으),

            (Self::위, Self::이) if opts.옛글자방식 => Some(Self::우이이),
            (Self::유, Self::아) if opts.옛글자방식 => Some(Self::유아),
            (Self::유, Self::애) if opts.옛글자방식 => Some(Self::유애),
            (Self::유, Self::어) if opts.옛글자방식 => Some(Self::유어),
            (Self::유, Self::에) if opts.옛글자방식 => Some(Self::유에),
            (Self::유, Self::여) if opts.옛글자방식 => Some(Self::유여),
            (Self::유, Self::예) if opts.옛글자방식 => Some(Self::유예),
            (Self::유, Self::오) if opts.옛글자방식 => Some(Self::유오),
            (Self::유, Self::우) if opts.옛글자방식 => Some(Self::유우),
            (Self::유, Self::이) if opts.옛글자방식 => Some(Self::유이),
            (Self::으, Self::아) if opts.옛글자방식 => Some(Self::으아),
            (Self::으, Self::어) if opts.옛글자방식 => Some(Self::으어),
            (Self::으, Self::에) if opts.옛글자방식 => Some(Self::으에),
            (Self::으, Self::오) if opts.옛글자방식 => Some(Self::으오),
            (Self::으, Self::우) if opts.옛글자방식 => Some(Self::으우),
            (Self::으, Self::으) if opts.옛글자방식 => Some(Self::으으),

            _ => None,
        }
    }
}
impl 끝소리 {
    pub const fn 옛글자인가(self) -> bool {
        self as u32 > 26
    }

    pub const fn try_add(self, other: Self, opts: 입력설정) -> Option<Self> {
        let compose_ssang = opts.두번치기_조합;

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
            _ if opts.옛글자방식 => match (self, other) {
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

    pub const fn 첫소리로(self) -> 끝소리To첫소리 {
        use 끝소리To첫소리::{Compose, Direct};
        match self {
            Self::기윽 => Direct(첫소리::기윽),
            Self::된기윽 => Direct(첫소리::된기윽),
            Self::기윽시읏 => Compose(Self::기윽, 첫소리::시읏),
            Self::니은 => Direct(첫소리::니은),
            Self::니은지읒 => Compose(Self::니은, 첫소리::지읒),
            Self::니은히읗 => Compose(Self::니은, 첫소리::히읗),
            Self::디읃 => Direct(첫소리::디읃),
            Self::리을 => Direct(첫소리::리을),
            Self::리을기윽 => Compose(Self::리을, 첫소리::기윽),
            Self::리을미음 => Compose(Self::리을, 첫소리::미음),
            Self::리을비읍 => Compose(Self::리을, 첫소리::비읍),
            Self::리을시읏 => Compose(Self::리을, 첫소리::시읏),
            Self::리을티읕 => Compose(Self::리을, 첫소리::티읕),
            Self::리을피읖 => Compose(Self::리을, 첫소리::피읖),
            Self::리을히읗 => Compose(Self::리을, 첫소리::히읗),
            Self::미음 => Direct(첫소리::미음),
            Self::비읍 => Direct(첫소리::비읍),
            Self::비읍시읏 => Compose(Self::비읍, 첫소리::시읏),
            Self::시읏 => Direct(첫소리::시읏),
            Self::된시읏 => Direct(첫소리::된시읏),
            Self::이응 => Direct(첫소리::이응),
            Self::지읒 => Direct(첫소리::지읒),
            Self::치읓 => Direct(첫소리::치읓),
            Self::키읔 => Direct(첫소리::키읔),
            Self::티읕 => Direct(첫소리::티읕),
            Self::피읖 => Direct(첫소리::피읖),
            Self::히읗 => Direct(첫소리::히읗),
            Self::기윽리을 => Compose(Self::기윽, 첫소리::리을),
            Self::기윽시읏기윽 => Compose(Self::기윽시읏, 첫소리::기윽),
            Self::니은기윽 => Compose(Self::니은, 첫소리::기윽),
            Self::니은디읃 => Compose(Self::니은, 첫소리::디읃),
            Self::니은시읏 => Compose(Self::니은, 첫소리::시읏),
            Self::니은반이소리 => Compose(Self::니은, 첫소리::반이소리),
            Self::니은티읕 => Compose(Self::니은, 첫소리::티읕),
            Self::디읃기윽 => Compose(Self::디읃, 첫소리::기윽),
            Self::디읃리을 => Compose(Self::디읃, 첫소리::리을),
            Self::리을기윽시읏 => Compose(Self::리을기윽, 첫소리::시읏),
            Self::리을니은 => Compose(Self::리을, 첫소리::니은),
            Self::리을디읃 => Compose(Self::리을, 첫소리::디읃),
            Self::리을디읃히읗 => Compose(Self::리을디읃, 첫소리::히읗),
            Self::두리을 => Compose(Self::리을, 첫소리::리을),
            Self::리을미음기윽 => Compose(Self::리을미음, 첫소리::기윽),
            Self::리을미음시읏 => Compose(Self::리을미음, 첫소리::시읏),
            Self::리을비읍시읏 => Compose(Self::리을비읍, 첫소리::시읏),
            Self::리을비읍히읗 => Compose(Self::리을비읍, 첫소리::히읗),
            Self::리을가벼운비읍 => Compose(Self::리을비읍, 첫소리::이응),
            Self::리을두시읏 => Compose(Self::리을시읏, 첫소리::시읏),
            Self::리을반이소리 => Compose(Self::리을, 첫소리::반이소리),
            Self::리을키읔 => Compose(Self::리을, 첫소리::키읔),
            Self::리을목구멍터집소리 => Compose(Self::리을, 첫소리::목구멍터집소리),
            Self::미음기윽 => Compose(Self::미음, 첫소리::기윽),
            Self::미음리을 => Compose(Self::미음, 첫소리::리을),
            Self::미음비읍 => Compose(Self::미음, 첫소리::비읍),
            Self::미음시읏 => Compose(Self::미음, 첫소리::시읏),
            Self::미음두시읏 => Compose(Self::미음시읏, 첫소리::시읏),
            Self::미음반이소리 => Compose(Self::미음, 첫소리::반이소리),
            Self::미음치읓 => Compose(Self::미음, 첫소리::치읓),
            Self::미음히읗 => Compose(Self::미음, 첫소리::히읗),
            Self::가벼운미음 => Compose(Self::미음, 첫소리::이응),
            Self::비읍리을 => Compose(Self::비읍, 첫소리::리을),
            Self::비읍피읖 => Compose(Self::비읍, 첫소리::피읖),
            Self::비읍히읗 => Compose(Self::비읍, 첫소리::히읗),
            Self::가벼운비읍 => Compose(Self::비읍, 첫소리::이응),
            Self::시읏기윽 => Compose(Self::시읏, 첫소리::기윽),
            Self::시읏디읃 => Compose(Self::시읏, 첫소리::디읃),
            Self::시읏리을 => Compose(Self::시읏, 첫소리::리을),
            Self::시읏비읍 => Compose(Self::시읏, 첫소리::비읍),
            Self::반이소리 => Direct(첫소리::반이소리),
            Self::이응기윽 => Compose(Self::어금이소리, 첫소리::기윽),
            Self::이응두기윽 => Compose(Self::어금이소리, 첫소리::된기윽),
            Self::두이응 => Compose(Self::어금이소리, 첫소리::어금이소리),
            Self::이응키읔 => Compose(Self::어금이소리, 첫소리::키읔),
            Self::어금이소리 => Direct(첫소리::어금이소리),
            Self::어금이소리시읏 => Compose(Self::어금이소리, 첫소리::시읏),
            Self::어금이소리반이소리 => Compose(Self::어금이소리, 첫소리::반이소리),
            Self::피읖비읍 => Compose(Self::피읖, 첫소리::비읍),
            Self::가벼운피읖 => Compose(Self::피읖, 첫소리::이응),
            Self::히읗니은 => Compose(Self::히읗, 첫소리::니은),
            Self::히읗리을 => Compose(Self::히읗, 첫소리::리을),
            Self::히읗미음 => Compose(Self::히읗, 첫소리::미음),
            Self::히읗비읍 => Compose(Self::히읗, 첫소리::비읍),
            Self::목구멍터집소리 => Direct(첫소리::목구멍터집소리),
            Self::기윽니은 => Compose(Self::기윽, 첫소리::니은),
            Self::기윽비읍 => Compose(Self::기윽, 첫소리::비읍),
            Self::기윽치읓 => Compose(Self::기윽, 첫소리::치읓),
            Self::기윽키읔 => Compose(Self::기윽, 첫소리::키읔),
            Self::기윽히읗 => Compose(Self::기윽, 첫소리::히읗),
            Self::두니은 => Compose(Self::니은, 첫소리::니은),
            Self::니은리을 => Compose(Self::니은, 첫소리::리을),
            Self::니은치읓 => Compose(Self::니은, 첫소리::치읓),
            Self::두디읃 => Compose(Self::디읃, 첫소리::디읃),
            Self::두디읃비읍 => Compose(Self::두디읃, 첫소리::비읍),
            Self::디읃비읍 => Compose(Self::디읃, 첫소리::비읍),
            Self::디읃시읏 => Compose(Self::디읃, 첫소리::시읏),
            Self::디읃시읏기윽 => Compose(Self::디읃시읏, 첫소리::기윽),
            Self::디읃지읒 => Compose(Self::디읃, 첫소리::지읒),
            Self::디읃치읓 => Compose(Self::디읃, 첫소리::치읓),
            Self::디읃티읕 => Compose(Self::디읃, 첫소리::티읕),
            Self::리을두기윽 => Compose(Self::리을기윽, 첫소리::기윽),
            Self::리을기윽히읗 => Compose(Self::리을기윽, 첫소리::히읗),
            Self::두리을키읔 => Compose(Self::두리을, 첫소리::키읔),
            Self::리을미음히읗 => Compose(Self::리을미음, 첫소리::히읗),
            Self::리을비읍디읃 => Compose(Self::리을비읍, 첫소리::디읃),
            Self::리을비읍피읖 => Compose(Self::리을비읍, 첫소리::피읖),
            Self::리을어금이소리 => Compose(Self::리을, 첫소리::어금이소리),
            Self::리을목구멍터집소리히읗 => {
                Compose(Self::리을목구멍터집소리, 첫소리::히읗)
            }
            Self::가벼운리을 => Compose(Self::리을, 첫소리::이응),
            Self::미음니은 => Compose(Self::미음, 첫소리::니은),
            Self::미음두니은 => Compose(Self::미음니은, 첫소리::니은),
            Self::두미음 => Compose(Self::미음, 첫소리::미음),
            Self::미음비읍시읏 => Compose(Self::미음비읍, 첫소리::시읏),
            Self::미음지읒 => Compose(Self::미음, 첫소리::지읒),
            Self::비읍디읃 => Compose(Self::비읍, 첫소리::디읃),
            Self::비읍리을피읖 => Compose(Self::비읍리을, 첫소리::피읖),
            Self::비읍미음 => Compose(Self::비읍, 첫소리::미음),
            Self::두비읍 => Compose(Self::비읍, 첫소리::비읍),
            Self::비읍시읏디읃 => Compose(Self::비읍시읏, 첫소리::디읃),
            Self::비읍지읒 => Compose(Self::비읍, 첫소리::지읒),
            Self::비읍치읓 => Compose(Self::비읍, 첫소리::치읓),
            Self::시읏미음 => Compose(Self::시읏, 첫소리::미음),
            Self::시읏가벼운비읍 => Compose(Self::시읏비읍, 첫소리::이응),
            Self::두시읏기윽 => Compose(Self::된시읏, 첫소리::기윽),
            Self::두시읏디읃 => Compose(Self::된시읏, 첫소리::디읃),
            Self::시읏반이소리 => Compose(Self::시읏, 첫소리::반이소리),
            Self::시읏지읒 => Compose(Self::시읏, 첫소리::지읒),
            Self::시읏치읓 => Compose(Self::시읏, 첫소리::치읓),
            Self::시읏티읕 => Compose(Self::시읏, 첫소리::티읕),
            Self::시읏히읗 => Compose(Self::시읏, 첫소리::히읗),
            Self::반이소리비읍 => Compose(Self::반이소리, 첫소리::비읍),
            Self::반이소리가벼운비읍 => Compose(Self::반이소리비읍, 첫소리::이응),
            Self::어금이소리미음 => Compose(Self::어금이소리, 첫소리::미음),
            Self::어금이소리히읗 => Compose(Self::어금이소리, 첫소리::히읗),
            Self::지읒비읍 => Compose(Self::지읒, 첫소리::비읍),
            Self::지읒두비읍 => Compose(Self::지읒비읍, 첫소리::비읍),
            Self::두지읒 => Compose(Self::지읒, 첫소리::지읒),
            Self::피읖시읏 => Compose(Self::피읖, 첫소리::시읏),
            Self::피읖티읕 => Compose(Self::피읖, 첫소리::티읕),
        }
    }
}

pub enum 끝소리To첫소리 {
    Direct(첫소리),
    Compose(끝소리, 첫소리),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum 건값 {
    첫소리 {
        첫소리: 첫소리,
    },
    가운데소리 {
        가운데소리: 가운데소리,
        조합: bool,
    },
    끝소리 {
        끝소리: 끝소리,
    },
    둘다 {
        첫소리: 첫소리,
        끝소리: 끝소리,
    },
    첫소리끝소리 {
        첫소리: 첫소리,
        끝소리: 끝소리,
        첫번째: bool,
    },
    첫소리가운데소리 {
        첫소리: 첫소리,
        가운데소리: 가운데소리,
        첫번째: bool,
        조합: bool,
    },
    가운데소리끝소리 {
        가운데소리: 가운데소리,
        끝소리: 끝소리,
        첫번째: bool,
        조합: bool,
    },
    통과(char),
}

impl 건값 {
    pub const fn 옛글자가_있는가(&self) -> bool {
        match self {
            Self::첫소리 { 첫소리 } => 첫소리.옛글자인가(),
            Self::가운데소리 {
                가운데소리, ..
            } => 가운데소리.옛글자인가(),
            Self::끝소리 { 끝소리 } => 끝소리.옛글자인가(),
            Self::둘다 {
                첫소리, 끝소리
            } => 첫소리.옛글자인가() || 끝소리.옛글자인가(),
            Self::첫소리끝소리 {
                첫소리, 끝소리,
            ..
            } => 첫소리.옛글자인가() || 끝소리.옛글자인가(),
            Self::첫소리가운데소리 {
                첫소리, 가운데소리,
            ..
            } => 첫소리.옛글자인가() || 가운데소리.옛글자인가(),
            Self::가운데소리끝소리 {
                가운데소리, 끝소리,
            ..
            } => 가운데소리.옛글자인가() || 끝소리.옛글자인가(),
            Self::통과(_) => false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeyValuePart {
    Cho {
        첫소리: 첫소리,
    },
    Jung {
        가운데소리: 가운데소리,
        조합: bool,
    },
    Jong {
        끝소리: 끝소리,
    },
}

impl KeyValuePart {
    fn parse(chars: &mut std::str::Chars) -> Option<Self> {
        use crate::char_utils::호환자모를_결합자모로;

        let 첫소리로 = |c: char| {
            첫소리::from_첫소리(c).or_else(|| 첫소리::from_첫소리(호환자모를_결합자모로(c)))
        };
        let to_medial = |c: char| {
            가운데소리::from_첫소리(c)
                .or_else(|| 가운데소리::from_첫소리(호환자모를_결합자모로(c)))
        };
        let to_final = |c: char| {
            끝소리::from_첫소리(c).or_else(|| 끝소리::from_첫소리(호환자모를_결합자모로(c)))
        };

        match chars.next()? {
            '$' => {
                let next = chars.next()?;
                if let Some(가운데소리) = to_medial(next) {
                    Some(Self::Jung {
                        가운데소리,
                        조합: false,
                    })
                } else {
                    Some(Self::Jong {
                        끝소리: to_final(next)?,
                    })
                }
            }
            c => {
                if let Some(첫소리) = 첫소리로(c) {
                    Some(Self::Cho { 첫소리 })
                } else {
                    Some(Self::Jung {
                        가운데소리: to_medial(c)?,
                        조합: true,
                    })
                }
            }
        }
    }
}

impl FromStr for 건값 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut next = || KeyValuePart::parse(&mut chars);

        match next() {
            None => Ok(Self::통과(s.chars().next().ok_or(())?)),
            Some(첫번째) => match 첫번째 {
                KeyValuePart::Cho { 첫소리 } => match next() {
                    Some(KeyValuePart::Jong { 끝소리 }) => Ok(Self::둘다 {
                        첫소리, 끝소리
                    }),
                    Some(KeyValuePart::Jung {
                        가운데소리,
                        조합,
                    }) => Ok(Self::첫소리가운데소리 {
                        첫소리,
                        가운데소리,
                        첫번째: true,
                        조합,
                    }),
                    None => {
                        // Check if it's also a valid 끝소리
                        첫소리_끝소리_변환(첫소리).map_or(
                            Ok(Self::첫소리 { 첫소리 }),
                            |끝소리| {
                                Ok(Self::둘다 {
                                    첫소리, 끝소리
                                })
                            },
                        )
                    }
                    _ => Err(()),
                },
                KeyValuePart::Jung {
                    가운데소리,
                    조합,
                } => match next() {
                    Some(KeyValuePart::Cho { 첫소리 }) => Ok(Self::첫소리가운데소리 {
                        첫소리,
                        가운데소리,
                        첫번째: false,
                        조합,
                    }),
                    Some(KeyValuePart::Jong { 끝소리 }) => Ok(Self::가운데소리끝소리 {
                        가운데소리,
                        끝소리,
                        첫번째: true,
                        조합,
                    }),
                    None => Ok(Self::가운데소리 {
                        가운데소리,
                        조합,
                    }),
                    _ => Err(()),
                },
                KeyValuePart::Jong { 끝소리 } => match next() {
                    Some(KeyValuePart::Cho { 첫소리 }) => Ok(Self::둘다 {
                        첫소리, 끝소리
                    }),
                    Some(KeyValuePart::Jung {
                        가운데소리,
                        조합,
                    }) => Ok(Self::가운데소리끝소리 {
                        가운데소리,
                        끝소리,
                        첫번째: false,
                        조합,
                    }),
                    None => Ok(Self::끝소리 { 끝소리 }),
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
        let (첫소리, 가운데소리, 끝소리) = 첫소리::분해('앙').unwrap();
        assert_eq!(첫소리, 첫소리::이응);
        assert_eq!(가운데소리, 가운데소리::아);
        assert_eq!(끝소리, Some(끝소리::이응));
        assert_eq!(첫소리.조합(가운데소리, 끝소리), "앙");
    }

    #[test]
    fn test_parse_keyvalue() {
        assert_eq!(
            "ㅇ".parse::<건값>().unwrap(),
            건값::둘다 {
                첫소리: 첫소리::이응,
                끝소리: 끝소리::이응
            }
        );
        assert_eq!(
            "ㅏ".parse::<건값>().unwrap(),
            건값::가운데소리 {
                가운데소리: 가운데소리::아,
                조합: true
            }
        );
        assert_eq!(
            "ㅋ$ㄱ".parse::<건값>().unwrap(),
            건값::둘다 {
                첫소리: 첫소리::키읔,
                끝소리: 끝소리::기윽,
            }
        );
    }

    #[test]
    fn test_old_jamo_initial() {
        assert_eq!(char::from(첫소리::목구멍터집소리), 'ᅙ');
        assert_eq!(char::from(첫소리::니은히읗), 'ᅝ');
        assert_eq!(char::from(첫소리::이머리소리시읏), 'ᄼ');
    }

    #[test]
    fn test_old_jamo_medial() {
        // Araea is U+119E (Conjoining Jamo), not U+3152 (Compatibility Jamo)
        assert_eq!(char::from(가운데소리::아래아), 'ᆞ');
    }

    #[test]
    fn test_old_jamo_final() {
        assert_eq!(char::from(끝소리::목구멍터집소리), 'ᇹ');
    }

    #[test]
    fn test_old_jamo_mode_combinations() {
        let mut opts = 입력설정 {
            옛글자방식: false,
            두번치기_조합: true,
            ..Default::default()
        };
        assert!(첫소리::기윽.try_add(첫소리::기윽, opts).is_some());

        opts.옛글자방식 = true;
        // Old initial combination: ᄂ + ᄒ → ᅝ
        assert_eq!(
            첫소리::니은.try_add(첫소리::히읗, opts),
            Some(첫소리::니은히읗)
        );
    }
}
