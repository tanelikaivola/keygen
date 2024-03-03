use std::{fmt, str::FromStr};

pub enum RandomSource {
    Combined,
    Rdrand,
    Os,
    CpuJitter,
    CpuJitterRaw,
}

impl fmt::Debug for RandomSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandomSource::Combined => write!(f, "combined"),
            RandomSource::Rdrand => write!(f, "rdrand"),
            RandomSource::Os => write!(f, "os"),
            RandomSource::CpuJitter => write!(f, "cpujitter"),
            RandomSource::CpuJitterRaw => write!(f, "cpujitter-raw"),
        }
    }
}

impl FromStr for RandomSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(RandomSource::Combined),
            "rdrand" => Ok(RandomSource::Rdrand),
            "os" => Ok(RandomSource::Os),
            "cpujitter" => Ok(RandomSource::CpuJitter),
            "cpujitter-raw" => Ok(RandomSource::CpuJitterRaw),
            _ => Err(()),
        }
    }
}
