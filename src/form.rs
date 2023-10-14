/// Type of letters
pub type LettersType = (char, Forms);

/// Form of the letter
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub(crate) enum LetterForm {
    Isolated,
    Initial,
    Medial,
    Final,
    Unsupported,
    Unshaped,
}

/// The main type used to show letter form in each position
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Forms {
    pub isolated: char,
    pub initial: char,
    pub medial: char,
    pub end: char,
}

impl Forms {
    pub const fn new(isolated: char, initial: char, medial: char, end: char) -> Self {
        Self {
            isolated,
            initial,
            medial,
            end,
        }
    }

    pub(crate) const fn get(&self, form: LetterForm) -> char {
        match form {
            LetterForm::Isolated => self.isolated,
            LetterForm::Initial => self.initial,
            LetterForm::Medial => self.medial,
            LetterForm::Final => self.end,
            _ => panic!("Unsupported Letter form"),
        }
    }
}
