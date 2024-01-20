use core::ops::Index;

use crate::{form::*, Language};

pub mod letters_db;

/// The main letters type.
///
/// this struct is responsible for managing all the letters data.
#[derive(Clone)]
pub(crate) struct Letters(pub &'static [LettersType]);

impl Default for Letters {
    fn default() -> Self {
        Self(&letters_db::LETTERS_ARABIC)
    }
}

impl Letters {
    /// Create a new [`Letters`] with the given [`Language`]
    pub const fn new(language: Language) -> Self {
        Self(match language {
            Language::Arabic => &letters_db::LETTERS_ARABIC,
            Language::ArabicV2 => &letters_db::LETTERS_ARABIC_V2,
            Language::Kurdish => &letters_db::LETTERS_KURDISH,
            Language::Custom(c) => c,
        })
    }

    /// Change the language of letters after creating the [`Letters`]
    pub fn change_language(&mut self, language: Language) {
        self.0 = match language {
            Language::Arabic => &letters_db::LETTERS_ARABIC,
            Language::ArabicV2 => &letters_db::LETTERS_ARABIC_V2,
            Language::Kurdish => &letters_db::LETTERS_KURDISH,
            Language::Custom(c) => c,
        };
    }

    /// Check if the given [`char`] exist in the letters
    pub fn contains_key(&self, key: &char) -> bool {
        self.0.iter().any(|(k, _)| k == key)
    }

    /// Try to get the forms corresponding the letters
    pub fn get(&self, key: &char) -> Option<&Forms> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, f)| f)
    }

    pub fn get_form(&self, letter: char, form: LetterForm) -> char {
        if matches!(form, LetterForm::Unshaped | LetterForm::Unsupported) {
            return letter;
        }

        let forms = self
            .get(&letter)
            .expect("Can't find character inside letters");

        forms.get(form)
    }

    pub fn connects_with_letter_before(&self, letter: char) -> bool {
        self.get(&letter)
            .is_some_and(|forms| forms.end != '\0' || forms.medial != '\0')
    }

    pub fn connects_with_letter_after(&self, letter: char) -> bool {
        self.get(&letter)
            .is_some_and(|forms| forms.initial != '\0' || forms.medial != '\0')
    }

    pub fn connects_with_letters_before_and_after(&self, letter: char) -> bool {
        self.get(&letter).is_some_and(|forms| forms.medial != '\0')
    }
}

impl Index<&char> for Letters {
    type Output = Forms;

    fn index(&self, index: &char) -> &Self::Output {
        &self.0.iter().find(|(k, _)| k == index).unwrap().1
    }
}
