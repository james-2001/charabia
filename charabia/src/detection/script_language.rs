use core::str::FromStr;

use super::chars;

macro_rules! make_language {
    ($($language:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum Language {
            $($language),+,
            Other,
        }
        impl From<whatlang::Lang> for Language {
            fn from(other: whatlang::Lang) -> Language {
                match other {
                    $(whatlang::Lang::$language => Language::$language), +
                }
            }
        }

        impl From<Language> for whatlang::Lang {
            fn from(other: Language) -> whatlang::Lang {
                match other {
                    $(Language::$language => whatlang::Lang::$language), +,
                    _other => whatlang::Lang::Eng,
                }
            }
        }

        impl Default for Language {
            fn default() -> Self {
                Self::Other
            }
        }

        impl Language {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Language::$language => whatlang::Lang::$language.code()), +,
                    _other => "other",
                }
            }

            pub fn from_name<S: AsRef<str>>(code: S) -> Language {
                whatlang::Lang::from_code(code.as_ref()).map(Language::from).unwrap_or_default()
            }
        }
    };
}

make_language! {
    Epo,
    Eng,
    Rus,
    Cmn,
    Spa,
    Por,
    Ita,
    Ben,
    Fra,
    Deu,
    Ukr,
    Kat,
    Ara,
    Hin,
    Jpn,
    Heb,
    Yid,
    Pol,
    Amh,
    Jav,
    Kor,
    Nob,
    Dan,
    Swe,
    Fin,
    Tur,
    Nld,
    Hun,
    Ces,
    Ell,
    Bul,
    Bel,
    Mar,
    Kan,
    Ron,
    Slv,
    Hrv,
    Srp,
    Mkd,
    Lit,
    Lav,
    Est,
    Tam,
    Vie,
    Urd,
    Tha,
    Guj,
    Uzb,
    Pan,
    Aze,
    Ind,
    Tel,
    Pes,
    Mal,
    Ori,
    Mya,
    Nep,
    Sin,
    Khm,
    Tuk,
    Aka,
    Zul,
    Sna,
    Afr,
    Lat,
    Slk,
    Cat,
    Tgl,
    Hye
}

macro_rules! make_script {
    ($($script:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum Script {
            $($script),+,
            Cj,
            Other,
        }

        impl From<whatlang::Script> for Script {
            fn from(other: whatlang::Script) -> Script {
                match other {
                    whatlang::Script::Hiragana |
                    whatlang::Script::Katakana |
                    whatlang::Script::Mandarin => Script::Cj,
                    $(whatlang::Script::$script => Script::$script), +
                }
            }

        }

        impl Script {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Script::$script => whatlang::Script::$script.name()), +,
                    Script::Cj => whatlang::Script::Mandarin.name(),
                    _other => "other",
                }
            }

            pub fn from_name<S: AsRef<str>>(code: S) -> Script {
                whatlang::Script::from_str(code.as_ref()).map(Script::from).unwrap_or_default()
            }
        }
    };
}

make_script! {
    Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Kannada,
    Khmer,
    Latin,
    Malayalam,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai
}

impl From<char> for Script {
    fn from(other: char) -> Script {
        if chars::is_latin(other) {
            Script::Latin
        } else if chars::is_cyrillic(other) {
            Script::Cyrillic
        } else if chars::is_arabic(other) {
            Script::Arabic
        } else if chars::is_devanagari(other) {
            Script::Devanagari
        } else if chars::is_hebrew(other) {
            Script::Hebrew
        } else if chars::is_ethiopic(other) {
            Script::Ethiopic
        } else if chars::is_georgian(other) {
            Script::Georgian
        } else if chars::is_bengali(other) {
            Script::Bengali
        } else if chars::is_hangul(other) {
            Script::Hangul
        } else if chars::is_hiragana(other)
            || chars::is_katakana(other)
            || chars::is_mandarin(other)
        {
            Script::Cj
        } else if chars::is_greek(other) {
            Script::Greek
        } else if chars::is_kannada(other) {
            Script::Kannada
        } else if chars::is_tamil(other) {
            Script::Tamil
        } else if chars::is_thai(other) {
            Script::Thai
        } else if chars::is_gujarati(other) {
            Script::Gujarati
        } else if chars::is_gurmukhi(other) {
            Script::Gurmukhi
        } else if chars::is_telugu(other) {
            Script::Telugu
        } else if chars::is_malayalam(other) {
            Script::Malayalam
        } else if chars::is_oriya(other) {
            Script::Oriya
        } else if chars::is_myanmar(other) {
            Script::Myanmar
        } else if chars::is_sinhala(other) {
            Script::Sinhala
        } else if chars::is_khmer(other) {
            Script::Khmer
        } else {
            Script::Other
        }
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::Other
    }
}

#[cfg(test)]
mod test {
    use crate::{Language, Script};

    #[test]
    fn from_into_language() {
        assert_eq!(Language::Eng.name(), "eng");
        assert_eq!(Language::from_name("eng"), Language::Eng);
        assert_eq!(Language::Jpn.name(), "jpn");
        assert_eq!(Language::from_name("jpn"), Language::Jpn);
        assert_eq!(Language::Cmn.name(), "cmn");
        assert_eq!(Language::from_name("cmn"), Language::Cmn);
    }

    #[test]
    fn from_into_script() {
        assert_eq!(Script::Latin.name(), "Latin");
        assert_eq!(Script::from_name("Latin"), Script::Latin);
        assert_eq!(Script::Cj.name(), "Mandarin");
        assert_eq!(Script::from_name("Mandarin"), Script::Cj);
    }
}
