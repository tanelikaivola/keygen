use super::{Alphabet, Error, Result};

/// Complete printable ASCII charset
pub struct Ascii {}

impl Alphabet for Ascii {
    fn count(&self) -> usize {
        const PRINTABLE_ASCII_START: u8 = 32;
        const PRINTABLE_ASCII_END: u8 = 126;

        (PRINTABLE_ASCII_END - PRINTABLE_ASCII_START + 1) as usize
    }
    fn item(&self, n: usize) -> Result<String> {
        const PRINTABLE_ASCII_START: usize = 32;
        const PRINTABLE_ASCII_END: usize = 126;

        if n > PRINTABLE_ASCII_END - PRINTABLE_ASCII_START {
            return Err(Error::NonExistentCharacter(n));
        }

        let c = n as u8 + PRINTABLE_ASCII_START as u8;
        let str = String::from_utf8(vec![c]).unwrap();
        Ok(str)
    }
}
