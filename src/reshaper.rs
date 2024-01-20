use core::ops::RangeInclusive;

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{
    config::ReshaperConfig,
    form::LetterForm,
    letters::{
        letters_db::{TATWEEL, ZWJ},
        *,
    },
    ligatures::*,
};

const EMPTY: (char, LetterForm) = ('\0', LetterForm::Unsupported);

static HARAKAT_RE: [RangeInclusive<char>; 9] = [
    '\u{0610}'..='\u{061a}',
    '\u{064b}'..='\u{065f}',
    '\u{0670}'..='\u{0670}',
    '\u{06d6}'..='\u{06dc}',
    '\u{06df}'..='\u{06e8}',
    '\u{06ea}'..='\u{06ed}',
    '\u{08d4}'..='\u{08e1}',
    '\u{08d4}'..='\u{08ed}',
    '\u{08e3}'..='\u{08ff}',
];

/// # ArabicReshaper
/// the main type for reconstructing sentences to be used in applications that don't support Arabic script.
#[derive(Default, Clone)]
pub struct ArabicReshaper {
    config: ReshaperConfig,
    letters: Letters,
}

impl ArabicReshaper {
    /// Create a new [ArabicReshaper] using the given config
    pub const fn new(config: ReshaperConfig) -> Self {
        Self {
            letters: Letters::new(config.language),
            config,
        }
    }

    /// Check whatever the text need reshaping or not.
    pub fn need_reshape<S>(&self, text: S) -> bool
    where
        S: AsRef<str>,
    {
        text.as_ref().chars().any(|c| self.letters.contains_key(&c))
    }

    /// Reshape the given line and return the reshaped string
    pub fn reshape<S>(&self, text: S) -> String
    where
        S: AsRef<str>,
    {
        let text = text.as_ref();

        if text.is_empty() {
            return String::new();
        }

        let ReshaperConfig {
            delete_harakat,
            shift_harakat_position,
            delete_tatweel,
            support_zwj,
            use_unshaped_instead_of_isolated,
            support_ligatures,
            ..
        } = self.config;

        let isolated_form = match use_unshaped_instead_of_isolated {
            true => LetterForm::Unshaped,
            false => LetterForm::Isolated,
        };

        let mut output = Vec::new();
        let mut position_harakat: BTreeMap<isize, Vec<char>> = BTreeMap::new();

        for letter in text.chars() {
            if HARAKAT_RE.iter().any(|h| h.contains(&letter)) {
                if !delete_harakat {
                    let mut position = (output.len() - 1) as isize;
                    if shift_harakat_position {
                        position -= 1
                    }

                    let entry = position_harakat.entry(position).or_default();

                    if shift_harakat_position {
                        entry.insert(0, letter);
                    } else {
                        entry.push(letter);
                    }
                }
            } else if letter == TATWEEL && delete_tatweel || letter == ZWJ && !support_zwj {
            } else if !self.letters.contains_key(&letter) {
                output.push((letter, LetterForm::Unsupported))
            } else if output.is_empty() {
                output.push((letter, isolated_form)) // first letter
            } else {
                let previous_letter = output.last_mut().unwrap();
                if (previous_letter.1 == LetterForm::Unsupported)
                    || (!self.letters.connects_with_letter_before(letter))
                    || (!self.letters.connects_with_letter_after(previous_letter.0))
                    || (previous_letter.1 == LetterForm::Final
                        && !self
                            .letters
                            .connects_with_letters_before_and_after(previous_letter.0))
                {
                    output.push((letter, isolated_form));
                } else if previous_letter.1 == isolated_form {
                    *previous_letter = (previous_letter.0, LetterForm::Initial);
                    output.push((letter, LetterForm::Final));
                } else {
                    // Otherwise, we will change the previous letter to connect
                    // to the current letter
                    *previous_letter = (previous_letter.0, LetterForm::Medial);
                    output.push((letter, LetterForm::Final));
                }
            }

            // Remove ZWJ if it's the second to last item as it won't be useful
            let len = output.len();
            if support_zwj && len > 1 && output[len - 2].0 == ZWJ {
                output.remove(len - 2);
            }
        }

        if support_zwj && !output.is_empty() && output.last().unwrap().0 == ZWJ {
            output.pop();
        }

        if support_ligatures {
            // Clean text from Harakat to be able to find ligatures
            let mut text: String = text
                .chars()
                .filter(|c| !HARAKAT_RE.iter().any(|r| r.contains(c)))
                .collect();

            // Clean text from Tatweel to find ligatures if delete_tatweel
            if delete_tatweel {
                text = text.replace(TATWEEL, "")
            }

            for ((tmatchs, forms), enabled) in
                LIGATURES.iter().zip(self.config.ligatures.list.iter())
            {
                if !enabled {
                    continue;
                }
                for tmatch in *tmatchs {
                    for (idx, m) in text.match_indices(tmatch) {
                        // match_indices returns bytes offset
                        // we want character position
                        let a = text[..idx].chars().count();
                        let b = text[..idx + m.len()].chars().count();

                        let a_form = output[a].1;
                        let b_form = output[b - 1].1;
                        let ligature_form: LetterForm;

                        // +-----------+----------+---------+---------+----------+
                        // | a   \   b | ISOLATED | INITIAL | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+
                        // | ISOLATED  | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | INITIAL   | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | MEDIAL    | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // | FINAL     | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+

                        if a_form == isolated_form || a_form == LetterForm::Initial {
                            if b_form == isolated_form || b_form == LetterForm::Final {
                                ligature_form = LetterForm::Isolated;
                            } else {
                                ligature_form = LetterForm::Initial;
                            }
                        } else if b_form == isolated_form || b_form == LetterForm::Final {
                            ligature_form = LetterForm::Final;
                        } else {
                            ligature_form = LetterForm::Medial;
                        }

                        if forms.get(ligature_form) == '\0' {
                            continue;
                        }

                        output[a] = (forms.get(ligature_form), LetterForm::Unsupported);

                        for e in output[a + 1..b].iter_mut() {
                            *e = EMPTY;
                        }
                    }
                }
            }
        }

        let mut result = Vec::with_capacity(text.len());

        if !delete_harakat {
            if let Some(ph) = position_harakat.get(&-1) {
                result.extend(ph);
            }
        }

        for (i, (letter, form)) in output.into_iter().enumerate() {
            if letter != '\0' {
                result.push(self.letters.get_form(letter, form))
            }

            if !delete_harakat {
                if let Some(ph) = position_harakat.get(&(i as isize)) {
                    result.extend(ph);
                }
            }
        }

        result.into_iter().collect()
    }

    /// Reshape all lines in the given slice and return a new [Vec<String>] of strings
    pub fn reshape_lines<S, L>(&self, lines: L) -> Vec<String>
    where
        S: AsRef<str>,
        L: AsRef<[S]>,
    {
        let lines = lines.as_ref();
        let mut result = Vec::with_capacity(lines.len());
        for line in lines {
            result.push(self.reshape(line.as_ref()));
        }
        result
    }

    /// A safe way to modify the config ([ReshaperConfig]) after creating
    /// the [ArabicReshaper].
    pub fn modify_config<F>(&mut self, func: F)
    where
        F: FnOnce(&mut ReshaperConfig),
    {
        let language_before = self.config.language;

        func(&mut self.config);

        if language_before != self.config.language {
            // language changed, update letters
            self.letters.change_language(self.config.language);
        }
    }
}

impl From<ReshaperConfig> for ArabicReshaper {
    fn from(value: ReshaperConfig) -> Self {
        ArabicReshaper::new(value)
    }
}
