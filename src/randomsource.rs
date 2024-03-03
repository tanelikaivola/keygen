use clap::ValueEnum;
use std::{fmt, str::FromStr};

#[derive(ValueEnum, Clone)]
pub enum RandomSource {
    Combined,
    Rdrand,
    Os,
    Cpujitter,
    CpujitterRaw,
}

impl fmt::Debug for RandomSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Combined => write!(f, "combined"),
            Self::Rdrand => write!(f, "rdrand"),
            Self::Os => write!(f, "os"),
            Self::Cpujitter => write!(f, "cpujitter"),
            Self::CpujitterRaw => write!(f, "cpujitter-raw"),
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
            "cpujitter" => Ok(Self::Cpujitter),
            "cpujitter-raw" => Ok(Self::CpujitterRaw),
            _ => Err(()),
        }
    }
}
