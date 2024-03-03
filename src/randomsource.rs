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
            Self::Combined => write!(f, "combined"),
            Self::Rdrand => write!(f, "rdrand"),
            Self::Os => write!(f, "os"),
            Self::CpuJitter => write!(f, "cpujitter"),
            Self::CpuJitterRaw => write!(f, "cpujitter-raw"),
        }
    }
}

impl FromStr for RandomSource {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "combined" => Ok(Self::Combined),
            "rdrand" => Ok(Self::Rdrand),
            "os" => Ok(Self::Os),
            "cpujitter" => Ok(Self::CpuJitter),
            "cpujitter-raw" => Ok(Self::CpuJitterRaw),
            _ => Err(()),
        }
    }
}
