# Arabic Reshaper Rust
Reconstruct Arabic sentences to be used in applications that don't support Arabic script.

# Usage:
resahpe a single line of string
```rs
use sentences_reshaper::{ArabicReshaper, reshape_line};

let reshaper = ArabicReshaper::default();

// You can reshape just a single string using
println!("{}", reshaper.reshape("سلام دنیا"));
// or using `reshape_line` method if you dont want to construct the [ArabicReshaper]
// and you just want to reshape a few strings with default config
println!("{}", reshape_line("سلام دنیا"));
// Both will reconstruct the string and print `ﺳﻼﻡ ﺩﻧﯿﺎ`
```
reshape a slice of strings
```rs
use sentences_reshaper::{ArabicReshaper}

let reshaper = ArabicReshaper::default();

let lines = [
    "سلام",
    "سلام، خوبی؟",
];

println!("{:#?}", reshaper.reshape_lines(lines));
// or you can just use reshape method in a loop... the choice is yours.
```
You can check **example** or **test** directory for more examples.

# Optional features:
- **config_from_font**: if you enable this feature the `ReshaperConfig` method will have another extra
method named `from_font` that can be used to enable ligatures only if they exist in the input font.

## Credits:
this project is based on the awesome `python-arabic-reshaper`.
you can check original project in [this](https://github.com/mpcabd/python-arabic-reshaper) repository