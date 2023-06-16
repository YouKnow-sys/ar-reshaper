use crate::{ArabicReshaper, ReshaperConfig};

#[test]
fn default_persian_reshaping() {
    let mut reshaper = ArabicReshaper::default();

    reshaper.modify_config(|c| {
        c.delete_harakat = false;
    });

    let cases = [
        ("سلام، حالت چطوره؟", "ﺳﻼﻡ، ﺣﺎﻟﺖ ﭼﻄﻮﺭﻩ؟"),
        ("مَمنون، عالیَم", "ﻣَﻤﻨﻮﻥ، ﻋﺎﻟﯿَﻢ"),
        ("نَظَرِت راجِبه راست چیه؟", "ﻧَﻈَﺮِﺕ ﺭﺍﺟِﺒﻪ ﺭﺍﺳﺖ ﭼﯿﻪ؟"),
        ("عاشِقِشَم!", "ﻋﺎﺷِﻘِﺸَﻢ!"),
        ("", ""), // test empty input
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}

#[test]
fn default_reshaping() {
    let reshaper = ArabicReshaper::default();
    let cases = [
        ("السلام عليكم", "ﺍﻟﺴﻼﻡ ﻋﻠﻴﻜﻢ"),
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴﻼﻡ ﻋﻠﻴﻜﻢ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎ ﻭﻧﻄﻘﺎ ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}

#[test]
fn zwj_reshaping() {
    use crate::letters::{FINAL, INITIAL, ISOLATED, LETTERS_ARABIC, MEDIAL, ZWJ};

    let reshaper = ArabicReshaper::default();

    const BEH: char = 'ب';
    let beh_isolated =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &BEH).unwrap().1[ISOLATED as usize].to_owned();
    let beh_initial =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &BEH).unwrap().1[INITIAL as usize].to_owned();
    let beh_medial =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &BEH).unwrap().1[MEDIAL as usize].to_owned();
    let beh_final =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &BEH).unwrap().1[FINAL as usize].to_owned();

    const ALEF: char = 'ا';
    let alef_final =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &ALEF).unwrap().1[FINAL as usize].to_owned();

    const HAMZA: char = 'ء';
    let hamza_isolated =
        LETTERS_ARABIC.iter().find(|(k, _)| k == &HAMZA).unwrap().1[ISOLATED as usize].to_owned();

    let zwj = ZWJ.to_string();
    let alef = ALEF.to_string();
    let beh = BEH.to_string();
    let hamza = HAMZA.to_string();

    let cases = [
        (beh.clone() + &hamza, beh_isolated.clone() + &hamza_isolated),
        (
            zwj.clone() + &beh + &hamza,
            beh_final.clone() + &hamza_isolated,
        ),
        (zwj.clone() + &beh, beh_final.clone()),
        (beh.clone() + &zwj, beh_initial.clone()),
        (zwj.clone() + &beh + &zwj, beh_medial),
        (
            beh.clone() + &zwj + &hamza,
            beh_initial.clone() + &hamza_isolated,
        ),
        (beh.clone() + &alef, beh_initial.clone() + &alef_final),
        (
            beh.clone() + &zwj + &alef,
            beh_initial.clone() + &alef_final,
        ),
        (
            beh.clone() + &zwj + &alef + &zwj,
            beh_initial.clone() + &alef_final,
        ),
        (
            beh.clone() + &alef + &beh,
            beh_initial.clone() + &alef_final + &beh_isolated,
        ),
        (
            beh.clone() + &zwj + &alef + &zwj + &beh,
            beh_initial.clone() + &alef_final + &beh_final,
        ),
        (
            beh.clone() + &zwj + &hamza + &beh,
            beh_initial.clone() + &hamza_isolated + &beh_isolated,
        ),
        (
            beh.clone() + &zwj + &hamza + &zwj + &beh,
            beh_initial + &hamza_isolated + &beh_final,
        ),
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}

#[test]
fn reshaping_with_harakat() {
    let config = ReshaperConfig {
        delete_harakat: false,
        ..Default::default()
    };

    let reshaper = ArabicReshaper::new(config);

    let cases = [
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴَﻼَْﻡٌ ﻋَﻠَﻴْﻜُﻢْ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎً ﻭﻧﻄﻘﺎً ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳُﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻷﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}

#[test]
fn reshaping_with_harakat_without_ligatures() {
    let config = ReshaperConfig {
        delete_harakat: false,
        support_ligatures: false,
        ..Default::default()
    };

    let reshaper = ArabicReshaper::new(config);

    let cases = [
        ("السَلَاْمٌ عَلَيْكُمْ", "ﺍﻟﺴَﻠَﺎْﻡٌ ﻋَﻠَﻴْﻜُﻢْ"),
        (
            "اللغة العربية هي أكثر اللغات",
            "ﺍﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ ﻫﻲ ﺃﻛﺜﺮ ﺍﻟﻠﻐﺎﺕ",
        ),
        ("تحدثاً ونطقاً ضمن مجموعة", "ﺗﺤﺪﺛﺎً ﻭﻧﻄﻘﺎً ﺿﻤﻦ ﻣﺠﻤﻮﻋﺔ"),
        ("اللغات السامية", "ﺍﻟﻠﻐﺎﺕ ﺍﻟﺴﺎﻣﻴﺔ"),
        ("العربية لغة رسمية في", "ﺍﻟﻌﺮﺑﻴﺔ ﻟﻐﺔ ﺭﺳﻤﻴﺔ ﻓﻲ"),
        ("كل دول الوطن العربي", "ﻛﻞ ﺩﻭﻝ ﺍﻟﻮﻃﻦ ﺍﻟﻌﺮﺑﻲ"),
        ("إضافة إلى كونها لغة", "ﺇﺿﺎﻓﺔ ﺇﻟﻰ ﻛﻮﻧﻬﺎ ﻟﻐﺔ"),
        ("رسمية في تشاد وإريتريا", "ﺭﺳﻤﻴﺔ ﻓﻲ ﺗﺸﺎﺩ ﻭﺇﺭﻳﺘﺮﻳﺎ"),
        ("وإسرائيل. وهي إحدى اللغات", "ﻭﺇﺳﺮﺍﺋﻴﻞ. ﻭﻫﻲ ﺇﺣﺪﻯ ﺍﻟﻠﻐﺎﺕ"),
        ("الرسمية الست في منظمة", "ﺍﻟﺮﺳﻤﻴﺔ ﺍﻟﺴﺖ ﻓﻲ ﻣﻨﻈﻤﺔ"),
        ("الأمم المتحدة، ويُحتفل", "ﺍﻟﺄﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ، ﻭﻳُﺤﺘﻔﻞ"),
        ("باليوم العالمي للغة العربية", "ﺑﺎﻟﻴﻮﻡ ﺍﻟﻌﺎﻟﻤﻲ ﻟﻠﻐﺔ ﺍﻟﻌﺮﺑﻴﺔ"),
        ("في 18 ديسمبر كذكرى اعتماد", "ﻓﻲ 18 ﺩﻳﺴﻤﺒﺮ ﻛﺬﻛﺮﻯ ﺍﻋﺘﻤﺎﺩ"),
        ("العربية بين لغات العمل في", "ﺍﻟﻌﺮﺑﻴﺔ ﺑﻴﻦ ﻟﻐﺎﺕ ﺍﻟﻌﻤﻞ ﻓﻲ"),
        ("الأمم المتحدة.", "ﺍﻟﺄﻣﻢ ﺍﻟﻤﺘﺤﺪﺓ."),
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}

#[test]
fn reshaping_with_shifted_harakat_without_ligatures() {
    let config = ReshaperConfig {
        delete_harakat: false,
        support_ligatures: false,
        shift_harakat_position: true,
        ..Default::default()
    };

    let reshaper = ArabicReshaper::new(config);

    let cases = [("فُعِلَ", "ُﻓِﻌَﻞ"), ("فُعِّلَ", "ُﻓِّﻌَﻞ")];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}
