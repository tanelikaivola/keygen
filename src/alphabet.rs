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

impl FromStr for Box<dyn Alphabet> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "words-fi" => Ok(Box::new(wordsfi::WordsFi {})),
            "commonsafe" => Ok(Box::new(commonsafe::CommonSafe {})),
            "normal" => Ok(Box::new(normal::Normal {})),
            "ascii" => Ok(Box::new(ascii::Ascii {})),
            "assembly" => Ok(Box::new(assembly::Assembly {})),
            _ => Err(Error::InvalidAlphabet(s.into())),
        }
    }
}
