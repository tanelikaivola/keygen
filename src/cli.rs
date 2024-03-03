pub use clap::{Parser, ValueEnum};

use crate::{numformat::NumFormat, randomsource::RandomSource};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Parser, Debug)]
#[clap(name = PACKAGE_NAME, version = VERSION, about = "Generates random passwords and keys.")]
pub struct Cli {
    /// Enable debug mode
    #[clap(long)]
    pub debug: bool,

    /// Specify the alphabet to use for random value generation
    #[clap(short, long, default_value = "commonsafe")]
    // , possible_values
    pub alphabet: Alphabet,

    /// Specify the amount of bits for each random value
    #[clap(short, long, default_value_t = 256)]
    pub bits: u32,

    /// Number of passwords to generate
    #[clap(short, long, default_value_t = 1)]
    pub count: usize,

    /// Sets the delimiter between each letter or word
    #[clap(
        short,
        long,
        value_name = "DELIMITER",
        requires = "alphabet",
        default_value = ""
    )]
    pub delimiter: String,

    /// Optional test mode for RNG testing. Will provide raw bytes to stdout.
    #[clap(short, long, conflicts_with_all = &["bits", "alphabet", "count"])]
    pub rngtest: Option<RandomSource>,

    /// Specifies the generated data size in u64 words for RNG testing.
    #[clap(short = 's', long, requires = "rngtest", conflicts_with_all = &["bits", "alphabet", "count"], default_value_t = 1)]
    pub size: u32,

    /// Specifies the data format for RNG testing.
    #[clap(
        short,
        long,
        requires = "rngtest",
        required_if_eq("rngtest", "generator"),
        default_value = "u64"
    )]
    pub format: NumFormat,
}

#[derive(ValueEnum, Clone, Debug, Copy)]
pub enum Alphabet {
    WordsFi,
    Commonsafe,
    Normal,
    Ascii,
    Assembly,
}
