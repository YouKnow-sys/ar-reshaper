# Arabic Reshaper Rust
[![Latest version](https://img.shields.io/crates/v/ar-reshaper)](https://crates.io/crates/ar-reshaper)
[![Build Status](https://github.com/YouKnow-sys/ar-reshaper/actions/workflows/rust.yml/badge.svg)](https://github.com/YouKnow-sys/ar-reshaper/actions?workflow=Rust%20CI)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YouKnow-sys/ar-reshaper/blob/master/LICENSE-MIT)

Reconstruct Arabic sentences to be used in applications that don't support Arabic script.

# Usage:
resahpe a single line of string
```rust
use ar_reshaper::{ArabicReshaper, reshape_line};

let reshaper = ArabicReshaper::default();

// You can reshape just a single string using
println!("{}", reshaper.reshape("سلام دنیا"));
// or using `reshape_line` method if you dont want to construct the [ArabicReshaper]
// and you just want to reshape a few strings with default config
println!("{}", reshape_line("سلام دنیا"));
// Both will reconstruct the string and print `ﺳﻼﻡ ﺩﻧﯿﺎ`
```
reshape a slice of strings
```rust
use ar_reshaper::{ArabicReshaper}

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
- **ttf-parser**: if you enable this feature the `ReshaperConfig` method will have another extra
method named `from_font` that can be used to enable ligatures only if they exist in the input font.

## Credits:
this project is based on the awesome `python-arabic-reshaper`.
you can check original project in [this](https://github.com/mpcabd/python-arabic-reshaper) repository