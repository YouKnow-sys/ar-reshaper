//! # ArabicReshaper
//! 
//! Reconstruct Arabic sentences to be used in applications that don't support Arabic script.
//! 
//! ## Usage:
//! ```
//! use ar_reshaper::{ArabicReshaper, reshape_line};
//! 
//! let reshaper = ArabicReshaper::default();
//! 
//! // You can reshape just a single string using
//! println!("{}", reshaper.reshape("سلام دنیا"));
//! // or [`reshape_line`] method if you dont want to construct the [ArabicReshaper]
//! // and you just want to reshape a line with default settings
//! println!("{}", reshape_line("سلام دنیا"));
//! // Both will reconstruct the string and print `ﺳﻼﻡ ﺩﻧﯿﺎ`
//! 
//! // Or a slice of strings
//! println!("{:#?}", reshaper.reshape_lines(&["سلام خوبی؟", "عالیم ممنون"]));
//! // this will reconstruct the string and print  ["ﺳﻼﻡ ﺧﻮﺑﯽ؟", "ﻋﺎﻟﯿﻢ ﻣﻤﻨﻮﻥ"]
//! ```
//! 
//! A rusty rewrite of [python-arabic-reshaper](https://github.com/mpcabd/python-arabic-reshaper)
//! You can check the original repository for more information.

pub use config::*;
pub use ligatures::LigatureNames;
pub use reshaper::ArabicReshaper;

mod config;
mod letters;
mod ligatures;
mod reshaper;
#[cfg(test)]
mod tests;

/// Reshape the given text with the default [ArabicReshaper] configuration.\
/// Keep in mind that if you want to reshape a large amount of lines its better
/// to first create a [ArabicReshaper] and use the [`ArabicReshaper::reshape`]
/// instead.
/// 
/// [`reshape`]: ArabicReshaper::reshape
pub fn reshape_line<S>(text: S) -> String
where
    S: AsRef<str>,
{
    let reshaper = ArabicReshaper::default();

    reshaper.reshape(text)
}
