use super::jamo::{Initial, Medial, Final, InputOptions, initial_to_final};
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
        if (self as u32) < 19 && (medial_sound as u32) < 21 && final_sound.is_none_or(|j| (j as u32) < 27) {
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
                (Self::Kiuk, Self::Kiuk) => return Some(Self::ToenKiuk),
                (Self::Piup, Self::Piup) => return Some(Self::ToenPiup),
                (Self::Siut, Self::Siut) => return Some(Self::ToenSiut),
                (Self::Jiut, Self::Jiut) => return Some(Self::ToenJiut),
                (Self::Tiut, Self::Tiut) => return Some(Self::ToenTiut),
                _ => {}
            }
        }

        if opts.old_jamo_mode {
            if let result @ Some(_) = match (self, other) {
                (Self::Kiuk, Self::Kiuk) => Some(Self::ToenKiuk),
                (Self::Kiuk, Self::Tiut) => Some(Self::KiukTiut),
                (Self::Niun, Self::Kiuk) => Some(Self::NiunKiuk),
                (Self::Niun, Self::Niun) => Some(Self::TuNiun),
                (Self::Niun, Self::Tiut) => Some(Self::NiunTiut),
                (Self::Niun, Self::Piup) => Some(Self::NiunPiup),
                (Self::Niun, Self::Siut) => Some(Self::NiunSiut),
                (Self::Niun, Self::Jiut) => Some(Self::NiunJiut),
                (Self::Niun, Self::Hiut) => Some(Self::NiunHiut),
                (Self::Tiut, Self::Kiuk) => Some(Self::TiutKiuk),
                (Self::Tiut, Self::Tiut) => Some(Self::ToenTiut),
                (Self::Tiut, Self::Riul) => Some(Self::TiutRiul),
                (Self::Tiut, Self::Mium) => Some(Self::TiutMium),
                (Self::Tiut, Self::Piup) => Some(Self::TiutPiup),
                (Self::Tiut, Self::Siut) => Some(Self::TiutSiut),
                (Self::Tiut, Self::Jiut) => Some(Self::TiutJiut),
                (Self::Riul, Self::Kiuk) => Some(Self::RiulKiuk),
                (Self::Riul, Self::ToenKiuk) => Some(Self::RiulTuKiuk),
                (Self::Riul, Self::Niun) => Some(Self::RiulNiun),
                (Self::Riul, Self::Tiut) => Some(Self::RiulTiut),
                (Self::Riul, Self::ToenTiut) => Some(Self::RiulTuTiut),
                (Self::Riul, Self::Riul) => Some(Self::TuRiul),
                (Self::Riul, Self::Mium) => Some(Self::RiulMium),
                (Self::Riul, Self::Piup) => Some(Self::RiulPiup),
                (Self::Riul, Self::ToenPiup) => Some(Self::RiulTuPiup),
                (Self::Riul, Self::Siut) => Some(Self::RiulSiut),
                (Self::Riul, Self::Iung) => Some(Self::KabyounRiul),
                (Self::Riul, Self::Jiut) => Some(Self::RiulJiut),
                (Self::Riul, Self::Khiuk) => Some(Self::RiulKhiuk),
                (Self::Riul, Self::Hiut) => Some(Self::RiulHiut),
                (Self::Riul, Self::KabyounPiup) => Some(Self::RiulKabyounPiup),
                (Self::Mium, Self::Kiuk) => Some(Self::MiumKiuk),
                (Self::Mium, Self::Tiut) => Some(Self::MiumTiut),
                (Self::Mium, Self::Piup) => Some(Self::MiumPiup),
                (Self::Mium, Self::Siut) => Some(Self::MiumSiut),
                (Self::Mium, Self::Iung) => Some(Self::KabyounMium),
                (Self::Piup, Self::Kiuk) => Some(Self::PiupKiuk),
                (Self::Piup, Self::Niun) => Some(Self::PiupNiun),
                (Self::Piup, Self::Tiut) => Some(Self::PiupTiut),
                (Self::Piup, Self::Piup) => Some(Self::ToenPiup),
                (Self::Piup, Self::Siut) => Some(Self::PiupSiut),
                (Self::Piup, Self::ToenSiut) => Some(Self::PiupTuSiut),
                (Self::Piup, Self::Iung) => Some(Self::KabyounPiup),
                (Self::Piup, Self::Jiut) => Some(Self::PiupJiut),
                (Self::Piup, Self::Chiut) => Some(Self::PiupChiut),
                (Self::Piup, Self::Khiuk) => Some(Self::PiupKhiuk),
                (Self::Piup, Self::Thiut) => Some(Self::PiupThiut),
                (Self::Piup, Self::Phiup) => Some(Self::PiupPhiup),
                (Self::Piup, Self::Hiut) => Some(Self::PiupHiut),
                (Self::Piup, Self::KabyounPiup) => Some(Self::KabyounTuPiup),
                (Self::Piup, Self::SiutKiuk) => Some(Self::PiupSiutKiuk),
                (Self::Piup, Self::SiutTiut) => Some(Self::PiupSiutTiut),
                (Self::Piup, Self::SiutPiup) => Some(Self::PiupSiutPiup),
                (Self::Piup, Self::SiutJiut) => Some(Self::PiupSiutJiut),
                (Self::Piup, Self::SiutThiut) => Some(Self::PiupSiutThiut),
                (Self::ToenPiup, Self::Iung) => Some(Self::KabyounTuPiup),
                (Self::Siut, Self::Kiuk) => Some(Self::SiutKiuk),
                (Self::Siut, Self::Niun) => Some(Self::SiutNiun),
                (Self::Siut, Self::Tiut) => Some(Self::SiutTiut),
                (Self::Siut, Self::Riul) => Some(Self::SiutRiul),
                (Self::Siut, Self::Mium) => Some(Self::SiutMium),
                (Self::Siut, Self::Piup) => Some(Self::SiutPiup),
                (Self::Siut, Self::Siut) => Some(Self::ToenSiut),
                (Self::Siut, Self::ToenSiut) => Some(Self::SiutTuSiut),
                (Self::Siut, Self::Iung) => Some(Self::SiutIung),
                (Self::Siut, Self::Jiut) => Some(Self::SiutJiut),
                (Self::Siut, Self::Chiut) => Some(Self::SiutChiut),
                (Self::Siut, Self::Khiuk) => Some(Self::SiutKhiuk),
                (Self::Siut, Self::Thiut) => Some(Self::SiutThiut),
                (Self::Siut, Self::Phiup) => Some(Self::SiutPhiup),
                (Self::Siut, Self::Hiut) => Some(Self::SiutHiut),
                (Self::Siut, Self::PiupKiuk) => Some(Self::SiutPiupKiuk),
                (Self::Siut, Self::SiutPiup) => Some(Self::TuSiutPiup),
                (Self::ToenSiut, Self::Piup) => Some(Self::TuSiutPiup),
                (Self::ToenSiut, Self::Siut) => Some(Self::SiutTuSiut),
                (Self::Iung, Self::Kiuk) => Some(Self::IungKiuk),
                (Self::Iung, Self::Tiut) => Some(Self::IungTiut),
                (Self::Iung, Self::Riul) => Some(Self::IungRiul),
                (Self::Iung, Self::Mium) => Some(Self::IungMium),
                (Self::Iung, Self::Piup) => Some(Self::IungPiup),
                (Self::Iung, Self::Siut) => Some(Self::IungSiut),
                (Self::Iung, Self::Iung) => Some(Self::TuIung),
                (Self::Iung, Self::Jiut) => Some(Self::IungJiut),
                (Self::Iung, Self::Chiut) => Some(Self::IungChiut),
                (Self::Iung, Self::Thiut) => Some(Self::IungThiut),
                (Self::Iung, Self::Phiup) => Some(Self::IungPhiup),
                (Self::Iung, Self::Hiut) => Some(Self::IungHiut),
                (Self::Iung, Self::Panisori) => Some(Self::IungPanisori),
                (Self::Jiut, Self::Iung) => Some(Self::JiutIung),
                (Self::Jiut, Self::Jiut) => Some(Self::ToenJiut),
                (Self::ToenJiut, Self::Hiut) => Some(Self::TuJiutHiut),
                (Self::Chiut, Self::Khiuk) => Some(Self::ChiutKhiuk),
                (Self::Chiut, Self::Hiut) => Some(Self::ChiutHiut),
                (Self::Thiut, Self::Thiut) => Some(Self::TuThiut),
                (Self::Phiup, Self::Piup) => Some(Self::PhiupPiup),
                (Self::Phiup, Self::Iung) => Some(Self::KabyounPhiup),
                (Self::Phiup, Self::Hiut) => Some(Self::PhiupHiut),
                (Self::Hiut, Self::Siut) => Some(Self::HiutSiut),
                (Self::Hiut, Self::Hiut) => Some(Self::TuHiut),
                (Self::PiupSiut, Self::Kiuk) => Some(Self::PiupSiutKiuk),
                (Self::PiupSiut, Self::Tiut) => Some(Self::PiupSiutTiut),
                (Self::PiupSiut, Self::Piup) => Some(Self::PiupSiutPiup),
                (Self::PiupSiut, Self::Siut) => Some(Self::PiupTuSiut),
                (Self::PiupSiut, Self::Jiut) => Some(Self::PiupSiutJiut),
                (Self::PiupSiut, Self::Thiut) => Some(Self::PiupSiutThiut),
                (Self::SiutPiup, Self::Kiuk) => Some(Self::SiutPiupKiuk),
                (Self::ImorisoriSiut, Self::ImorisoriSiut) => Some(Self::ImorisoriTuSiut),
                (Self::ImomsoriSiut, Self::ImomsoriSiut) => Some(Self::ImomsoriTuSiut),
                (Self::ImorisoriJiut, Self::ImorisoriJiut) => Some(Self::ImorisoriTuJiut),
                (Self::ImomsoriJiut, Self::ImomsoriJiut) => Some(Self::ImomsoriTuJiut),
                (Self::Mokkumongthojimsori, Self::Mokkumongthojimsori) => Some(Self::TuMokkumongthojimsori),
                (Self::RiulKiuk, Self::Kiuk) => Some(Self::RiulTuKiuk),
                (Self::RiulTiut, Self::Tiut) => Some(Self::RiulTuTiut),
                (Self::RiulPiup, Self::Piup) => Some(Self::RiulTuPiup),
                (Self::RiulPiup, Self::Iung) => Some(Self::RiulKabyounPiup),
                _ => None
            } {
                return result;
            }
        }

        match (self, other) {
            (Self::Chiut, Self::Hiut) => Some(Self::Chiut),
            _ => None,
        }
    }

    pub const fn backspace(self) -> Option<Self> {
        match self {
            Self::ToenKiuk => Some(Self::Kiuk),
            Self::ToenPiup => Some(Self::Piup),
            Self::ToenSiut => Some(Self::Siut),
            Self::ToenJiut => Some(Self::Jiut),
            Self::ToenTiut => Some(Self::Tiut),
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
            (Self::A, Self::I) => Some(Self::Ae),
            (Self::Ya, Self::I) => Some(Self::Yae),
            (Self::Eo, Self::I) => Some(Self::E),
            (Self::Yeo, Self::I) => Some(Self::Ye),
            (Self::O, Self::A) => Some(Self::Wa),
            (Self::O, Self::I) => Some(Self::Oe),
            (Self::O, Self::Ae) | (Self::Wa, Self::I) => Some(Self::Wae),
            (Self::U, Self::Eo) => Some(Self::Weo),
            (Self::U, Self::I) => Some(Self::Wi),
            (Self::U, Self::E) | (Self::Weo, Self::I) => Some(Self::We),
            (Self::Eu, Self::I) => Some(Self::Yi),
            (Self::Yi, Self::U) if opts.old_jamo_mode => Some(Self::YiU),
            (Self::I, Self::A) if opts.old_jamo_mode => Some(Self::IA),
            (Self::I, Self::Ya) if opts.old_jamo_mode => Some(Self::IYa),
            (Self::I, Self::Yae) if opts.old_jamo_mode => Some(Self::IYae),
            (Self::I, Self::Yeo) if opts.old_jamo_mode => Some(Self::IYeo),
            (Self::I, Self::Ye) if opts.old_jamo_mode => Some(Self::IYe),
            (Self::I, Self::O) if opts.old_jamo_mode => Some(Self::IO),
            (Self::I, Self::Yo) if opts.old_jamo_mode => Some(Self::IYo),
            (Self::I, Self::U) if opts.old_jamo_mode => Some(Self::IU),
            (Self::I, Self::Yu) if opts.old_jamo_mode => Some(Self::IYu),
            (Self::I, Self::Eu) if opts.old_jamo_mode => Some(Self::IEu),
            (Self::I, Self::I) if opts.old_jamo_mode => Some(Self::II),
            (Self::I, Self::YaO) if opts.old_jamo_mode => Some(Self::IYaO),
            (Self::I, Self::Araea) if opts.old_jamo_mode => Some(Self::IAraea),
            (Self::OO, Self::I) if opts.old_jamo_mode => Some(Self::OOI),
            (Self::IYa, Self::O) if opts.old_jamo_mode => Some(Self::IYaO),
            (Self::IO, Self::I) if opts.old_jamo_mode => Some(Self::IOI),
            (Self::Araea, Self::A) if opts.old_jamo_mode => Some(Self::AraeaA),
            (Self::Araea, Self::Eo) if opts.old_jamo_mode => Some(Self::AraeaEo),
            (Self::Araea, Self::E) if opts.old_jamo_mode => Some(Self::AraeaE),
            (Self::Araea, Self::U) if opts.old_jamo_mode => Some(Self::AraeaU),
            (Self::Araea, Self::I) if opts.old_jamo_mode => Some(Self::AraeaI),
            (Self::Araea, Self::Araea) if opts.old_jamo_mode => Some(Self::TuAraea),
            (Self::AraeaA, Self::A) if opts.old_jamo_mode => Some(Self::TuAraea),

            (Self::A, Self::A) if opts.old_jamo_mode => Some(Self::Araea),
            (Self::A, Self::O) if opts.old_jamo_mode => Some(Self::AO),
            (Self::A, Self::U) if opts.old_jamo_mode => Some(Self::AU),
            (Self::A, Self::Eu) if opts.old_jamo_mode => Some(Self::AEu),

            (Self::Ya, Self::O) if opts.old_jamo_mode => Some(Self::YaO),
            (Self::Ya, Self::Yo) if opts.old_jamo_mode => Some(Self::YaYo),
            (Self::Ya, Self::U) if opts.old_jamo_mode => Some(Self::YaU),

            (Self::Eo, Self::O) if opts.old_jamo_mode => Some(Self::EoO),
            (Self::Eo, Self::U) if opts.old_jamo_mode => Some(Self::EoU),
            (Self::Eo, Self::Eu) if opts.old_jamo_mode => Some(Self::EoEu),

            (Self::Yeo, Self::Ya) if opts.old_jamo_mode => Some(Self::YeoYa),
            (Self::Yeo, Self::O) if opts.old_jamo_mode => Some(Self::YeoO),
            (Self::Yeo, Self::U) if opts.old_jamo_mode => Some(Self::YeoU),

            (Self::O, Self::Ya) if opts.old_jamo_mode => Some(Self::OYa),
            (Self::O, Self::Yae) if opts.old_jamo_mode => Some(Self::OYae),
            (Self::O, Self::Eo) if opts.old_jamo_mode => Some(Self::OEo),
            (Self::O, Self::E) if opts.old_jamo_mode => Some(Self::OE),
            (Self::O, Self::Yeo) if opts.old_jamo_mode => Some(Self::OYeo),
            (Self::O, Self::Ye) if opts.old_jamo_mode => Some(Self::OYe),
            (Self::O, Self::O) if opts.old_jamo_mode => Some(Self::OO),
            (Self::O, Self::U) if opts.old_jamo_mode => Some(Self::OU),

            (Self::Yo, Self::A) if opts.old_jamo_mode => Some(Self::YoA),
            (Self::Yo, Self::Ae) if opts.old_jamo_mode => Some(Self::YoAe),
            (Self::Yo, Self::Ya) if opts.old_jamo_mode => Some(Self::YoYa),
            (Self::Yo, Self::Yae) if opts.old_jamo_mode => Some(Self::YoYae),
            (Self::Yo, Self::Eo) if opts.old_jamo_mode => Some(Self::YoEo),
            (Self::Yo, Self::Yeo) if opts.old_jamo_mode => Some(Self::YoYeo),
            (Self::Yo, Self::O) if opts.old_jamo_mode => Some(Self::YoO),
            (Self::Yo, Self::I) if opts.old_jamo_mode => Some(Self::YoI),
            (Self::U, Self::A) if opts.old_jamo_mode => Some(Self::UA),
            (Self::U, Self::Ae) if opts.old_jamo_mode => Some(Self::UAe),

            (Self::U, Self::Yeo) if opts.old_jamo_mode => Some(Self::UYeo),
            (Self::U, Self::Ye) if opts.old_jamo_mode => Some(Self::UYe),
            (Self::U, Self::U) if opts.old_jamo_mode => Some(Self::UU),

            (Self::U, Self::EoEu) if opts.old_jamo_mode => Some(Self::UEoEu),
            (Self::U, Self::II) if opts.old_jamo_mode => Some(Self::UII),
            (Self::Weo, Self::Eu) if opts.old_jamo_mode => Some(Self::UEoEu),

            (Self::Wi, Self::I) if opts.old_jamo_mode => Some(Self::UII),
            (Self::Yu, Self::A) if opts.old_jamo_mode => Some(Self::YuA),
            (Self::Yu, Self::Ae) if opts.old_jamo_mode => Some(Self::YuAe),
            (Self::Yu, Self::Eo) if opts.old_jamo_mode => Some(Self::YuEo),
            (Self::Yu, Self::E) if opts.old_jamo_mode => Some(Self::YuE),
            (Self::Yu, Self::Yeo) if opts.old_jamo_mode => Some(Self::YuYeo),
            (Self::Yu, Self::Ye) if opts.old_jamo_mode => Some(Self::YuYe),
            (Self::Yu, Self::O) if opts.old_jamo_mode => Some(Self::YuO),
            (Self::Yu, Self::U) if opts.old_jamo_mode => Some(Self::YuU),
            (Self::Yu, Self::I) if opts.old_jamo_mode => Some(Self::YuI),
            (Self::Eu, Self::A) if opts.old_jamo_mode => Some(Self::EuA),
            (Self::Eu, Self::Eo) if opts.old_jamo_mode => Some(Self::EuEo),
            (Self::Eu, Self::E) if opts.old_jamo_mode => Some(Self::EuE),
            (Self::Eu, Self::O) if opts.old_jamo_mode => Some(Self::EuO),
            (Self::Eu, Self::U) if opts.old_jamo_mode => Some(Self::EuU),
            (Self::Eu, Self::Eu) if opts.old_jamo_mode => Some(Self::EuEu),

            _ => None,
        }
    }

    pub const fn backspace(self) -> Option<Self> {
        match self {
            Self::Ae => Some(Self::A),
            Self::Yae => Some(Self::Ya),
            Self::Ye => Some(Self::Yeo),
            Self::Wa | Self::Oe | Self::Wae => Some(Self::O),
            Self::Weo | Self::Wi | Self::We => Some(Self::U),
            Self::Yi => Some(Self::Eu),
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
            (Self::Kiuk, Self::Kiuk) if compose_ssang => Some(Self::ToenKiuk),
            (Self::Siut, Self::Siut) if compose_ssang => Some(Self::ToenSiut),
            (Self::Kiuk, Self::Siut) => Some(Self::KiukSiut),
            (Self::Niun, Self::Hiut) => Some(Self::NiunHiut),
            (Self::Niun, Self::Jiut) => Some(Self::NiunJiut),
            (Self::Riul, Self::Kiuk) => Some(Self::RiulKiuk),
            (Self::Riul, Self::Mium) => Some(Self::RiulMium),
            (Self::Riul, Self::Piup) => Some(Self::RiulPiup),
            (Self::Riul, Self::Siut) => Some(Self::RiulSiut),
            (Self::Riul, Self::Thiut) => Some(Self::RiulThiut),
            (Self::Riul, Self::Phiup) => Some(Self::RiulPhiup),
            (Self::Riul, Self::Hiut) => Some(Self::RiulHiut),
            (Self::Piup, Self::Siut) => Some(Self::PiupSiut),
            _ if opts.old_jamo_mode => {
                match (self, other) {
                    (Self::Kiuk, Self::Kiuk) => Some(Self::ToenKiuk),
                    (Self::Kiuk, Self::Niun) => Some(Self::KiukNiun),
                    (Self::Kiuk, Self::Riul) => Some(Self::KiukRiul),
                    (Self::Kiuk, Self::Piup) => Some(Self::KiukPiup),
                    (Self::Kiuk, Self::Siut) => Some(Self::KiukSiut),
                    (Self::Kiuk, Self::Chiut) => Some(Self::KiukChiut),
                    (Self::Kiuk, Self::Khiuk) => Some(Self::KiukKhiuk),
                    (Self::Kiuk, Self::Hiut) => Some(Self::KiukHiut),
                    (Self::Kiuk, Self::SiutKiuk) => Some(Self::KiukSiutKiuk),
                    (Self::KiukSiut, Self::Kiuk) => Some(Self::KiukSiutKiuk),
                    (Self::Niun, Self::Kiuk) => Some(Self::NiunKiuk),
                    (Self::Niun, Self::Niun) => Some(Self::TuNiun),
                    (Self::Niun, Self::Tiut) => Some(Self::NiunTiut),
                    (Self::Niun, Self::Riul) => Some(Self::NiunRiul),
                    (Self::Niun, Self::Siut) => Some(Self::NiunSiut),
                    (Self::Niun, Self::Jiut) => Some(Self::NiunJiut),
                    (Self::Niun, Self::Chiut) => Some(Self::NiunChiut),
                    (Self::Niun, Self::Thiut) => Some(Self::NiunThiut),
                    (Self::Niun, Self::Hiut) => Some(Self::NiunHiut),
                    (Self::Niun, Self::Panisori) => Some(Self::NiunPanisori),
                    (Self::Tiut, Self::Kiuk) => Some(Self::TiutKiuk),
                    (Self::Tiut, Self::Tiut) => Some(Self::TuTiut),
                    (Self::Tiut, Self::Riul) => Some(Self::TiutRiul),
                    (Self::Tiut, Self::Piup) => Some(Self::TiutPiup),
                    (Self::Tiut, Self::Siut) => Some(Self::TiutSiut),
                    (Self::Tiut, Self::Jiut) => Some(Self::TiutJiut),
                    (Self::Tiut, Self::Chiut) => Some(Self::TiutChiut),
                    (Self::Tiut, Self::Thiut) => Some(Self::TiutThiut),
                    (Self::Tiut, Self::SiutKiuk) => Some(Self::TiutSiutKiuk),
                    (Self::Tiut, Self::TiutPiup) => Some(Self::TuTiutPiup),
                    (Self::Riul, Self::Kiuk) => Some(Self::RiulKiuk),
                    (Self::Riul, Self::ToenKiuk) => Some(Self::RiulTuKiuk),
                    (Self::Riul, Self::KiukSiut) => Some(Self::RiulKiukSiut),
                    (Self::Riul, Self::Niun) => Some(Self::RiulNiun),
                    (Self::Riul, Self::Tiut) => Some(Self::RiulTiut),
                    (Self::Riul, Self::Riul) => Some(Self::TuRiul),
                    (Self::Riul, Self::Mium) => Some(Self::RiulMium),
                    (Self::Riul, Self::Piup) => Some(Self::RiulPiup),
                    (Self::Riul, Self::PiupSiut) => Some(Self::RiulPiupSiut),
                    (Self::Riul, Self::Siut) => Some(Self::RiulSiut),
                    (Self::Riul, Self::ToenSiut) => Some(Self::RiulTuSiut),
                    (Self::Riul, Self::Iung) => Some(Self::KabyounRiul),
                    (Self::Riul, Self::Khiuk) => Some(Self::RiulKhiuk),
                    (Self::Riul, Self::Thiut) => Some(Self::RiulThiut),
                    (Self::Riul, Self::Phiup) => Some(Self::RiulPhiup),
                    (Self::Riul, Self::Hiut) => Some(Self::RiulHiut),
                    (Self::Riul, Self::RiulKhiuk) => Some(Self::TuRiulKhiuk),
                    (Self::Riul, Self::MiumKiuk) => Some(Self::RiulMiumKiuk),
                    (Self::Riul, Self::MiumSiut) => Some(Self::RiulMiumSiut),
                    (Self::Riul, Self::MiumHiut) => Some(Self::RiulMiumHiut),
                    (Self::Riul, Self::PiupPhiup) => Some(Self::RiulPiupPhiup),
                    (Self::Riul, Self::PiupHiut) => Some(Self::RiulPiupHiut),
                    (Self::Riul, Self::KabyounPiup) => Some(Self::RiulKabyounPiup),
                    (Self::Riul, Self::Panisori) => Some(Self::RiulPanisori),
                    (Self::Riul, Self::Ogumisori) => Some(Self::RiulOgumisori),
                    (Self::Riul, Self::Mokkumongthojimsori) => Some(Self::RiulMokkumongthojimsori),
                    (Self::Riul, Self::KiukHiut) => Some(Self::RiulKiukHiut),
                    (Self::Riul, Self::PiupTiut) => Some(Self::RiulPiupTiut),
                    (Self::RiulKiuk, Self::Kiuk) => Some(Self::RiulTuKiuk),
                    (Self::RiulKiuk, Self::Siut) => Some(Self::RiulKiukSiut),
                    (Self::RiulKiuk, Self::Hiut) => Some(Self::RiulKiukHiut),
                    (Self::RiulMium, Self::Kiuk) => Some(Self::RiulMiumKiuk),
                    (Self::RiulMium, Self::Siut) => Some(Self::RiulMiumSiut),
                    (Self::RiulMium, Self::Hiut) => Some(Self::RiulMiumHiut),
                    (Self::RiulPiup, Self::Tiut) => Some(Self::RiulPiupTiut),
                    (Self::RiulPiup, Self::Siut) => Some(Self::RiulPiupSiut),
                    (Self::RiulPiup, Self::Iung) => Some(Self::RiulKabyounPiup),
                    (Self::RiulPiup, Self::Phiup) => Some(Self::RiulPiupPhiup),
                    (Self::RiulPiup, Self::Hiut) => Some(Self::RiulPiupHiut),
                    (Self::RiulSiut, Self::Siut) => Some(Self::RiulTuSiut),
                    (Self::Mium, Self::Kiuk) => Some(Self::MiumKiuk),
                    (Self::Mium, Self::Niun) => Some(Self::MiumNiun),
                    (Self::Mium, Self::Riul) => Some(Self::MiumRiul),
                    (Self::Mium, Self::Mium) => Some(Self::TuMium),
                    (Self::Mium, Self::Piup) => Some(Self::MiumPiup),
                    (Self::Mium, Self::PiupSiut) => Some(Self::MiumPiupSiut),
                    (Self::Mium, Self::Siut) => Some(Self::MiumSiut),
                    (Self::Mium, Self::ToenSiut) => Some(Self::MiumTuSiut),
                    (Self::Mium, Self::Iung) => Some(Self::KabyounMium),
                    (Self::Mium, Self::Jiut) => Some(Self::MiumJiut),
                    (Self::Mium, Self::Chiut) => Some(Self::MiumChiut),
                    (Self::Mium, Self::Hiut) => Some(Self::MiumHiut),
                    (Self::Mium, Self::Panisori) => Some(Self::MiumPanisori),
                    (Self::Mium, Self::TuNiun) => Some(Self::MiumTuNiun),
                    (Self::Piup, Self::Tiut) => Some(Self::PiupTiut),
                    (Self::Piup, Self::Riul) => Some(Self::PiupRiul),
                    (Self::Piup, Self::RiulPhiup) => Some(Self::PiupRiulPhiup),
                    (Self::Piup, Self::Mium) => Some(Self::PiupMium),
                    (Self::Piup, Self::Piup) => Some(Self::TuPiup),
                    (Self::Piup, Self::Siut) => Some(Self::PiupSiut),
                    (Self::Piup, Self::Iung) => Some(Self::KabyounPiup),
                    (Self::Piup, Self::Jiut) => Some(Self::PiupJiut),
                    (Self::Piup, Self::Chiut) => Some(Self::PiupChiut),
                    (Self::Piup, Self::Phiup) => Some(Self::PiupPhiup),
                    (Self::Piup, Self::Hiut) => Some(Self::PiupHiut),
                    (Self::Piup, Self::SiutTiut) => Some(Self::PiupSiutTiut),
                    (Self::PiupSiut, Self::Tiut) => Some(Self::PiupSiutTiut),
                    (Self::Siut, Self::Kiuk) => Some(Self::SiutKiuk),
                    (Self::Siut, Self::Tiut) => Some(Self::SiutTiut),
                    (Self::Siut, Self::Riul) => Some(Self::SiutRiul),
                    (Self::Siut, Self::Mium) => Some(Self::SiutMium),
                    (Self::Siut, Self::Piup) => Some(Self::SiutPiup),
                    (Self::Siut, Self::Siut) => Some(Self::ToenSiut),
                    (Self::Siut, Self::Jiut) => Some(Self::SiutJiut),
                    (Self::Siut, Self::Chiut) => Some(Self::SiutChiut),
                    (Self::Siut, Self::Thiut) => Some(Self::SiutThiut),
                    (Self::Siut, Self::Hiut) => Some(Self::SiutHiut),
                    (Self::Siut, Self::KabyounPiup) => Some(Self::SiutKabyounPiup),
                    (Self::Siut, Self::SiutKiuk) => Some(Self::TuSiutKiuk),
                    (Self::Siut, Self::SiutTiut) => Some(Self::TuSiutTiut),
                    (Self::Siut, Self::Panisori) => Some(Self::SiutPanisori),
                    (Self::ToenSiut, Self::Kiuk) => Some(Self::TuSiutKiuk),
                    (Self::ToenSiut, Self::Tiut) => Some(Self::TuSiutTiut),
                    (Self::Jiut, Self::Piup) => Some(Self::JiutPiup),
                    (Self::Jiut, Self::Jiut) => Some(Self::TuJiut),
                    (Self::Jiut, Self::TuPiup) => Some(Self::JiutTuPiup),
                    (Self::Phiup, Self::Piup) => Some(Self::PhiupPiup),
                    (Self::Phiup, Self::Siut) => Some(Self::PhiupSiut),
                    (Self::Phiup, Self::Iung) => Some(Self::KabyounPhiup),
                    (Self::Phiup, Self::Thiut) => Some(Self::PhiupThiut),
                    (Self::Hiut, Self::Niun) => Some(Self::HiutNiun),
                    (Self::Hiut, Self::Riul) => Some(Self::HiutRiul),
                    (Self::Hiut, Self::Mium) => Some(Self::HiutMium),
                    (Self::Hiut, Self::Piup) => Some(Self::HiutPiup),
                    (Self::RiulTiut, Self::Hiut) => Some(Self::RiulTiutHiut),
                    (Self::TuRiul, Self::Khiuk) => Some(Self::TuRiulKhiuk),
                    (Self::RiulMokkumongthojimsori, Self::Hiut) => Some(Self::RiulMokkumongthojimsoriHiut),
                    (Self::MiumPiup, Self::Siut) => Some(Self::MiumPiupSiut),
                    (Self::MiumSiut, Self::Siut) => Some(Self::MiumTuSiut),
                    (Self::PiupRiul, Self::Phiup) => Some(Self::PiupRiulPhiup),
                    (Self::SiutPiup, Self::Iung) => Some(Self::SiutKabyounPiup),
                    (Self::Panisori, Self::Piup) => Some(Self::PanisoriPiup),
                    (Self::Panisori, Self::KabyounPiup) => Some(Self::PanisoriKabyounPiup),
                    (Self::IungKiuk, Self::Kiuk) => Some(Self::IungTuKiuk),
                    (Self::Ogumisori, Self::Kiuk) => Some(Self::IungKiuk),
                    (Self::Ogumisori, Self::ToenKiuk) => Some(Self::IungTuKiuk),
                    (Self::Ogumisori, Self::Mium) => Some(Self::OgumisoriMium),
                    (Self::Ogumisori, Self::Siut) => Some(Self::OgumisoriSiut),
                    (Self::Ogumisori, Self::Khiuk) => Some(Self::IungKhiuk),
                    (Self::Ogumisori, Self::Hiut) => Some(Self::OgumisoriHiut),
                    (Self::Ogumisori, Self::Panisori) => Some(Self::OgumisoriPanisori),
                    (Self::Ogumisori, Self::Ogumisori) => Some(Self::TuIung),
                    (Self::TuTiut, Self::Piup) => Some(Self::TuTiutPiup),
                    (Self::TiutSiut, Self::Kiuk) => Some(Self::TiutSiutKiuk),
                    (Self::MiumNiun, Self::Niun) => Some(Self::MiumTuNiun),
                    (Self::PanisoriPiup, Self::Iung) => Some(Self::PanisoriKabyounPiup),
                    (Self::JiutPiup, Self::Piup) => Some(Self::JiutTuPiup),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub const fn backspace(self) -> Option<Self> {
        match self {
            Self::ToenKiuk | Self::KiukSiut => Some(Self::Kiuk),
            Self::ToenSiut => Some(Self::Siut),
            Self::NiunHiut | Self::NiunJiut => Some(Self::Niun),
            Self::RiulMium
            | Self::RiulPiup
            | Self::RiulSiut
            | Self::RiulThiut
            | Self::RiulHiut => Some(Self::Riul),
            Self::PiupSiut => Some(Self::Piup),
            _ => None,
        }
    }

    pub const fn to_initial(self) -> FinalToInitial {
        use FinalToInitial::{Compose, Direct};
        match self {
            Self::Kiuk => Direct(Initial::Kiuk),
            Self::ToenKiuk => Direct(Initial::ToenKiuk),
            Self::KiukSiut => Compose(Self::Kiuk, Initial::Siut),
            Self::Niun => Direct(Initial::Niun),
            Self::NiunJiut => Compose(Self::Niun, Initial::Jiut),
            Self::NiunHiut => Compose(Self::Niun, Initial::Hiut),
            Self::Tiut => Direct(Initial::Tiut),
            Self::Riul => Direct(Initial::Riul),
            Self::RiulKiuk => Compose(Self::Riul, Initial::Kiuk),
            Self::RiulMium => Compose(Self::Riul, Initial::Mium),
            Self::RiulPiup => Compose(Self::Riul, Initial::Piup),
            Self::RiulSiut => Compose(Self::Riul, Initial::Siut),
            Self::RiulThiut => Compose(Self::Riul, Initial::Thiut),
            Self::RiulPhiup => Compose(Self::Riul, Initial::Phiup),
            Self::RiulHiut => Compose(Self::Riul, Initial::Hiut),
            Self::Mium => Direct(Initial::Mium),
            Self::Piup => Direct(Initial::Piup),
            Self::PiupSiut => Compose(Self::Piup, Initial::Siut),
            Self::Siut => Direct(Initial::Siut),
            Self::ToenSiut => Direct(Initial::ToenSiut),
            Self::Iung => Direct(Initial::Iung),
            Self::Jiut => Direct(Initial::Jiut),
            Self::Chiut => Direct(Initial::Chiut),
            Self::Khiuk => Direct(Initial::Khiuk),
            Self::Thiut => Direct(Initial::Thiut),
            Self::Phiup => Direct(Initial::Phiup),
            Self::Hiut => Direct(Initial::Hiut),
            Self::KiukRiul => Compose(Self::Kiuk, Initial::Riul),
            Self::KiukSiutKiuk => Compose(Self::KiukSiut, Initial::Kiuk),
            Self::NiunKiuk => Compose(Self::Niun, Initial::Kiuk),
            Self::NiunTiut => Compose(Self::Niun, Initial::Tiut),
            Self::NiunSiut => Compose(Self::Niun, Initial::Siut),
            Self::NiunPanisori => Compose(Self::Niun, Initial::Panisori),
            Self::NiunThiut => Compose(Self::Niun, Initial::Thiut),
            Self::TiutKiuk => Compose(Self::Tiut, Initial::Kiuk),
            Self::TiutRiul => Compose(Self::Tiut, Initial::Riul),
            Self::RiulKiukSiut => Compose(Self::RiulKiuk, Initial::Siut),
            Self::RiulNiun => Compose(Self::Riul, Initial::Niun),
            Self::RiulTiut => Compose(Self::Riul, Initial::Tiut),
            Self::RiulTiutHiut => Compose(Self::RiulTiut, Initial::Hiut),
            Self::TuRiul => Compose(Self::Riul, Initial::Riul),
            Self::RiulMiumKiuk => Compose(Self::RiulMium, Initial::Kiuk),
            Self::RiulMiumSiut => Compose(Self::RiulMium, Initial::Siut),
            Self::RiulPiupSiut => Compose(Self::RiulPiup, Initial::Siut),
            Self::RiulPiupHiut => Compose(Self::RiulPiup, Initial::Hiut),
            Self::RiulKabyounPiup => Compose(Self::RiulPiup, Initial::Iung),
            Self::RiulTuSiut => Compose(Self::RiulSiut, Initial::Siut),
            Self::RiulPanisori => Compose(Self::Riul, Initial::Panisori),
            Self::RiulKhiuk => Compose(Self::Riul, Initial::Khiuk),
            Self::RiulMokkumongthojimsori => Compose(Self::Riul, Initial::Mokkumongthojimsori),
            Self::MiumKiuk => Compose(Self::Mium, Initial::Kiuk),
            Self::MiumRiul => Compose(Self::Mium, Initial::Riul),
            Self::MiumPiup => Compose(Self::Mium, Initial::Piup),
            Self::MiumSiut => Compose(Self::Mium, Initial::Siut),
            Self::MiumTuSiut => Compose(Self::MiumSiut, Initial::Siut),
            Self::MiumPanisori => Compose(Self::Mium, Initial::Panisori),
            Self::MiumChiut => Compose(Self::Mium, Initial::Chiut),
            Self::MiumHiut => Compose(Self::Mium, Initial::Hiut),
            Self::KabyounMium => Compose(Self::Mium, Initial::Iung),
            Self::PiupRiul => Compose(Self::Piup, Initial::Riul),
            Self::PiupPhiup => Compose(Self::Piup, Initial::Phiup),
            Self::PiupHiut => Compose(Self::Piup, Initial::Hiut),
            Self::KabyounPiup => Compose(Self::Piup, Initial::Iung),
            Self::SiutKiuk => Compose(Self::Siut, Initial::Kiuk),
            Self::SiutTiut => Compose(Self::Siut, Initial::Tiut),
            Self::SiutRiul => Compose(Self::Siut, Initial::Riul),
            Self::SiutPiup => Compose(Self::Siut, Initial::Piup),
            Self::Panisori => Direct(Initial::Panisori),
            Self::IungKiuk => Compose(Self::Ogumisori, Initial::Kiuk),
            Self::IungTuKiuk => Compose(Self::Ogumisori, Initial::ToenKiuk),
            Self::TuIung => Compose(Self::Ogumisori, Initial::Ogumisori),
            Self::IungKhiuk => Compose(Self::Ogumisori, Initial::Khiuk),
            Self::Ogumisori => Direct(Initial::Ogumisori),
            Self::OgumisoriSiut => Compose(Self::Ogumisori, Initial::Siut),
            Self::OgumisoriPanisori => Compose(Self::Ogumisori, Initial::Panisori),
            Self::PhiupPiup => Compose(Self::Phiup, Initial::Piup),
            Self::KabyounPhiup => Compose(Self::Phiup, Initial::Iung),
            Self::HiutNiun => Compose(Self::Hiut, Initial::Niun),
            Self::HiutRiul => Compose(Self::Hiut, Initial::Riul),
            Self::HiutMium => Compose(Self::Hiut, Initial::Mium),
            Self::HiutPiup => Compose(Self::Hiut, Initial::Piup),
            Self::Mokkumongthojimsori => Direct(Initial::Mokkumongthojimsori),
            Self::KiukNiun => Compose(Self::Kiuk, Initial::Niun),
            Self::KiukPiup => Compose(Self::Kiuk, Initial::Piup),
            Self::KiukChiut => Compose(Self::Kiuk, Initial::Chiut),
            Self::KiukKhiuk => Compose(Self::Kiuk, Initial::Khiuk),
            Self::KiukHiut => Compose(Self::Kiuk, Initial::Hiut),
            Self::TuNiun => Compose(Self::Niun, Initial::Niun),
            Self::NiunRiul => Compose(Self::Niun, Initial::Riul),
            Self::NiunChiut => Compose(Self::Niun, Initial::Chiut),
            Self::TuTiut => Compose(Self::Tiut, Initial::Tiut),
            Self::TuTiutPiup => Compose(Self::TuTiut, Initial::Piup),
            Self::TiutPiup => Compose(Self::Tiut, Initial::Piup),
            Self::TiutSiut => Compose(Self::Tiut, Initial::Siut),
            Self::TiutSiutKiuk => Compose(Self::TiutSiut, Initial::Kiuk),
            Self::TiutJiut => Compose(Self::Tiut, Initial::Jiut),
            Self::TiutChiut => Compose(Self::Tiut, Initial::Chiut),
            Self::TiutThiut => Compose(Self::Tiut, Initial::Thiut),
            Self::RiulTuKiuk => Compose(Self::RiulKiuk, Initial::Kiuk),
            Self::RiulKiukHiut => Compose(Self::RiulKiuk, Initial::Hiut),
            Self::TuRiulKhiuk => Compose(Self::TuRiul, Initial::Khiuk),
            Self::RiulMiumHiut => Compose(Self::RiulMium, Initial::Hiut),
            Self::RiulPiupTiut => Compose(Self::RiulPiup, Initial::Tiut),
            Self::RiulPiupPhiup => Compose(Self::RiulPiup, Initial::Phiup),
            Self::RiulOgumisori => Compose(Self::Riul, Initial::Ogumisori),
            Self::RiulMokkumongthojimsoriHiut => Compose(Self::RiulMokkumongthojimsori, Initial::Hiut),
            Self::KabyounRiul => Compose(Self::Riul, Initial::Iung),
            Self::MiumNiun => Compose(Self::Mium, Initial::Niun),
            Self::MiumTuNiun => Compose(Self::MiumNiun, Initial::Niun),
            Self::TuMium => Compose(Self::Mium, Initial::Mium),
            Self::MiumPiupSiut => Compose(Self::MiumPiup, Initial::Siut),
            Self::MiumJiut => Compose(Self::Mium, Initial::Jiut),
            Self::PiupTiut => Compose(Self::Piup, Initial::Tiut),
            Self::PiupRiulPhiup => Compose(Self::PiupRiul, Initial::Phiup),
            Self::PiupMium => Compose(Self::Piup, Initial::Mium),
            Self::TuPiup => Compose(Self::Piup, Initial::Piup),
            Self::PiupSiutTiut => Compose(Self::PiupSiut, Initial::Tiut),
            Self::PiupJiut => Compose(Self::Piup, Initial::Jiut),
            Self::PiupChiut => Compose(Self::Piup, Initial::Chiut),
            Self::SiutMium => Compose(Self::Siut, Initial::Mium),
            Self::SiutKabyounPiup => Compose(Self::SiutPiup, Initial::Iung),
            Self::TuSiutKiuk => Compose(Self::ToenSiut, Initial::Kiuk),
            Self::TuSiutTiut => Compose(Self::ToenSiut, Initial::Tiut),
            Self::SiutPanisori => Compose(Self::Siut, Initial::Panisori),
            Self::SiutJiut => Compose(Self::Siut, Initial::Jiut),
            Self::SiutChiut => Compose(Self::Siut, Initial::Chiut),
            Self::SiutThiut => Compose(Self::Siut, Initial::Thiut),
            Self::SiutHiut => Compose(Self::Siut, Initial::Hiut),
            Self::PanisoriPiup => Compose(Self::Panisori, Initial::Piup),
            Self::PanisoriKabyounPiup => Compose(Self::PanisoriPiup, Initial::Iung),
            Self::OgumisoriMium => Compose(Self::Ogumisori, Initial::Mium),
            Self::OgumisoriHiut => Compose(Self::Ogumisori, Initial::Hiut),
            Self::JiutPiup => Compose(Self::Jiut, Initial::Piup),
            Self::JiutTuPiup => Compose(Self::JiutPiup, Initial::Piup),
            Self::TuJiut => Compose(Self::Jiut, Initial::Jiut),
            Self::PhiupSiut => Compose(Self::Phiup, Initial::Siut),
            Self::PhiupThiut => Compose(Self::Phiup, Initial::Thiut),

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
            Self::Medial {
                medial_sound, ..
            } => medial_sound.is_old(),
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
                    Some(KeyValuePart::Jung { medial_sound, compose }) => Ok(Self::ChoJung {
                        initial_sound,
                        medial_sound,
                        first: true,
                        compose,
                    }),
                    None => {
                        // Check if it's also a valid final_sound
                        initial_to_final(initial_sound).map_or(Ok(Self::Initial { initial_sound }), |final_sound| {
                            Ok(Self::Both { initial_sound, final_sound })
                        })
                    }
                    _ => Err(()),
                },
                KeyValuePart::Jung { medial_sound, compose } => match next() {
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
                    None => Ok(Self::Medial { medial_sound, compose }),
                    _ => Err(()),
                },
                KeyValuePart::Jong { final_sound } => match next() {
                    Some(KeyValuePart::Cho { initial_sound }) => Ok(Self::Both {
                        initial_sound,
                        final_sound,
                    }),
                    Some(KeyValuePart::Jung { medial_sound, compose }) => Ok(Self::JungJong {
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
        assert_eq!(initial_sound, Initial::Iung);
        assert_eq!(medial_sound, Medial::A);
        assert_eq!(final_sound, Some(Final::Iung));
        assert_eq!(initial_sound.compose(medial_sound, final_sound), "앙");
    }

    #[test]
    fn test_parse_keyvalue() {
        assert_eq!(
            "ㅇ".parse::<KeyValue>().unwrap(),
            KeyValue::Both {
                initial_sound: Initial::Iung,
                final_sound: Final::Iung
            }
        );
        assert_eq!(
            "ㅏ".parse::<KeyValue>().unwrap(),
            KeyValue::Medial {
                medial_sound: Medial::A,
                compose: true
            }
        );
        assert_eq!(
            "ㅋ$ㄱ".parse::<KeyValue>().unwrap(),
            KeyValue::Both {
                initial_sound: Initial::Khiuk,
                final_sound: Final::Kiuk,
            }
        );
    }

    #[test]
    fn test_old_jamo_initial() {
        assert_eq!(char::from(Initial::Mokkumongthojimsori), 'ᅙ');
        assert_eq!(char::from(Initial::NiunHiut), 'ᅝ');
        assert_eq!(char::from(Initial::ImorisoriSiut), 'ᄼ');
    }

    #[test]
    fn test_old_jamo_medial() {
        // Araea is U+119E (Conjoining Jamo), not U+3152 (Compatibility Jamo)
        assert_eq!(char::from(Medial::Araea), 'ᆞ');
    }

    #[test]
    fn test_old_jamo_final() {
        assert_eq!(char::from(Final::Mokkumongthojimsori), 'ᇹ');
    }

    #[test]
    fn test_old_jamo_mode_combinations() {
        let mut opts = InputOptions {
            old_jamo_mode: false,
            combi_on_double_stroke: true,
            ..Default::default()
        };
        assert!(Initial::Kiuk.try_add(Initial::Kiuk, opts).is_some());

        opts.old_jamo_mode = true;
        // Old initial combination: ᄂ + ᄒ → ᅝ
        assert_eq!(
            Initial::Niun.try_add(Initial::Hiut, opts),
            Some(Initial::NiunHiut)
        );
    }
}
