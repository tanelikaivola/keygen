//! Alphabet definitions

use std::{process, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Attempting to access non-existent character of index {0}.")]
    NonExistentCharacter(usize),
    #[error("Invalid UTF-8 in alphabet for index {0}")]
    InvalidUtf8(usize),
    #[error("Invalid alphabet: {0}")]
    InvalidAlphabet(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Alphabet trait for getting items from an alphabet definition
pub trait Alphabet {
    fn count(&self) -> usize;
    fn item(&self, n: usize) -> Result<String>;
    fn bits_per_element(&self) -> f64 {
        (self.count() as f64).log2()
    }
}

pub mod ascii;
pub mod assembly;
pub mod commonsafe;
pub mod normal;
pub mod wordsfi;

impl From<crate::cli::Alphabet> for Box<dyn Alphabet> {
    fn from(alphabet: crate::cli::Alphabet) -> Self {
        match alphabet {
            crate::cli::Alphabet::WordsFi => Box::new(wordsfi::WordsFi {}),
            crate::cli::Alphabet::Commonsafe => Box::new(commonsafe::CommonSafe {}),
            crate::cli::Alphabet::Normal => Box::new(normal::Normal {}),
            crate::cli::Alphabet::Ascii => Box::new(ascii::Ascii {}),
            crate::cli::Alphabet::Assembly => Box::new(assembly::Assembly {}),
        }
    }
}
