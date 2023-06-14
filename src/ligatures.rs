//! Ligatures

// Each ligature is of the format:
//
//   ('<key>', <replacement>)
//
// Where <key> is used in the configuration and <replacement> is of the format:
//
//   ('<match>', ('<isolated>', '<initial>', '<medial>', '<final>'))
//
// Where <match> is the string to replace, and <isolated> is the replacement in
// case <match> was in isolated form, <initial> is the replacement in case
// <match> was in initial form, <medial> is the replacement in case <match> was
// in medial form, and <final> is the replacement in case <match> was in final
// form. If no replacement is specified for a form, then no replacement of
// <match> will occur.
//
// Order here is important, it should be:
//   1. Sentences
//   2. Words
//   3. Letters
// This way we make sure we replace the longest ligatures first

use std::ops::Range;

pub(crate) const SENTENCES_LIGATURES_RANGE: Range<usize> = 0..3;
pub(crate) const WORDS_LIGATURES_RANGE: Range<usize> = 3..12;
pub(crate) const LETTERS_LIGATURES_RANGE: Range<usize> = 12..286;

macro_rules! create_ligatures {
    ($ligatures_size:literal, $($id:ident => ($words:expr, [$isolated:literal, $initial:literal, $medial:literal, $final:literal]),)*) => {
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone, Eq, PartialEq, Hash)]
        #[repr(usize)]
        pub enum LigatureNames {
            $($id),*
        }

        pub const LIGATURES: [(&[&'static str], [&'static str; 4]); $ligatures_size] = [
            $(($words, [$isolated, $initial, $medial, $final])),*
        ];

    };
}

create_ligatures! [
    286,

    ARABIC_LIGATURE_BISMILLAH_AR_RAHMAN_AR_RAHEEM => (
        &[concat!(
            "\u{0628}\u{0633}\u{0645}\u{0020}",
            "\u{0627}\u{0644}\u{0644}\u{0647}\u{0020}",
            "\u{0627}\u{0644}\u{0631}\u{062D}\u{0645}\u{0646}\u{0020}",
            "\u{0627}\u{0644}\u{0631}\u{062D}\u{064A}\u{0645}",
        )],
        ["\u{FDFD}", "", "", ""]
    ),
    ARABIC_LIGATURE_JALLAJALALOUHOU => (
        &["\u{062C}\u{0644}\u{0020}\u{062C}\u{0644}\u{0627}\u{0644}\u{0647}"],
        ["\u{FDFB}", "", "", ""]
    ),
    ARABIC_LIGATURE_SALLALLAHOU_ALAYHE_WASALLAM => (
        &[concat!(
            "\u{0635}\u{0644}\u{0649}\u{0020}",
            "\u{0627}\u{0644}\u{0644}\u{0647}\u{0020}",
            "\u{0639}\u{0644}\u{064A}\u{0647}\u{0020}",
            "\u{0648}\u{0633}\u{0644}\u{0645}",
        )],
        ["\u{FDFA}", "", "", ""]
    ),

    ARABIC_LIGATURE_ALLAH => (
        &["\u{0627}\u{0644}\u{0644}\u{0647}"],
        ["\u{FDF2}", "", "", ""]
    ),
    ARABIC_LIGATURE_AKBAR => (
        &["\u{0623}\u{0643}\u{0628}\u{0631}"],
        ["\u{FDF3}", "", "", ""]
    ),
    ARABIC_LIGATURE_ALAYHE => (
        &["\u{0639}\u{0644}\u{064A}\u{0647}"],
        ["\u{FDF7}", "", "", ""]
    ),
    ARABIC_LIGATURE_MOHAMMAD => (
        &["\u{0645}\u{062D}\u{0645}\u{062F}"],
        ["\u{FDF4}", "", "", ""]
    ),
    ARABIC_LIGATURE_RASOUL => (
        &["\u{0631}\u{0633}\u{0648}\u{0644}"],
        ["\u{FDF6}", "", "", ""]
    ),
    ARABIC_LIGATURE_SALAM => (
        &["\u{0635}\u{0644}\u{0639}\u{0645}"],
        ["\u{FDF5}", "", "", ""]
    ),
    ARABIC_LIGATURE_SALLA => (
        &["\u{0635}\u{0644}\u{0649}"],
        ["\u{FDF9}", "", "", ""]
    ),
    ARABIC_LIGATURE_WASALLAM => (
        &["\u{0648}\u{0633}\u{0644}\u{0645}"],
        ["\u{FDF8}", "", "", ""]
    ),
    RIAL_SIGN => (
        &["\u{0631}[\u{06CC}\u{064A}]\u{0627}\u{0644}"],
        ["\u{FDFC}", "", "", ""]
    ),

    ARABIC_LIGATURE_AIN_WITH_ALEF_MAKSURA => (
        &["\u{0639}\u{0649}"],
        ["\u{FCF7}", "", "", "\u{FD13}"]
    ),
    ARABIC_LIGATURE_AIN_WITH_JEEM => (
        &["\u{0639}\u{062C}"],
        ["\u{FC29}", "\u{FCBA}", "", ""]
    ),
    ARABIC_LIGATURE_AIN_WITH_JEEM_WITH_MEEM => (
        &["\u{0639}\u{062C}\u{0645}"],
        ["", "\u{FDC4}", "", "\u{FD75}"]
    ),
    ARABIC_LIGATURE_AIN_WITH_MEEM => (
        &["\u{0639}\u{0645}"],
        ["\u{FC2A}", "\u{FCBB}", "", ""]
    ),
    ARABIC_LIGATURE_AIN_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{0639}\u{0645}\u{0649}"],
        ["", "", "", "\u{FD78}"]
    ),
    ARABIC_LIGATURE_AIN_WITH_MEEM_WITH_MEEM => (
        &["\u{0639}\u{0645}\u{0645}"],
        ["", "\u{FD77}", "", "\u{FD76}"]
    ),
    ARABIC_LIGATURE_AIN_WITH_MEEM_WITH_YEH => (
        &["\u{0639}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDB6}"]
    ),
    ARABIC_LIGATURE_AIN_WITH_YEH => (
        &["\u{0639}\u{064A}"],
        ["\u{FCF8}", "", "", "\u{FD14}"]
    ),
    ARABIC_LIGATURE_ALEF_MAKSURA_WITH_SUPERSCRIPT_ALEF => (
        &["\u{0649}\u{0670}"],
        ["\u{FC5D}", "", "", "\u{FC90}"]
    ),
    ARABIC_LIGATURE_ALEF_WITH_FATHATAN => (
        &["\u{0627}\u{064B}"],
        ["\u{FD3D}", "", "", "\u{FD3C}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_ALEF_MAKSURA => (
        &["\u{0628}\u{0649}"],
        ["\u{FC09}", "", "", "\u{FC6E}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_HAH => (
        &["\u{0628}\u{062D}"],
        ["\u{FC06}", "\u{FC9D}", "", ""]
    ),
    ARABIC_LIGATURE_BEH_WITH_HAH_WITH_YEH => (
        &["\u{0628}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDC2}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_HEH => (
        &["\u{0628}\u{0647}"],
        ["", "\u{FCA0}", "\u{FCE2}", ""]
    ),
    ARABIC_LIGATURE_BEH_WITH_JEEM => (
        &["\u{0628}\u{062C}"],
        ["\u{FC05}", "\u{FC9C}", "", ""]
    ),
    ARABIC_LIGATURE_BEH_WITH_KHAH => (
        &["\u{0628}\u{062E}"],
        ["\u{FC07}", "\u{FC9E}", "", ""]
    ),
    ARABIC_LIGATURE_BEH_WITH_KHAH_WITH_YEH => (
        &["\u{0628}\u{062E}\u{064A}"],
        ["", "", "", "\u{FD9E}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_MEEM => (
        &["\u{0628}\u{0645}"],
        ["\u{FC08}", "\u{FC9F}", "\u{FCE1}", "\u{FC6C}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_NOON => (
        &["\u{0628}\u{0646}"],
        ["", "", "", "\u{FC6D}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_REH => (
        &["\u{0628}\u{0631}"],
        ["", "", "", "\u{FC6A}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_YEH => (
        &["\u{0628}\u{064A}"],
        ["\u{FC0A}", "", "", "\u{FC6F}"]
    ),
    ARABIC_LIGATURE_BEH_WITH_ZAIN => (
        &["\u{0628}\u{0632}"],
        ["", "", "", "\u{FC6B}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_ALEF_MAKSURA => (
        &["\u{0636}\u{0649}"],
        ["\u{FD07}", "", "", "\u{FD23}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_HAH => (
        &["\u{0636}\u{062D}"],
        ["\u{FC23}", "\u{FCB5}", "", ""]
    ),
    ARABIC_LIGATURE_DAD_WITH_HAH_WITH_ALEF_MAKSURA => (
        &["\u{0636}\u{062D}\u{0649}"],
        ["", "", "", "\u{FD6E}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_HAH_WITH_YEH => (
        &["\u{0636}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDAB}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_JEEM => (
        &["\u{0636}\u{062C}"],
        ["\u{FC22}", "\u{FCB4}", "", ""]
    ),
    ARABIC_LIGATURE_DAD_WITH_KHAH => (
        &["\u{0636}\u{062E}"],
        ["\u{FC24}", "\u{FCB6}", "", ""]
    ),
    ARABIC_LIGATURE_DAD_WITH_KHAH_WITH_MEEM => (
        &["\u{0636}\u{062E}\u{0645}"],
        ["", "\u{FD70}", "", "\u{FD6F}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_MEEM => (
        &["\u{0636}\u{0645}"],
        ["\u{FC25}", "\u{FCB7}", "", ""]
    ),
    ARABIC_LIGATURE_DAD_WITH_REH => (
        &["\u{0636}\u{0631}"],
        ["\u{FD10}", "", "", "\u{FD2C}"]
    ),
    ARABIC_LIGATURE_DAD_WITH_YEH => (
        &["\u{0636}\u{064A}"],
        ["\u{FD08}", "", "", "\u{FD24}"]
    ),
    ARABIC_LIGATURE_FEH_WITH_ALEF_MAKSURA => (
        &["\u{0641}\u{0649}"],
        ["\u{FC31}", "", "", "\u{FC7C}"]
    ),
    ARABIC_LIGATURE_FEH_WITH_HAH => (
        &["\u{0641}\u{062D}"],
        ["\u{FC2E}", "\u{FCBF}", "", ""]
    ),
    ARABIC_LIGATURE_FEH_WITH_JEEM => (
        &["\u{0641}\u{062C}"],
        ["\u{FC2D}", "\u{FCBE}", "", ""]
    ),
    ARABIC_LIGATURE_FEH_WITH_KHAH => (
        &["\u{0641}\u{062E}"],
        ["\u{FC2F}", "\u{FCC0}", "", ""]
    ),
    ARABIC_LIGATURE_FEH_WITH_KHAH_WITH_MEEM => (
        &["\u{0641}\u{062E}\u{0645}"],
        ["", "\u{FD7D}", "", "\u{FD7C}"]
    ),
    ARABIC_LIGATURE_FEH_WITH_MEEM => (
        &["\u{0641}\u{0645}"],
        ["\u{FC30}", "\u{FCC1}", "", ""]
    ),
    ARABIC_LIGATURE_FEH_WITH_MEEM_WITH_YEH => (
        &["\u{0641}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDC1}"]
    ),
    ARABIC_LIGATURE_FEH_WITH_YEH => (
        &["\u{0641}\u{064A}"],
        ["\u{FC32}", "", "", "\u{FC7D}"]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_ALEF_MAKSURA => (
        &["\u{063A}\u{0649}"],
        ["\u{FCF9}", "", "", "\u{FD15}"]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_JEEM => (
        &["\u{063A}\u{062C}"],
        ["\u{FC2B}", "\u{FCBC}", "", ""]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_MEEM => (
        &["\u{063A}\u{0645}"],
        ["\u{FC2C}", "\u{FCBD}", "", ""]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{063A}\u{0645}\u{0649}"],
        ["", "", "", "\u{FD7B}"]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_MEEM_WITH_MEEM => (
        &["\u{063A}\u{0645}\u{0645}"],
        ["", "", "", "\u{FD79}"]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_MEEM_WITH_YEH => (
        &["\u{063A}\u{0645}\u{064A}"],
        ["", "", "", "\u{FD7A}"]
    ),
    ARABIC_LIGATURE_GHAIN_WITH_YEH => (
        &["\u{063A}\u{064A}"],
        ["\u{FCFA}", "", "", "\u{FD16}"]
    ),
    ARABIC_LIGATURE_HAH_WITH_ALEF_MAKSURA => (
        &["\u{062D}\u{0649}"],
        ["\u{FCFF}", "", "", "\u{FD1B}"]
    ),
    ARABIC_LIGATURE_HAH_WITH_JEEM => (
        &["\u{062D}\u{062C}"],
        ["\u{FC17}", "\u{FCA9}", "", ""]
    ),
    ARABIC_LIGATURE_HAH_WITH_JEEM_WITH_YEH => (
        &["\u{062D}\u{062C}\u{064A}"],
        ["", "", "", "\u{FDBF}"]
    ),
    ARABIC_LIGATURE_HAH_WITH_MEEM => (
        &["\u{062D}\u{0645}"],
        ["\u{FC18}", "\u{FCAA}", "", ""]
    ),
    ARABIC_LIGATURE_HAH_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{062D}\u{0645}\u{0649}"],
        ["", "", "", "\u{FD5B}"]
    ),
    ARABIC_LIGATURE_HAH_WITH_MEEM_WITH_YEH => (
        &["\u{062D}\u{0645}\u{064A}"],
        ["", "", "", "\u{FD5A}"]
    ),
    ARABIC_LIGATURE_HAH_WITH_YEH => (
        &["\u{062D}\u{064A}"],
        ["\u{FD00}", "", "", "\u{FD1C}"]
    ),
    ARABIC_LIGATURE_HEH_WITH_ALEF_MAKSURA => (
        &["\u{0647}\u{0649}"],
        ["\u{FC53}", "", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_JEEM => (
        &["\u{0647}\u{062C}"],
        ["\u{FC51}", "\u{FCD7}", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_MEEM => (
        &["\u{0647}\u{0645}"],
        ["\u{FC52}", "\u{FCD8}", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_MEEM_WITH_JEEM => (
        &["\u{0647}\u{0645}\u{062C}"],
        ["", "\u{FD93}", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_MEEM_WITH_MEEM => (
        &["\u{0647}\u{0645}\u{0645}"],
        ["", "\u{FD94}", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_SUPERSCRIPT_ALEF => (
        &["\u{0647}\u{0670}"],
        ["", "\u{FCD9}", "", ""]
    ),
    ARABIC_LIGATURE_HEH_WITH_YEH => (
        &["\u{0647}\u{064A}"],
        ["\u{FC54}", "", "", ""]
    ),
    ARABIC_LIGATURE_JEEM_WITH_ALEF_MAKSURA => (
        &["\u{062C}\u{0649}"],
        ["\u{FD01}", "", "", "\u{FD1D}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_HAH => (
        &["\u{062C}\u{062D}"],
        ["\u{FC15}", "\u{FCA7}", "", ""]
    ),
    ARABIC_LIGATURE_JEEM_WITH_HAH_WITH_ALEF_MAKSURA => (
        &["\u{062C}\u{062D}\u{0649}"],
        ["", "", "", "\u{FDA6}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_HAH_WITH_YEH => (
        &["\u{062C}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDBE}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_MEEM => (
        &["\u{062C}\u{0645}"],
        ["\u{FC16}", "\u{FCA8}", "", ""]
    ),
    ARABIC_LIGATURE_JEEM_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{062C}\u{0645}\u{0649}"],
        ["", "", "", "\u{FDA7}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_MEEM_WITH_HAH => (
        &["\u{062C}\u{0645}\u{062D}"],
        ["", "\u{FD59}", "", "\u{FD58}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_MEEM_WITH_YEH => (
        &["\u{062C}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDA5}"]
    ),
    ARABIC_LIGATURE_JEEM_WITH_YEH => (
        &["\u{062C}\u{064A}"],
        ["\u{FD02}", "", "", "\u{FD1E}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_ALEF => (
        &["\u{0643}\u{0627}"],
        ["\u{FC37}", "", "", "\u{FC80}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_ALEF_MAKSURA => (
        &["\u{0643}\u{0649}"],
        ["\u{FC3D}", "", "", "\u{FC83}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_HAH => (
        &["\u{0643}\u{062D}"],
        ["\u{FC39}", "\u{FCC5}", "", ""]
    ),
    ARABIC_LIGATURE_KAF_WITH_JEEM => (
        &["\u{0643}\u{062C}"],
        ["\u{FC38}", "\u{FCC4}", "", ""]
    ),
    ARABIC_LIGATURE_KAF_WITH_KHAH => (
        &["\u{0643}\u{062E}"],
        ["\u{FC3A}", "\u{FCC6}", "", ""]
    ),
    ARABIC_LIGATURE_KAF_WITH_LAM => (
        &["\u{0643}\u{0644}"],
        ["\u{FC3B}", "\u{FCC7}", "\u{FCEB}", "\u{FC81}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_MEEM => (
        &["\u{0643}\u{0645}"],
        ["\u{FC3C}", "\u{FCC8}", "\u{FCEC}", "\u{FC82}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_MEEM_WITH_MEEM => (
        &["\u{0643}\u{0645}\u{0645}"],
        ["", "\u{FDC3}", "", "\u{FDBB}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_MEEM_WITH_YEH => (
        &["\u{0643}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDB7}"]
    ),
    ARABIC_LIGATURE_KAF_WITH_YEH => (
        &["\u{0643}\u{064A}"],
        ["\u{FC3E}", "", "", "\u{FC84}"]
    ),
    ARABIC_LIGATURE_KHAH_WITH_ALEF_MAKSURA => (
        &["\u{062E}\u{0649}"],
        ["\u{FD03}", "", "", "\u{FD1F}"]
    ),
    ARABIC_LIGATURE_KHAH_WITH_HAH => (
        &["\u{062E}\u{062D}"],
        ["\u{FC1A}", "", "", ""]
    ),
    ARABIC_LIGATURE_KHAH_WITH_JEEM => (
        &["\u{062E}\u{062C}"],
        ["\u{FC19}", "\u{FCAB}", "", ""]
    ),
    ARABIC_LIGATURE_KHAH_WITH_MEEM => (
        &["\u{062E}\u{0645}"],
        ["\u{FC1B}", "\u{FCAC}", "", ""]
    ),
    ARABIC_LIGATURE_KHAH_WITH_YEH => (
        &["\u{062E}\u{064A}"],
        ["\u{FD04}", "", "", "\u{FD20}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_ALEF => (
        &["\u{0644}\u{0627}"],
        ["\u{FEFB}", "", "", "\u{FEFC}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_ALEF_MAKSURA => (
        &["\u{0644}\u{0649}"],
        ["\u{FC43}", "", "", "\u{FC86}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_ABOVE => (
        &["\u{0644}\u{0623}"],
        ["\u{FEF7}", "", "", "\u{FEF8}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_BELOW => (
        &["\u{0644}\u{0625}"],
        ["\u{FEF9}", "", "", "\u{FEFA}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_MADDA_ABOVE => (
        &["\u{0644}\u{0622}"],
        ["\u{FEF5}", "", "", "\u{FEF6}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_HAH => (
        &["\u{0644}\u{062D}"],
        ["\u{FC40}", "\u{FCCA}", "", ""]
    ),
    ARABIC_LIGATURE_LAM_WITH_HAH_WITH_ALEF_MAKSURA => (
        &["\u{0644}\u{062D}\u{0649}"],
        ["", "", "", "\u{FD82}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_HAH_WITH_MEEM => (
        &["\u{0644}\u{062D}\u{0645}"],
        ["", "\u{FDB5}", "", "\u{FD80}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_HAH_WITH_YEH => (
        &["\u{0644}\u{062D}\u{064A}"],
        ["", "", "", "\u{FD81}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_HEH => (
        &["\u{0644}\u{0647}"],
        ["", "\u{FCCD}", "", ""]
    ),
    ARABIC_LIGATURE_LAM_WITH_JEEM => (
        &["\u{0644}\u{062C}"],
        ["\u{FC3F}", "\u{FCC9}", "", ""]
    ),
    ARABIC_LIGATURE_LAM_WITH_JEEM_WITH_JEEM => (
        &["\u{0644}\u{062C}\u{062C}"],
        ["", "\u{FD83}", "", "\u{FD84}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_JEEM_WITH_MEEM => (
        &["\u{0644}\u{062C}\u{0645}"],
        ["", "\u{FDBA}", "", "\u{FDBC}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_JEEM_WITH_YEH => (
        &["\u{0644}\u{062C}\u{064A}"],
        ["", "", "", "\u{FDAC}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_KHAH => (
        &["\u{0644}\u{062E}"],
        ["\u{FC41}", "\u{FCCB}", "", ""]
    ),
    ARABIC_LIGATURE_LAM_WITH_KHAH_WITH_MEEM => (
        &["\u{0644}\u{062E}\u{0645}"],
        ["", "\u{FD86}", "", "\u{FD85}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_MEEM => (
        &["\u{0644}\u{0645}"],
        ["\u{FC42}", "\u{FCCC}", "\u{FCED}", "\u{FC85}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_MEEM_WITH_HAH => (
        &["\u{0644}\u{0645}\u{062D}"],
        ["", "\u{FD88}", "", "\u{FD87}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_MEEM_WITH_YEH => (
        &["\u{0644}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDAD}"]
    ),
    ARABIC_LIGATURE_LAM_WITH_YEH => (
        &["\u{0644}\u{064A}"],
        ["\u{FC44}", "", "", "\u{FC87}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_ALEF => (
        &["\u{0645}\u{0627}"],
        ["", "", "", "\u{FC88}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{0645}\u{0649}"],
        ["\u{FC49}", "", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_HAH => (
        &["\u{0645}\u{062D}"],
        ["\u{FC46}", "\u{FCCF}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_HAH_WITH_JEEM => (
        &["\u{0645}\u{062D}\u{062C}"],
        ["", "\u{FD89}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_HAH_WITH_MEEM => (
        &["\u{0645}\u{062D}\u{0645}"],
        ["", "\u{FD8A}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_HAH_WITH_YEH => (
        &["\u{0645}\u{062D}\u{064A}"],
        ["", "", "", "\u{FD8B}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_JEEM => (
        &["\u{0645}\u{062C}"],
        ["\u{FC45}", "\u{FCCE}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_JEEM_WITH_HAH => (
        &["\u{0645}\u{062C}\u{062D}"],
        ["", "\u{FD8C}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_JEEM_WITH_KHAH => (
        &["\u{0645}\u{062C}\u{062E}"],
        ["", "\u{FD92}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_JEEM_WITH_MEEM => (
        &["\u{0645}\u{062C}\u{0645}"],
        ["", "\u{FD8D}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_JEEM_WITH_YEH => (
        &["\u{0645}\u{062C}\u{064A}"],
        ["", "", "", "\u{FDC0}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_KHAH => (
        &["\u{0645}\u{062E}"],
        ["\u{FC47}", "\u{FCD0}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_KHAH_WITH_JEEM => (
        &["\u{0645}\u{062E}\u{062C}"],
        ["", "\u{FD8E}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_KHAH_WITH_MEEM => (
        &["\u{0645}\u{062E}\u{0645}"],
        ["", "\u{FD8F}", "", ""]
    ),
    ARABIC_LIGATURE_MEEM_WITH_KHAH_WITH_YEH => (
        &["\u{0645}\u{062E}\u{064A}"],
        ["", "", "", "\u{FDB9}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_MEEM => (
        &["\u{0645}\u{0645}"],
        ["\u{FC48}", "\u{FCD1}", "", "\u{FC89}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_MEEM_WITH_YEH => (
        &["\u{0645}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDB1}"]
    ),
    ARABIC_LIGATURE_MEEM_WITH_YEH => (
        &["\u{0645}\u{064A}"],
        ["\u{FC4A}", "", "", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_ALEF_MAKSURA => (
        &["\u{0646}\u{0649}"],
        ["\u{FC4F}", "", "", "\u{FC8E}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_HAH => (
        &["\u{0646}\u{062D}"],
        ["\u{FC4C}", "\u{FCD3}", "", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_HAH_WITH_ALEF_MAKSURA => (
        &["\u{0646}\u{062D}\u{0649}"],
        ["", "", "", "\u{FD96}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_HAH_WITH_MEEM => (
        &["\u{0646}\u{062D}\u{0645}"],
        ["", "\u{FD95}", "", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_HAH_WITH_YEH => (
        &["\u{0646}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDB3}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_HEH => (
        &["\u{0646}\u{0647}"],
        ["", "\u{FCD6}", "\u{FCEF}", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_JEEM => (
        &["\u{0646}\u{062C}"],
        ["\u{FC4B}", "\u{FCD2}", "", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_JEEM_WITH_ALEF_MAKSURA => (
        &["\u{0646}\u{062C}\u{0649}"],
        ["", "", "", "\u{FD99}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_JEEM_WITH_HAH => (
        &["\u{0646}\u{062C}\u{062D}"],
        ["", "\u{FDB8}", "", "\u{FDBD}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_JEEM_WITH_MEEM => (
        &["\u{0646}\u{062C}\u{0645}"],
        ["", "\u{FD98}", "", "\u{FD97}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_JEEM_WITH_YEH => (
        &["\u{0646}\u{062C}\u{064A}"],
        ["", "", "", "\u{FDC7}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_KHAH => (
        &["\u{0646}\u{062E}"],
        ["\u{FC4D}", "\u{FCD4}", "", ""]
    ),
    ARABIC_LIGATURE_NOON_WITH_MEEM => (
        &["\u{0646}\u{0645}"],
        ["\u{FC4E}", "\u{FCD5}", "\u{FCEE}", "\u{FC8C}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{0646}\u{0645}\u{0649}"],
        ["", "", "", "\u{FD9B}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_MEEM_WITH_YEH => (
        &["\u{0646}\u{0645}\u{064A}"],
        ["", "", "", "\u{FD9A}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_NOON => (
        &["\u{0646}\u{0646}"],
        ["", "", "", "\u{FC8D}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_REH => (
        &["\u{0646}\u{0631}"],
        ["", "", "", "\u{FC8A}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_YEH => (
        &["\u{0646}\u{064A}"],
        ["\u{FC50}", "", "", "\u{FC8F}"]
    ),
    ARABIC_LIGATURE_NOON_WITH_ZAIN => (
        &["\u{0646}\u{0632}"],
        ["", "", "", "\u{FC8B}"]
    ),
    ARABIC_LIGATURE_QAF_WITH_ALEF_MAKSURA => (
        &["\u{0642}\u{0649}"],
        ["\u{FC35}", "", "", "\u{FC7E}"]
    ),
    ARABIC_LIGATURE_QAF_WITH_HAH => (
        &["\u{0642}\u{062D}"],
        ["\u{FC33}", "\u{FCC2}", "", ""]
    ),
    ARABIC_LIGATURE_QAF_WITH_MEEM => (
        &["\u{0642}\u{0645}"],
        ["\u{FC34}", "\u{FCC3}", "", ""]
    ),
    ARABIC_LIGATURE_QAF_WITH_MEEM_WITH_HAH => (
        &["\u{0642}\u{0645}\u{062D}"],
        ["", "\u{FDB4}", "", "\u{FD7E}"]
    ),
    ARABIC_LIGATURE_QAF_WITH_MEEM_WITH_MEEM => (
        &["\u{0642}\u{0645}\u{0645}"],
        ["", "", "", "\u{FD7F}"]
    ),
    ARABIC_LIGATURE_QAF_WITH_MEEM_WITH_YEH => (
        &["\u{0642}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDB2}"]
    ),
    ARABIC_LIGATURE_QAF_WITH_YEH => (
        &["\u{0642}\u{064A}"],
        ["\u{FC36}", "", "", "\u{FC7F}"]
    ),
    ARABIC_LIGATURE_QALA_USED_AS_KORANIC_STOP_SIGN => (
        &["\u{0642}\u{0644}\u{06D2}"],
        ["\u{FDF1}", "", "", ""]
    ),
    ARABIC_LIGATURE_REH_WITH_SUPERSCRIPT_ALEF => (
        &["\u{0631}\u{0670}"],
        ["\u{FC5C}", "", "", ""]
    ),
    ARABIC_LIGATURE_SAD_WITH_ALEF_MAKSURA => (
        &["\u{0635}\u{0649}"],
        ["\u{FD05}", "", "", "\u{FD21}"]
    ),
    ARABIC_LIGATURE_SAD_WITH_HAH => (
        &["\u{0635}\u{062D}"],
        ["\u{FC20}", "\u{FCB1}", "", ""]
    ),
    ARABIC_LIGATURE_SAD_WITH_HAH_WITH_HAH => (
        &["\u{0635}\u{062D}\u{062D}"],
        ["", "\u{FD65}", "", "\u{FD64}"]
    ),
    ARABIC_LIGATURE_SAD_WITH_HAH_WITH_YEH => (
        &["\u{0635}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDA9}"]
    ),
    ARABIC_LIGATURE_SAD_WITH_KHAH => (
        &["\u{0635}\u{062E}"],
        ["", "\u{FCB2}", "", ""]
    ),
    ARABIC_LIGATURE_SAD_WITH_MEEM => (
        &["\u{0635}\u{0645}"],
        ["\u{FC21}", "\u{FCB3}", "", ""]
    ),
    ARABIC_LIGATURE_SAD_WITH_MEEM_WITH_MEEM => (
        &["\u{0635}\u{0645}\u{0645}"],
        ["", "\u{FDC5}", "", "\u{FD66}"]
    ),
    ARABIC_LIGATURE_SAD_WITH_REH => (
        &["\u{0635}\u{0631}"],
        ["\u{FD0F}", "", "", "\u{FD2B}"]
    ),
    ARABIC_LIGATURE_SAD_WITH_YEH => (
        &["\u{0635}\u{064A}"],
        ["\u{FD06}", "", "", "\u{FD22}"]
    ),
    ARABIC_LIGATURE_SALLA_USED_AS_KORANIC_STOP_SIGN => (
        &["\u{0635}\u{0644}\u{06D2}"],
        ["\u{FDF0}", "", "", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_ALEF_MAKSURA => (
        &["\u{0633}\u{0649}"],
        ["\u{FCFB}", "", "", "\u{FD17}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_HAH => (
        &["\u{0633}\u{062D}"],
        ["\u{FC1D}", "\u{FCAE}", "\u{FD35}", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_HAH_WITH_JEEM => (
        &["\u{0633}\u{062D}\u{062C}"],
        ["", "\u{FD5C}", "", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_HEH => (
        &["\u{0633}\u{0647}"],
        ["", "\u{FD31}", "\u{FCE8}", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_JEEM => (
        &["\u{0633}\u{062C}"],
        ["\u{FC1C}", "\u{FCAD}", "\u{FD34}", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_JEEM_WITH_ALEF_MAKSURA => (
        &["\u{0633}\u{062C}\u{0649}"],
        ["", "", "", "\u{FD5E}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_JEEM_WITH_HAH => (
        &["\u{0633}\u{062C}\u{062D}"],
        ["", "\u{FD5D}", "", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_KHAH => (
        &["\u{0633}\u{062E}"],
        ["\u{FC1E}", "\u{FCAF}", "\u{FD36}", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_KHAH_WITH_ALEF_MAKSURA => (
        &["\u{0633}\u{062E}\u{0649}"],
        ["", "", "", "\u{FDA8}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_KHAH_WITH_YEH => (
        &["\u{0633}\u{062E}\u{064A}"],
        ["", "", "", "\u{FDC6}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_MEEM => (
        &["\u{0633}\u{0645}"],
        ["\u{FC1F}", "\u{FCB0}", "\u{FCE7}", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_MEEM_WITH_HAH => (
        &["\u{0633}\u{0645}\u{062D}"],
        ["", "\u{FD60}", "", "\u{FD5F}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_MEEM_WITH_JEEM => (
        &["\u{0633}\u{0645}\u{062C}"],
        ["", "\u{FD61}", "", ""]
    ),
    ARABIC_LIGATURE_SEEN_WITH_MEEM_WITH_MEEM => (
        &["\u{0633}\u{0645}\u{0645}"],
        ["", "\u{FD63}", "", "\u{FD62}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_REH => (
        &["\u{0633}\u{0631}"],
        ["\u{FD0E}", "", "", "\u{FD2A}"]
    ),
    ARABIC_LIGATURE_SEEN_WITH_YEH => (
        &["\u{0633}\u{064A}"],
        ["\u{FCFC}", "", "", "\u{FD18}"]
    ),

    // Arabic ligatures with Shadda, the order of characters doesn't matter
    ARABIC_LIGATURE_SHADDA_WITH_DAMMATAN_ISOLATED_FORM => (
        &["\u{064C}\u{0651}", "\u{0651}\u{064C}"],
        ["\u{FC5E}", "\u{FC5E}", "\u{FC5E}", "\u{FC5E}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_KASRATAN_ISOLATED_FORM => (
        &["\u{064D}\u{0651}", "\u{0651}\u{064D}"],
        ["\u{FC5F}", "\u{FC5F}", "\u{FC5F}", "\u{FC5F}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_FATHA_ISOLATED_FORM => (
        &["\u{064E}\u{0651}", "\u{0651}\u{064E}"],
        ["\u{FC60}", "\u{FC60}", "\u{FC60}", "\u{FC60}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_DAMMA_ISOLATED_FORM => (
        &["\u{064F}\u{0651}", "\u{0651}\u{064F}"],
        ["\u{FC61}", "\u{FC61}", "\u{FC61}", "\u{FC61}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_KASRA_ISOLATED_FORM => (
        &["\u{0650}\u{0651}", "\u{0651}\u{0650}"],
        ["\u{FC62}", "\u{FC62}", "\u{FC62}", "\u{FC62}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_SUPERSCRIPT_ALEF => (
        &["\u{0651}\u{0670}", "\u{0670}\u{0651}"],
        ["\u{FC63}", "", "", ""]
    ),

    // There is a special case when they are with Tatweel
    ARABIC_LIGATURE_SHADDA_WITH_FATHA_MEDIAL_FORM => (
        &["\u{0640}\u{064E}\u{0651}", "\u{0640}\u{0651}\u{064E}"],
        ["\u{FCF2}", "\u{FCF2}", "\u{FCF2}", "\u{FCF2}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_DAMMA_MEDIAL_FORM => (
        &["\u{0640}\u{064F}\u{0651}", "\u{0640}\u{0651}\u{064F}"],
        ["\u{FCF3}", "\u{FCF3}", "\u{FCF3}", "\u{FCF3}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_KASRA_MEDIAL_FORM => (
        &["\u{0640}\u{0650}\u{0651}", "\u{0640}\u{0651}\u{0650}"],
        ["\u{FCF4}", "\u{FCF4}", "\u{FCF4}", "\u{FCF4}"]
    ),

    // Repeated with different keys to be backward compatible
    ARABIC_LIGATURE_SHADDA_WITH_FATHA => (
        &["\u{0640}\u{064E}\u{0651}", "\u{0640}\u{0651}\u{064E}"],
        ["\u{FCF2}", "\u{FCF2}", "\u{FCF2}", "\u{FCF2}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_DAMMA => (
        &["\u{0640}\u{064F}\u{0651}", "\u{0640}\u{0651}\u{064F}"],
        ["\u{FCF3}", "\u{FCF3}", "\u{FCF3}", "\u{FCF3}"]
    ),
    ARABIC_LIGATURE_SHADDA_WITH_KASRA => (
        &["\u{0640}\u{0650}\u{0651}", "\u{0640}\u{0651}\u{0650}"],
        ["\u{FCF4}", "\u{FCF4}", "\u{FCF4}", "\u{FCF4}"]
    ),

    ARABIC_LIGATURE_SHEEN_WITH_ALEF_MAKSURA => (
        &["\u{0634}\u{0649}"],
        ["\u{FCFD}", "", "", "\u{FD19}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_HAH => (
        &["\u{0634}\u{062D}"],
        ["\u{FD0A}", "\u{FD2E}", "\u{FD38}", "\u{FD26}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_HAH_WITH_MEEM => (
        &["\u{0634}\u{062D}\u{0645}"],
        ["", "\u{FD68}", "", "\u{FD67}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_HAH_WITH_YEH => (
        &["\u{0634}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDAA}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_HEH => (
        &["\u{0634}\u{0647}"],
        ["", "\u{FD32}", "\u{FCEA}", ""]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_JEEM => (
        &["\u{0634}\u{062C}"],
        ["\u{FD09}", "\u{FD2D}", "\u{FD37}", "\u{FD25}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_JEEM_WITH_YEH => (
        &["\u{0634}\u{062C}\u{064A}"],
        ["", "", "", "\u{FD69}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_KHAH => (
        &["\u{0634}\u{062E}"],
        ["\u{FD0B}", "\u{FD2F}", "\u{FD39}", "\u{FD27}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_MEEM => (
        &["\u{0634}\u{0645}"],
        ["\u{FD0C}", "\u{FD30}", "\u{FCE9}", "\u{FD28}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_MEEM_WITH_KHAH => (
        &["\u{0634}\u{0645}\u{062E}"],
        ["", "\u{FD6B}", "", "\u{FD6A}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_MEEM_WITH_MEEM => (
        &["\u{0634}\u{0645}\u{0645}"],
        ["", "\u{FD6D}", "", "\u{FD6C}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_REH => (
        &["\u{0634}\u{0631}"],
        ["\u{FD0D}", "", "", "\u{FD29}"]
    ),
    ARABIC_LIGATURE_SHEEN_WITH_YEH => (
        &["\u{0634}\u{064A}"],
        ["\u{FCFE}", "", "", "\u{FD1A}"]
    ),
    ARABIC_LIGATURE_TAH_WITH_ALEF_MAKSURA => (
        &["\u{0637}\u{0649}"],
        ["\u{FCF5}", "", "", "\u{FD11}"]
    ),
    ARABIC_LIGATURE_TAH_WITH_HAH => (
        &["\u{0637}\u{062D}"],
        ["\u{FC26}", "\u{FCB8}", "", ""]
    ),
    ARABIC_LIGATURE_TAH_WITH_MEEM => (
        &["\u{0637}\u{0645}"],
        ["\u{FC27}", "\u{FD33}", "\u{FD3A}", ""]
    ),
    ARABIC_LIGATURE_TAH_WITH_MEEM_WITH_HAH => (
        &["\u{0637}\u{0645}\u{062D}"],
        ["", "\u{FD72}", "", "\u{FD71}"]
    ),
    ARABIC_LIGATURE_TAH_WITH_MEEM_WITH_MEEM => (
        &["\u{0637}\u{0645}\u{0645}"],
        ["", "\u{FD73}", "", ""]
    ),
    ARABIC_LIGATURE_TAH_WITH_MEEM_WITH_YEH => (
        &["\u{0637}\u{0645}\u{064A}"],
        ["", "", "", "\u{FD74}"]
    ),
    ARABIC_LIGATURE_TAH_WITH_YEH => (
        &["\u{0637}\u{064A}"],
        ["\u{FCF6}", "", "", "\u{FD12}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_ALEF_MAKSURA => (
        &["\u{062A}\u{0649}"],
        ["\u{FC0F}", "", "", "\u{FC74}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_HAH => (
        &["\u{062A}\u{062D}"],
        ["\u{FC0C}", "\u{FCA2}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_HAH_WITH_JEEM => (
        &["\u{062A}\u{062D}\u{062C}"],
        ["", "\u{FD52}", "", "\u{FD51}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_HAH_WITH_MEEM => (
        &["\u{062A}\u{062D}\u{0645}"],
        ["", "\u{FD53}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_HEH => (
        &["\u{062A}\u{0647}"],
        ["", "\u{FCA5}", "\u{FCE4}", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_JEEM => (
        &["\u{062A}\u{062C}"],
        ["\u{FC0B}", "\u{FCA1}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_JEEM_WITH_ALEF_MAKSURA => (
        &["\u{062A}\u{062C}\u{0649}"],
        ["", "", "", "\u{FDA0}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_JEEM_WITH_MEEM => (
        &["\u{062A}\u{062C}\u{0645}"],
        ["", "\u{FD50}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_JEEM_WITH_YEH => (
        &["\u{062A}\u{062C}\u{064A}"],
        ["", "", "", "\u{FD9F}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_KHAH => (
        &["\u{062A}\u{062E}"],
        ["\u{FC0D}", "\u{FCA3}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_KHAH_WITH_ALEF_MAKSURA => (
        &["\u{062A}\u{062E}\u{0649}"],
        ["", "", "", "\u{FDA2}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_KHAH_WITH_MEEM => (
        &["\u{062A}\u{062E}\u{0645}"],
        ["", "\u{FD54}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_KHAH_WITH_YEH => (
        &["\u{062A}\u{062E}\u{064A}"],
        ["", "", "", "\u{FDA1}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM => (
        &["\u{062A}\u{0645}"],
        ["\u{FC0E}", "\u{FCA4}", "\u{FCE3}", "\u{FC72}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM_WITH_ALEF_MAKSURA => (
        &["\u{062A}\u{0645}\u{0649}"],
        ["", "", "", "\u{FDA4}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM_WITH_HAH => (
        &["\u{062A}\u{0645}\u{062D}"],
        ["", "\u{FD56}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM_WITH_JEEM => (
        &["\u{062A}\u{0645}\u{062C}"],
        ["", "\u{FD55}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM_WITH_KHAH => (
        &["\u{062A}\u{0645}\u{062E}"],
        ["", "\u{FD57}", "", ""]
    ),
    ARABIC_LIGATURE_TEH_WITH_MEEM_WITH_YEH => (
        &["\u{062A}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDA3}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_NOON => (
        &["\u{062A}\u{0646}"],
        ["", "", "", "\u{FC73}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_REH => (
        &["\u{062A}\u{0631}"],
        ["", "", "", "\u{FC70}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_YEH => (
        &["\u{062A}\u{064A}"],
        ["\u{FC10}", "", "", "\u{FC75}"]
    ),
    ARABIC_LIGATURE_TEH_WITH_ZAIN => (
        &["\u{062A}\u{0632}"],
        ["", "", "", "\u{FC71}"]
    ),
    ARABIC_LIGATURE_THAL_WITH_SUPERSCRIPT_ALEF => (
        &["\u{0630}\u{0670}"],
        ["\u{FC5B}", "", "", ""]
    ),
    ARABIC_LIGATURE_THEH_WITH_ALEF_MAKSURA => (
        &["\u{062B}\u{0649}"],
        ["\u{FC13}", "", "", "\u{FC7A}"]
    ),
    ARABIC_LIGATURE_THEH_WITH_HEH => (
        &["\u{062B}\u{0647}"],
        ["", "", "\u{FCE6}", ""]
    ),
    ARABIC_LIGATURE_THEH_WITH_JEEM => (
        &["\u{062B}\u{062C}"],
        ["\u{FC11}", "", "", ""]
    ),
    ARABIC_LIGATURE_THEH_WITH_MEEM => (
        &["\u{062B}\u{0645}"],
        ["\u{FC12}", "\u{FCA6}", "\u{FCE5}", "\u{FC78}"]
    ),
    ARABIC_LIGATURE_THEH_WITH_NOON => (
        &["\u{062B}\u{0646}"],
        ["", "", "", "\u{FC79}"]
    ),
    ARABIC_LIGATURE_THEH_WITH_REH => (
        &["\u{062B}\u{0631}"],
        ["", "", "", "\u{FC76}"]
    ),
    ARABIC_LIGATURE_THEH_WITH_YEH => (
        &["\u{062B}\u{064A}"],
        ["\u{FC14}", "", "", "\u{FC7B}"]
    ),
    ARABIC_LIGATURE_THEH_WITH_ZAIN => (
        &["\u{062B}\u{0632}"],
        ["", "", "", "\u{FC77}"]
    ),
    ARABIC_LIGATURE_UIGHUR_KIRGHIZ_YEH_WITH_HAMZA_ABOVE_WITH_ALEF_MAKSURA => (
        &["\u{0626}\u{0649}"],
        ["\u{FBF9}", "\u{FBFB}", "", "\u{FBFA}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_ALEF_MAKSURA => (
        &["\u{064A}\u{0649}"],
        ["\u{FC59}", "", "", "\u{FC95}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAH => (
        &["\u{064A}\u{062D}"],
        ["\u{FC56}", "\u{FCDB}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAH_WITH_YEH => (
        &["\u{064A}\u{062D}\u{064A}"],
        ["", "", "", "\u{FDAE}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_AE => (
        &["\u{0626}\u{06D5}"],
        ["\u{FBEC}", "", "", "\u{FBED}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_ALEF => (
        &["\u{0626}\u{0627}"],
        ["\u{FBEA}", "", "", "\u{FBEB}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_ALEF_MAKSURA => (
        &["\u{0626}\u{0649}"],
        ["\u{FC03}", "", "", "\u{FC68}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_E => (
        &["\u{0626}\u{06D0}"],
        ["\u{FBF6}", "\u{FBF8}", "", "\u{FBF7}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_HAH => (
        &["\u{0626}\u{062D}"],
        ["\u{FC01}", "\u{FC98}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_HEH => (
        &["\u{0626}\u{0647}"],
        ["", "\u{FC9B}", "\u{FCE0}", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_JEEM => (
        &["\u{0626}\u{062C}"],
        ["\u{FC00}", "\u{FC97}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_KHAH => (
        &["\u{0626}\u{062E}"],
        ["", "\u{FC99}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_MEEM => (
        &["\u{0626}\u{0645}"],
        ["\u{FC02}", "\u{FC9A}", "\u{FCDF}", "\u{FC66}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_NOON => (
        &["\u{0626}\u{0646}"],
        ["", "", "", "\u{FC67}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_OE => (
        &["\u{0626}\u{06C6}"],
        ["\u{FBF2}", "", "", "\u{FBF3}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_REH => (
        &["\u{0626}\u{0631}"],
        ["", "", "", "\u{FC64}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_U => (
        &["\u{0626}\u{06C7}"],
        ["\u{FBF0}", "", "", "\u{FBF1}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_WAW => (
        &["\u{0626}\u{0648}"],
        ["\u{FBEE}", "", "", "\u{FBEF}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_YEH => (
        &["\u{0626}\u{064A}"],
        ["\u{FC04}", "", "", "\u{FC69}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_YU => (
        &["\u{0626}\u{06C8}"],
        ["\u{FBF4}", "", "", "\u{FBF5}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HAMZA_ABOVE_WITH_ZAIN => (
        &["\u{0626}\u{0632}"],
        ["", "", "", "\u{FC65}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_HEH => (
        &["\u{064A}\u{0647}"],
        ["", "\u{FCDE}", "\u{FCF1}", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_JEEM => (
        &["\u{064A}\u{062C}"],
        ["\u{FC55}", "\u{FCDA}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_JEEM_WITH_YEH => (
        &["\u{064A}\u{062C}\u{064A}"],
        ["", "", "", "\u{FDAF}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_KHAH => (
        &["\u{064A}\u{062E}"],
        ["\u{FC57}", "\u{FCDC}", "", ""]
    ),
    ARABIC_LIGATURE_YEH_WITH_MEEM => (
        &["\u{064A}\u{0645}"],
        ["\u{FC58}", "\u{FCDD}", "\u{FCF0}", "\u{FC93}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_MEEM_WITH_MEEM => (
        &["\u{064A}\u{0645}\u{0645}"],
        ["", "\u{FD9D}", "", "\u{FD9C}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_MEEM_WITH_YEH => (
        &["\u{064A}\u{0645}\u{064A}"],
        ["", "", "", "\u{FDB0}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_NOON => (
        &["\u{064A}\u{0646}"],
        ["", "", "", "\u{FC94}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_REH => (
        &["\u{064A}\u{0631}"],
        ["", "", "", "\u{FC91}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_YEH => (
        &["\u{064A}\u{064A}"],
        ["\u{FC5A}", "", "", "\u{FC96}"]
    ),
    ARABIC_LIGATURE_YEH_WITH_ZAIN => (
        &["\u{064A}\u{0632}"],
        ["", "", "", "\u{FC92}"]
    ),
    ARABIC_LIGATURE_ZAH_WITH_MEEM => (
        &["\u{0638}\u{0645}"],
        ["\u{FC28}", "\u{FCB9}", "\u{FD3B}", ""]
    ),
];
