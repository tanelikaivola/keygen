use std::str::FromStr;

use clap::ArgMatches;

use crate::{numformat::NumFormat, randomsource::RandomSource};

pub struct Config {
    pub debug: bool,
    pub bits: u32,
    pub alphabet: String,
    pub delimiter: String,
    pub count: usize,
    pub rngtest: Option<(RandomSource, u32, NumFormat)>,
}

impl From<ArgMatches<'_>> for Config {
    fn from(matches: ArgMatches) -> Self {
        Self {
            debug: matches.is_present("debug"),
            bits: matches
                .value_of("bits")
                .map(|b| b.parse().unwrap())
                .unwrap_or(256),
            alphabet: matches
                .value_of("alphabet")
                .unwrap_or("commonsafe")
                .to_string(),
            count: matches
                .value_of("count")
                .map(|i| i.parse::<usize>().unwrap_or(1))
                .unwrap_or(1),
            delimiter: matches.value_of("delimiter").unwrap_or("").to_string(),

            rngtest: if matches.is_present("rngtest") {
                let generator_str = matches.value_of("rngtest").unwrap();
                let generator = RandomSource::from_str(generator_str).expect("Invalid generator");
                let data_size = matches
                    .value_of("size")
                    .map(|s| s.parse::<u32>().unwrap_or(1))
                    .unwrap_or(1);
                let num_format_str = matches.value_of("format").unwrap_or("u64");
                let num_format =
                    NumFormat::from_str(num_format_str).expect("Invalid number format");
                Some((generator, data_size, num_format))
            } else {
                None
            },
        }
    }
}
