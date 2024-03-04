use std::fmt;

pub struct BitVector {
    bits: Vec<bool>,
}

impl BitVector {
    pub const fn new() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn add_bit(&mut self, bit: bool) {
        self.bits.push(bit);
    }

    pub fn is_full(&self) -> bool {
        self.bits.len() == 64
    }

    pub fn to_u64(&self) -> u64 {
        let mut result: u64 = 0;
        for (i, &bit) in self.bits.iter().enumerate() {
            if bit {
                result |= 1u64 << i;
            }
        }
        result
    }
}

impl fmt::Debug for BitVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitVector {{ bits: {:?} }}", self.bits)
    }
}
