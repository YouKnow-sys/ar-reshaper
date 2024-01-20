use crate::{form::LettersType, ligatures::*, ArabicReshaper};

/// Flags to enable some or all groups of ligatures
#[derive(Debug, Clone, Copy)]
pub struct LigaturesFlags {
    #[cfg_attr(
        feature = "ttf-parser",
        doc = "We dont check and enable default ligatures when loading ligatures from font"
    )]
    pub default_ligatures: bool,
    pub sentences_ligatures: bool,
    pub words_ligatures: bool,
    pub letters_ligatures: bool,
}

impl LigaturesFlags {
    /// Enable just some of defaults ligatures
    pub const fn default() -> Self {
        Self {
            default_ligatures: true,
            sentences_ligatures: false,
            words_ligatures: false,
            letters_ligatures: false,
        }
    }

    /// Enable all ligatures
    pub const fn all() -> Self {
        Self {
            default_ligatures: false,
            sentences_ligatures: true,
            words_ligatures: true,
            letters_ligatures: true,
        }
    }

    /// Disable all ligatures
    pub const fn none() -> Self {
        Self {
            default_ligatures: false,
            sentences_ligatures: false,
            words_ligatures: false,
            letters_ligatures: false,
        }
    }

    /// Check if no ligature is enabled
    pub const fn is_none_enabled(&self) -> bool {
        !self.sentences_ligatures && !self.words_ligatures && !self.letters_ligatures
    }
}

/// Supported languages
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    /// `Arabic` is default and recommended to work in most of the cases
    #[default]
    Arabic,
    /// `ArabicV2` is only to be used with certain font that you run into missing chars
    ArabicV2,
    /// `Kurdish` if you are using Kurdish Sarchia font is recommended,
    /// work with both unicode and classic Arabic-Kurdish keybouard
    Kurdish,
    /// Custom language
    #[cfg_attr(feature = "serde", serde(skip))] // we can't serialize this
    Custom(&'static [LettersType]),
}

impl core::fmt::Display for Language {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Language::Arabic => "Arabic",
            Language::ArabicV2 => "ArabicV2",
            Language::Kurdish => "Kurdish",
            Language::Custom(_) => "Custom",
        }
        .fmt(f)
    }
}

/// Hold state of whatever some ligatures are enabled or not.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ligatures {
    #[cfg_attr(feature = "serde", serde(with = "arrays"))]
    pub(crate) list: [bool; LIGATURES.len()],
}

impl Ligatures {
    /// Create a new [Ligatures] with all the ligatures disabled
    pub const fn empty() -> Self {
        Self {
            list: [false; LIGATURES.len()],
        }
    }

    /// Enable some default ligatures
    pub const fn default() -> Self {
        let mut ligatures = Self::empty();

        ligatures.list[LigatureNames::ARABIC_LIGATURE_ALLAH as usize] = true;
        ligatures.list[LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF as usize] = true;
        ligatures.list[LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_ABOVE as usize] =
            true;
        ligatures.list[LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_BELOW as usize] =
            true;
        ligatures.list[LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_MADDA_ABOVE as usize] =
            true;

        ligatures
    }

    /// Is any ligature enabled
    const fn is_any_enabled(&self) -> bool {
        let mut idx = 0;
        while idx < self.list.len() {
            if self.list[idx] {
                return true;
            }
            idx += 1;
        }
        false
    }

    /// Is the input ligature enabled
    pub const fn is_ligature_enabled(&self, name: LigatureNames) -> bool {
        self.list[name as usize]
    }
}

/// The main Config struct for the [ArabicReshaper]
///
/// You can change all kinds of settings about [ArabicReshaper] using this struct.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReshaperConfig {
    /// Supported languages are: **Arabic, ArabicV2, Kurdish**
    /// More languages might be supported soon.
    pub language: Language,
    /// Whether to delete the Harakat (Tashkeel) before reshaping or not.
    pub delete_harakat: bool,
    /// Whether to shift the Harakat (Tashkeel) one position so they appear
    /// correctly when string is reversed
    pub shift_harakat_position: bool,
    /// Whether to delete the Tatweel (U+0640) before reshaping or not.
    pub delete_tatweel: bool,
    /// Whether to support ZWJ (U+200D) or not.
    pub support_zwj: bool,
    /// Use unshaped form instead of isolated form.
    pub use_unshaped_instead_of_isolated: bool,
    /// Whether to use ligatures or not.
    /// Serves as a shortcut to disable all ligatures.
    pub support_ligatures: bool,
    /// When `support_ligatures` is enabled.
    /// Separate ligatures configuration take precedence over it.
    /// When `support_ligatures` is disabled,
    /// separate ligatures configurations are ignored.
    pub ligatures: Ligatures,
}

impl Default for ReshaperConfig {
    fn default() -> Self {
        let mut ligatures = Ligatures::empty();
        // enable some default ones
        for name in [
            LigatureNames::ARABIC_LIGATURE_ALLAH,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_ABOVE,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_BELOW,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_MADDA_ABOVE,
        ] {
            ligatures.list[name as usize] = true;
        }

        Self {
            language: Default::default(),
            delete_harakat: true,
            shift_harakat_position: false,
            delete_tatweel: false,
            support_zwj: true,
            use_unshaped_instead_of_isolated: false,
            support_ligatures: true,
            ligatures,
        }
    }
}

impl ReshaperConfig {
    /// Create a new [ReshaperConfig] with the given [LigaturesFlags].
    pub const fn new(language: Language, ligatures_flags: LigaturesFlags) -> Self {
        let mut ligatures = if ligatures_flags.default_ligatures {
            Ligatures::default()
        } else {
            Ligatures::empty()
        };

        if !ligatures_flags.is_none_enabled() {
            let LigaturesFlags {
                sentences_ligatures,
                words_ligatures,
                letters_ligatures,
                ..
            } = ligatures_flags;

            macro_rules! enable_ligatures {
                ($range:expr) => {{
                    let mut idx = $range.start;
                    while idx < $range.end {
                        ligatures.list[idx] = true;
                        idx += 1;
                    }
                }};
            }

            if sentences_ligatures {
                enable_ligatures!(SENTENCES_LIGATURES_RANGE)
            }
            if words_ligatures {
                enable_ligatures!(WORDS_LIGATURES_RANGE)
            }
            if letters_ligatures {
                enable_ligatures!(LETTERS_LIGATURES_RANGE)
            }
        }

        Self {
            language,
            support_ligatures: !ligatures_flags.is_none_enabled(),
            ligatures,
            delete_harakat: true,
            shift_harakat_position: false,
            delete_tatweel: false,
            support_zwj: true,
            use_unshaped_instead_of_isolated: false,
        }
    }

    /// Create a new [ArabicReshaper] from the config.
    pub fn to_reshaper(self) -> ArabicReshaper {
        ArabicReshaper::new(self)
    }

    /// Create a new [ReshaperConfig] based on the input **true type font** font.\
    /// Keep in mind that we are currently using `ttf-parser` crate for parsing ttf
    /// files, this crate doesn't support cmap8, this may change in future.
    // This function need more testing, I haven't tested well yet.
    #[cfg(feature = "ttf-parser")]
    pub fn from_font(
        bytes: &[u8],
        language: Language,
        ligatures_flags: LigaturesFlags,
    ) -> Result<Self, alloc::string::String> {
        use crate::{form::Forms, letters::Letters};
        use alloc::{string::ToString, vec::Vec};
        use ttf_parser::Face;

        let font = Face::parse(bytes, 0).map_err(|e| e.to_string())?;

        let mut config = Self {
            support_ligatures: !ligatures_flags.is_none_enabled(),
            ..Default::default()
        };

        if let Some(tables) = font.tables().cmap {
            'top: for (_, v) in Letters::new(language).0 {
                for table in tables.subtables {
                    if v.isolated != '\0' && table.glyph_index(v.isolated as _).is_some() {
                        continue 'top;
                    }
                }
                config.use_unshaped_instead_of_isolated = true;
                break;
            }

            let mut process_ligatures = |ligatures: &[(&'static [&'static str], Forms)]| {
                for (idx, (_, forms)) in ligatures.iter().enumerate() {
                    let forms: Vec<_> = [forms.isolated, forms.initial, forms.medial, forms.end]
                        .into_iter()
                        .filter(|c| *c != '\0')
                        .collect();
                    let mut n = forms.len();
                    for form in forms {
                        for table in tables.subtables {
                            // we are filtering empty string, so it should be ok to just unwrap
                            if table.glyph_index(form as _).is_some() {
                                n -= 1;
                                break;
                            }
                        }
                    }
                    config.ligatures.list[idx] = n == 0;
                }
            };

            if !ligatures_flags.is_none_enabled() {
                let LigaturesFlags {
                    sentences_ligatures,
                    words_ligatures,
                    letters_ligatures,
                    ..
                } = ligatures_flags;

                for (enabled, range) in [
                    (sentences_ligatures, SENTENCES_LIGATURES_RANGE),
                    (words_ligatures, WORDS_LIGATURES_RANGE),
                    (letters_ligatures, LETTERS_LIGATURES_RANGE),
                ] {
                    if enabled {
                        process_ligatures(&LIGATURES[range]);
                    }
                }
            }
        } else {
            config.use_unshaped_instead_of_isolated = true;
            // Im not sure what should I do here, but the best thing that I can think of right now is
            // to disable all the ligatures
            config
                .ligatures
                .list
                .iter_mut()
                .for_each(|enabled| *enabled = false);
        }

        Ok(config)
    }

    /// Update the given [LigatureNames].
    pub fn update_ligature(&mut self, name: LigatureNames, enable: bool) {
        self.ligatures.list[name as usize] = enable;
        // enable or disable ligatures if anything is enabled
        self.support_ligatures = self.ligatures.is_any_enabled();
    }
}

/// A simple hack for serialize and deserialize arrays that are bigger then 32.
/// we have to use this because serde dont have support for const generic in array size...
#[cfg(feature = "serde")]
mod arrays {
    use alloc::vec::Vec;
    use core::{convert::TryInto, marker::PhantomData};

    use serde::{
        de::{SeqAccess, Visitor},
        ser::SerializeTuple,
        Deserialize, Deserializer, Serialize, Serializer,
    };
    pub fn serialize<S: Serializer, T: Serialize, const N: usize>(
        data: &[T; N],
        ser: S,
    ) -> Result<S::Ok, S::Error> {
        let mut s = ser.serialize_tuple(N)?;
        for item in data {
            s.serialize_element(item)?;
        }
        s.end()
    }

    struct ArrayVisitor<T, const N: usize>(PhantomData<T>);

    impl<'de, T, const N: usize> Visitor<'de> for ArrayVisitor<T, N>
    where
        T: Deserialize<'de>,
    {
        type Value = [T; N];

        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
            formatter.write_str(&alloc::format!("an array of length {}", N))
        }

        #[inline]
        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // can be optimized using MaybeUninit
            let mut data = Vec::with_capacity(N);
            for _ in 0..N {
                match (seq.next_element())? {
                    Some(val) => data.push(val),
                    None => return Err(serde::de::Error::invalid_length(N, &self)),
                }
            }
            match data.try_into() {
                Ok(arr) => Ok(arr),
                Err(_) => unreachable!(),
            }
        }
    }
    pub fn deserialize<'de, D, T, const N: usize>(deserializer: D) -> Result<[T; N], D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        deserializer.deserialize_tuple(N, ArrayVisitor::<T, N>(PhantomData))
    }
}
