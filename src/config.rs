use crate::{numformat::NumFormat, randomsource::RandomSource};

pub struct Config {
    pub debug: bool,
    pub bits: u32,
    pub alphabet: crate::cli::Alphabet,
    pub delimiter: String,
    pub count: usize,
    pub rngtest: Option<(RandomSource, u32, NumFormat)>,
}

use crate::cli::Cli;

impl From<Cli> for Config {
    fn from(matches: Cli) -> Self {
        Self {
            debug: matches.debug,
            bits: matches.bits,
            alphabet: matches.alphabet,
            count: matches.count,
            delimiter: matches.delimiter,

            rngtest: if let Some(rngtest) = matches.rngtest {
                let generator = rngtest;
                let data_size = matches.size;
                let num_format = matches.format;
                Some((generator, data_size, num_format))
            } else {
                None
            },
        }
    }
}
