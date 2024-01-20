# Arabic Reshaper Rust

[![Latest version](https://img.shields.io/crates/v/ar-reshaper)](https://crates.io/crates/ar-reshaper)
[![Documentation](https://docs.rs/ar-reshaper/badge.svg)](https://docs.rs/ar-reshaper)
[![Build Status](https://github.com/YouKnow-sys/ar-reshaper/actions/workflows/rust.yml/badge.svg)](https://github.com/YouKnow-sys/ar-reshaper/actions?workflow=Rust%20CI)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YouKnow-sys/ar-reshaper/blob/master/LICENSE)

A **no-std** crate to reconstruct Arabic, Turkish and Persian sentences to be used in applications that don't support Arabic script.

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
use ar_reshaper::ArabicReshaper;

let reshaper = ArabicReshaper::default();

let lines = [
    "سلام",
    "سلام، خوبی؟",
];

println!("{:#?}", reshaper.reshape_lines(lines));
// or you can just use reshape method in a loop... the choice is yours.
```

reshape strings on a iterator

```rust
use ar_reshaper::prelude::*;

for line in ["یک", "دو"].iter().reshape_default() {
    println!("{line}");
}
```

You can check [**examples**](https://github.com/YouKnow-sys/ar-reshaper/examples) or [**tests**](https://github.com/YouKnow-sys/ar-reshaper/tests) directory for more examples.

# features:

- **serde**: if this feature is enabled the `ReshaperConfig` can be serialized and de-serialized using serde.
- **ttf-parser**: if you enable this feature the `ReshaperConfig` method will have another extra
  method named `from_font` that can be used to enable ligatures only if they exist in the input font.

## notes:

- keep in mind that this crate need a allocator to work, because we depend on `alloc` internally.

## Credits:

this project is based on the awesome [`python-arabic-reshaper`](https://github.com/mpcabd/python-arabic-reshaper).
