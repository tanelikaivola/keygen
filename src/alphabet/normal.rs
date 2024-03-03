use super::{Alphabet, Error, Result};

static ALPHABET_NORMAL: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// "Normal characters"
pub struct Normal {}

impl Alphabet for Normal {
    fn count(&self) -> usize {
        ALPHABET_NORMAL.len()
    }

    fn item(&self, n: usize) -> Result<String> {
        ALPHABET_NORMAL
            .get(n)
            .map(std::string::ToString::to_string)
            .ok_or(Error::NonExistentCharacter(n))
    }
}
