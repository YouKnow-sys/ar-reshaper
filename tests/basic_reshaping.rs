use ar_reshaper::ArabicReshaper;

#[test]
fn need_reshape() {
    let reshaper = ArabicReshaper::default();

    let cases = [
        ("سلام", true),
        ("خوبی؟", true),
        ("Yeah, Im good", false),
        ("How about you?", false),
    ];

    for (text, neeed_reshape) in cases {
        assert_eq!(reshaper.need_reshape(text), neeed_reshape);
    }
}

#[test]
fn default_reshaping() {
    let reshaper = ArabicReshaper::default();

    let cases = [
        ("چۆمان", "ﭼﯚﻣﺎﻥ"),
        ("گۆیژە", "ﮔﯚﯾﮋە"),
        ("ﺧﯚﻣﺎﻥ ﺧﯚﺵ", "ﺧﯚﻣﺎﻥ ﺧﯚﺵ"),
    ];

    for (before, after) in cases {
        assert_eq!(reshaper.reshape(before), after);
    }
}
