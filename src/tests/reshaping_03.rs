use crate::ArabicReshaper;

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
