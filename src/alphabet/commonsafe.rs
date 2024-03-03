//! Common and safe characters to use with various different keymaps. The default.

use super::{Alphabet, Error, Result};

static ALPHABET_COMMONSAFE: &[char] = &[
    '!', '#', '%', ',', '.', '1', '2', '3', '4', '5', '6', '7', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'A', 'C',
    'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
];

pub struct CommonSafe {}

impl Alphabet for CommonSafe {
    fn count(&self) -> usize {
        ALPHABET_COMMONSAFE.len()
    }

    fn item(&self, n: usize) -> Result<String> {
        ALPHABET_COMMONSAFE
            .get(n)
            .map(std::string::ToString::to_string)
            .ok_or(Error::NonExistentCharacter(n))
    }
}
