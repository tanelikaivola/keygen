use std::str::FromStr;

pub enum NumFormat {
    RawBinary,
    U8,
    U16,
    U32,
    U64,
}

impl FromStr for NumFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "raw" => Ok(NumFormat::RawBinary),
            "u8" => Ok(NumFormat::U8),
            "u16" => Ok(NumFormat::U16),
            "u32" => Ok(NumFormat::U32),
            "u64" => Ok(NumFormat::U64),
            _ => Err(()),
        }
    }
}
