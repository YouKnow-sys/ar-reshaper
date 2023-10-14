use std::{collections::HashMap, ops::RangeInclusive};

use crate::{config::ReshaperConfig, letters::*, ligatures::*};

const NOT_SUPPORTED: i16 = -1;
const EMPTY: (CharType, i16) = (CharType::Unsupported, NOT_SUPPORTED);

#[derive(Copy, Clone)]
enum CharType {
    Supported(char),
    Unsupported,
}

impl From<char> for CharType {
    fn from(c: char) -> Self {
        CharType::Supported(c)
    }
}

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
    pub fn new(config: ReshaperConfig) -> Self {
        Self {
            letters: Letters::new(config.language),
            config,
        }
    }

    /// Reshape the given line and return the reshaped string
    pub fn reshape<S>(&self, text: S) -> String
    where
        S: AsRef<str>,
    {
        if text.as_ref().is_empty() {
            return String::new();
        }

        let mut output = Vec::new();

        let ReshaperConfig {
            delete_harakat,
            shift_harakat_position,
            delete_tatweel,
            support_zwj,
            use_unshaped_instead_of_isolated,
            support_ligatures,
            ..
        } = self.config;

        let mut position_harakat: HashMap<i16, Vec<char>> = HashMap::new();

        let isolated_form = match use_unshaped_instead_of_isolated {
            true => UNSHAPED,
            false => ISOLATED,
        };

        for letter in text.as_ref().chars() {
            if HARAKAT_RE.iter().any(|h| h.contains(&letter)) {
                if !delete_harakat {
                    let mut position = (output.len() - 1) as i16;
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
                output.push((letter, NOT_SUPPORTED))
            } else if output.is_empty() {
                output.push((letter, isolated_form)) // first letter
            } else {
                let previous_letter = output.last_mut().unwrap();
                if (previous_letter.1 == NOT_SUPPORTED)
                    || (!self.letters.connects_with_letter_before(letter))
                    || (!self.letters.connects_with_letter_after(previous_letter.0))
                    || (previous_letter.1 == FINAL
                        && !self
                            .letters
                            .connects_with_letters_before_and_after(previous_letter.0))
                {
                    output.push((letter, isolated_form));
                } else if previous_letter.1 == isolated_form {
                    *previous_letter = (previous_letter.0, INITIAL);
                    output.push((letter, FINAL));
                } else {
                    // Otherwise, we will change the previous letter to connect
                    // to the current letter
                    *previous_letter = (previous_letter.0, MEDIAL);
                    output.push((letter, FINAL));
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

        // this is needed because we can't have an empty char
        // we can use &str instead for output type but then we have
        // to change some other part of program to use &str instead
        // of char... un-needed complexity
        let mut output: Vec<(CharType, i16)> = output
            .into_iter()
            .map(|(c, i)| (CharType::from(c), i))
            .collect();

        if support_ligatures {
            // Clean text from Harakat to be able to find ligatures
            let mut text: String = text
                .as_ref()
                .chars()
                .filter(|c| !HARAKAT_RE.iter().any(|r| r.contains(c)))
                .collect();

            // Clean text from Tatweel to find ligatures if delete_tatweel
            if delete_tatweel {
                text = text.replace(TATWEEL, "")
            }

            for ((tmatchs, forms), enabled) in LIGATURES.iter().zip(self.config.ligatures.iter()) {
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
                        let ligature_form: i16;

                        // +-----------+----------+---------+---------+----------+
                        // | a   \   b | ISOLATED | INITIAL | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+
                        // | ISOLATED  | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | INITIAL   | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | MEDIAL    | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // | FINAL     | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+

                        if a_form == isolated_form || a_form == INITIAL {
                            if b_form == isolated_form || b_form == FINAL {
                                ligature_form = ISOLATED;
                            } else {
                                ligature_form = INITIAL;
                            }
                        } else if b_form == isolated_form || b_form == FINAL {
                            ligature_form = FINAL;
                        } else {
                            ligature_form = MEDIAL;
                        }

                        if forms[ligature_form as usize].is_empty() {
                            continue;
                        }

                        output[a] = (
                            forms[ligature_form as usize].chars().next().unwrap().into(),
                            NOT_SUPPORTED,
                        );

                        for e in output[a + 1..b].iter_mut() {
                            *e = EMPTY;
                        }
                    }
                }
            }
        }

        let mut result = Vec::new();

        if !delete_harakat && position_harakat.contains_key(&-1) {
            result.extend(position_harakat[&-1].clone());
        }

        for (i, o) in output.iter().enumerate() {
            if let CharType::Supported(c) = o.0 {
                if o.1 == NOT_SUPPORTED || o.1 == UNSHAPED {
                    result.push(c);
                } else {
                    let unc = self.letters[&c][o.1 as usize];
                    result.push(unc.chars().next().unwrap());
                }
            }

            if !delete_harakat && position_harakat.contains_key(&(i as i16)) {
                result.extend(position_harakat[&(i as i16)].clone());
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

    /// Reshape the given text and return the reshaped string.
    /// this method support to filter the characters that we wantt to reshape.
    /// its usefull when we want to keep the connection correct but don't reshape
    /// all the string.
    pub fn reshape_filter<S>(&self, text: S, filter: Vec<char>) -> String
    where
        S: AsRef<str>,
    {
        if text.as_ref().is_empty() {
            return String::new();
        }

        let mut output = Vec::new();

        let ReshaperConfig {
            delete_harakat,
            shift_harakat_position,
            delete_tatweel,
            support_zwj,
            use_unshaped_instead_of_isolated,
            support_ligatures,
            ..
        } = self.config;

        let mut position_harakat: HashMap<i16, Vec<char>> = HashMap::new();

        let isolated_form = match use_unshaped_instead_of_isolated {
            true => UNSHAPED,
            false => ISOLATED,
        };

        for letter in text.as_ref().chars() {
            if HARAKAT_RE.iter().any(|h| h.contains(&letter)) {
                if !delete_harakat {
                    let mut position = (output.len() - 1) as i16;
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
                output.push((letter, NOT_SUPPORTED))
            } else if output.is_empty() {
                output.push((letter, isolated_form)) // first letter
            } else {
                let previous_letter = output.last_mut().unwrap();
                if (previous_letter.1 == NOT_SUPPORTED)
                    || (!self.letters.connects_with_letter_before(letter))
                    || (!self.letters.connects_with_letter_after(previous_letter.0))
                    || (previous_letter.1 == FINAL
                        && !self
                            .letters
                            .connects_with_letters_before_and_after(previous_letter.0))
                {
                    output.push((letter, isolated_form));
                } else if previous_letter.1 == isolated_form {
                    *previous_letter = (previous_letter.0, INITIAL);
                    output.push((letter, FINAL));
                } else {
                    // Otherwise, we will change the previous letter to connect
                    // to the current letter
                    *previous_letter = (previous_letter.0, MEDIAL);
                    output.push((letter, FINAL));
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

        // this is needed because we can't have an empty char
        // we can use &str instead for output type but then we have
        // to change some other part of program to use &str instead
        // of char... un-needed complexity
        let mut output: Vec<(CharType, i16)> = output
            .into_iter()
            .map(|(c, i)| (CharType::from(c), i))
            .collect();

        if support_ligatures {
            // Clean text from Harakat to be able to find ligatures
            let mut text: String = text
                .as_ref()
                .chars()
                .filter(|c| !HARAKAT_RE.iter().any(|r| r.contains(c)))
                .collect();

            // Clean text from Tatweel to find ligatures if delete_tatweel
            if delete_tatweel {
                text = text.replace(TATWEEL, "")
            }

            for ((tmatchs, forms), enabled) in LIGATURES.iter().zip(self.config.ligatures.iter()) {
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
                        let ligature_form: i16;

                        // +-----------+----------+---------+---------+----------+
                        // | a   \   b | ISOLATED | INITIAL | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+
                        // | ISOLATED  | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | INITIAL   | ISOLATED | INITIAL | INITIAL | ISOLATED |
                        // | MEDIAL    | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // | FINAL     | FINAL    | MEDIAL  | MEDIAL  | FINAL    |
                        // +-----------+----------+---------+---------+----------+

                        if a_form == isolated_form || a_form == INITIAL {
                            if b_form == isolated_form || b_form == FINAL {
                                ligature_form = ISOLATED;
                            } else {
                                ligature_form = INITIAL;
                            }
                        } else if b_form == isolated_form || b_form == FINAL {
                            ligature_form = FINAL;
                        } else {
                            ligature_form = MEDIAL;
                        }

                        if forms[ligature_form as usize].is_empty() {
                            continue;
                        }

                        output[a] = (
                            forms[ligature_form as usize].chars().next().unwrap().into(),
                            NOT_SUPPORTED,
                        );

                        for e in output[a + 1..b].iter_mut() {
                            *e = EMPTY;
                        }
                    }
                }
            }
        }

        let mut result = Vec::new();

        if !delete_harakat && position_harakat.contains_key(&-1) {
            result.extend(position_harakat[&-1].clone());
        }

        for (i, o) in output.iter().enumerate() {
            if let CharType::Supported(c) = o.0 {
                if o.1 == NOT_SUPPORTED || o.1 == UNSHAPED || !filter.contains(&c) {
                    result.push(c);
                } else {
                    let unc = self.letters[&c][o.1 as usize];
                    result.push(unc.chars().next().unwrap());
                }
            }

            if !delete_harakat && position_harakat.contains_key(&(i as i16)) {
                result.extend(position_harakat[&(i as i16)].clone());
            }
        }

        result.into_iter().collect()
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
