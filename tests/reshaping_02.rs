use ar_reshaper::{ArabicReshaper, ReshaperConfig};

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
    use ar_reshaper::letters::letters_db::{LETTERS_ARABIC, ZWJ};

    let reshaper = ArabicReshaper::default();

    const BEH: char = 'ب';
    let beh_isolated = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &BEH)
        .unwrap()
        .1
        .isolated;
    let beh_initial = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &BEH)
        .unwrap()
        .1
        .initial;
    let beh_medial = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &BEH)
        .unwrap()
        .1
        .medial;
    let beh_final = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &BEH)
        .unwrap()
        .1
        .end;

    const ALEF: char = 'ا';
    let alef_final = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &ALEF)
        .unwrap()
        .1
        .end;

    const HAMZA: char = 'ء';
    let hamza_isolated = LETTERS_ARABIC
        .iter()
        .find(|(k, _)| k == &HAMZA)
        .unwrap()
        .1
        .isolated;

    let cases = [
        (
            format!("{BEH}{HAMZA}"),
            format!("{beh_isolated}{hamza_isolated}"),
        ),
        (
            format!("{ZWJ}{BEH}{HAMZA}"),
            format!("{beh_final}{hamza_isolated}"),
        ),
        (format!("{ZWJ}{BEH}"), beh_final.to_string()),
        (format!("{BEH}{ZWJ}"), beh_initial.to_string()),
        (format!("{ZWJ}{BEH}{ZWJ}"), beh_medial.to_string()),
        (
            format!("{BEH}{ZWJ}{HAMZA}"),
            format!("{beh_initial}{hamza_isolated}"),
        ),
        (format!("{BEH}{ALEF}"), format!("{beh_initial}{alef_final}")),
        (
            format!("{BEH}{ZWJ}{ALEF}"),
            format!("{beh_initial}{alef_final}"),
        ),
        (
            format!("{BEH}{ZWJ}{ALEF}{ZWJ}"),
            format!("{beh_initial}{alef_final}"),
        ),
        (
            format!("{BEH}{ALEF}{BEH}"),
            format!("{beh_initial}{alef_final}{beh_isolated}"),
        ),
        (
            format!("{BEH}{ZWJ}{ALEF}{ZWJ}{BEH}"),
            format!("{beh_initial}{alef_final}{beh_final}"),
        ),
        (
            format!("{BEH}{ZWJ}{HAMZA}{BEH}"),
            format!("{beh_initial}{hamza_isolated}{beh_isolated}"),
        ),
        (
            format!("{BEH}{ZWJ}{HAMZA}{ZWJ}{BEH}"),
            format!("{beh_initial}{hamza_isolated}{beh_final}"),
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
