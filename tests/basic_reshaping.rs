use ar_reshaper::{config::LigaturesFlags, ArabicReshaper, Language, ReshaperConfig};

// we can't create a reshaper fully const!
const RESHAPER: ArabicReshaper = ArabicReshaper::new(ReshaperConfig::new(
    Language::Arabic,
    LigaturesFlags::default(),
));

#[test]
fn need_reshape() {
    let cases = [
        ("سلام", true),
        ("خوبی؟", true),
        ("Yeah, Im good", false),
        ("How about you?", false),
    ];

    for (text, neeed_reshape) in cases {
        assert_eq!(RESHAPER.need_reshape(text), neeed_reshape);
    }
}

#[test]
fn default_reshaping() {
    let cases = [
        ("چۆمان", "ﭼﯚﻣﺎﻥ"),
        ("گۆیژە", "ﮔﯚﯾﮋە"),
        ("ﺧﯚﻣﺎﻥ ﺧﯚﺵ", "ﺧﯚﻣﺎﻥ ﺧﯚﺵ"),
    ];

    for (before, after) in cases {
        assert_eq!(RESHAPER.reshape(before), after);
    }
}
