use crate::ligatures::*;

bitflags::bitflags! {
    /// Flags to enable some or all groups of ligatures
    #[derive(Clone, Copy)]
    pub struct LigaturesFlags: u8 {
        const ENABLE_NO_LIGATURES = 0b000;
        const ENABLE_SENTENCES_LIGATURES = 0b001;
        const ENABLE_WORDS_LIGATURES = 0b010;
        const ENABLE_LETTERS_LIGATURES = 0b100;
        const ENABLE_ALL_LIGATURES =
            Self::ENABLE_SENTENCES_LIGATURES.bits()
            | Self::ENABLE_WORDS_LIGATURES.bits()
            | Self::ENABLE_LETTERS_LIGATURES.bits();
    }
}

/// Supported languages
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Language {
    /// `Arabic` is default and recommended to work in most of the cases and
    #[default]
    Arabic,
    /// `ArabicV2` is only to be used with certain font that you run into missing chars
    ArabicV2,
    /// `Kurdish` if you are using Kurdish Sarchia font is recommended,
    /// work with both unicode and classic Arabic-Kurdish keybouard
    Kurdish,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

/// The main Config struct for the [ArabicReshaper]
/// 
/// You can change all kinds of settings about [ArabicReshaper] using this struct.
/// 
/// [ArabicReshaper]: crate::reshaper::ArabicReshaper
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
    pub(crate) ligatures: Vec<bool>,
}

impl Default for ReshaperConfig {
    fn default() -> Self {
        let mut ligatures = vec![false; LIGATURES.len()];
        // enable some default ones
        for name in [
            LigatureNames::ARABIC_LIGATURE_ALLAH,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_ABOVE,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_HAMZA_BELOW,
            LigatureNames::ARABIC_LIGATURE_LAM_WITH_ALEF_WITH_MADDA_ABOVE,
        ] {
            ligatures[name as usize] = true;
        }

        Self {
            language: Default::default(),
            delete_harakat: true,
            shift_harakat_position: false,
            delete_tatweel: false,
            support_zwj: true,
            use_unshaped_instead_of_isolated: false,
            support_ligatures: true,
            // this will enable all the ligatures
            ligatures,
        }
    }
}

impl ReshaperConfig {
    /// Create a new [ReshaperConfig] with the given [LigaturesFlags].
    pub fn new(language: Language, ligatures_flags: LigaturesFlags) -> Self {
        let mut ligatures = vec![false; LIGATURES.len()];

        if !ligatures_flags.contains(LigaturesFlags::ENABLE_NO_LIGATURES) {
            for (flag, range) in [
                (
                    LigaturesFlags::ENABLE_SENTENCES_LIGATURES,
                    SENTENCES_LIGATURES_RANGE,
                ),
                (
                    LigaturesFlags::ENABLE_WORDS_LIGATURES,
                    WORDS_LIGATURES_RANGE,
                ),
                (
                    LigaturesFlags::ENABLE_LETTERS_LIGATURES,
                    LETTERS_LIGATURES_RANGE,
                ),
            ] {
                if ligatures_flags.contains(flag) {
                    ligatures[range]
                        .iter_mut()
                        .for_each(|enabled| *enabled = true);
                }
            }
        }

        Self {
            language,
            support_ligatures: !ligatures_flags.contains(LigaturesFlags::ENABLE_NO_LIGATURES),
            ligatures,
            ..Default::default()
        }
    }

    /// Create a new [ReshaperConfig] based on the input **true type font** font.\
    /// Keep in mind that we are currently using `ttf-parser` crate for parsing ttf
    /// files, this crate doesn't support cmap8, this may change in future.
    #[cfg(feature = "ttf-parser")]
    pub fn from_font(
        bytes: &[u8],
        language: Language,
        ligatures_flags: LigaturesFlags,
    ) -> Result<Self, String> {
        use crate::letters::{Letters, ISOLATED};
        use ttf_parser::Face;

        let font = Face::parse(bytes, 0).map_err(|e| e.to_string())?;

        let mut config = Self {
            support_ligatures: !ligatures_flags.contains(LigaturesFlags::ENABLE_NO_LIGATURES),
            ..Default::default()
        };

        if let Some(tables) = font.tables().cmap {
            'top: for (_, v) in Letters::new(language).0 {
                for table in tables.subtables {
                    if let Some(v) = v[ISOLATED as usize].chars().next() {
                        if table.glyph_index(v as _).is_some() {
                            continue 'top;
                        }
                    }
                }
                config.use_unshaped_instead_of_isolated = true;
                break;
            }

            let mut process_ligatures = |ligatures: &[(&[&str], [&str; 4])]| {
                for (idx, (_, chars)) in ligatures.iter().enumerate() {
                    let forms: Vec<_> = chars.iter().filter(|c| c.is_empty()).collect();
                    let mut n = forms.len();
                    for form in forms {
                        for table in tables.subtables {
                            // we are filtering empty string, so it should be ok to just unwrap
                            if table
                                .glyph_index(form.chars().next().unwrap() as _)
                                .is_some()
                            {
                                n -= 1;
                                break;
                            }
                        }
                    }
                    config.ligatures[idx] = n == 0;
                }
            };

            if !ligatures_flags.contains(LigaturesFlags::ENABLE_NO_LIGATURES) {
                for (flag, range) in [
                    (
                        LigaturesFlags::ENABLE_SENTENCES_LIGATURES,
                        SENTENCES_LIGATURES_RANGE,
                    ),
                    (
                        LigaturesFlags::ENABLE_WORDS_LIGATURES,
                        WORDS_LIGATURES_RANGE,
                    ),
                    (
                        LigaturesFlags::ENABLE_LETTERS_LIGATURES,
                        LETTERS_LIGATURES_RANGE,
                    ),
                ] {
                    if ligatures_flags.contains(flag) {
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
                .iter_mut()
                .for_each(|enabled| *enabled = false);
        }

        Ok(config)
    }

    /// Update the given [LigatureNames].
    pub fn update_ligature(&mut self, key: LigatureNames, enable: bool) {
        self.ligatures[key as usize] = enable;
        // enable or disable ligatures if anything is enabled
        self.support_ligatures = self.ligatures.iter().any(|enabled| *enabled);
    }
}
