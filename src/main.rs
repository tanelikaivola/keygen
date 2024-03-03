#![allow(unused_imports)]
#![allow(unused_assignments)]

mod hmac_drbg;

mod random;
use random::{generate_u64, generate_u64_cpujitter, generate_u64_os, generate_u64_rdrand};

mod alphabet;
use alphabet::Alphabet;
use zeroize::Zeroize;

mod numformat;
use numformat::{print_formatted_value, NumFormat, PrintFormattedValue};

mod randomsource;
use randomsource::RandomSource;

use std::env;
use std::fmt;
use std::str::FromStr;

mod cli;
mod config;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli::get_matches();
    let config: Config = matches.into();

    if let Some((generator, data_size, data_format)) = &config.rngtest {
        let num_values = *data_size as u64;

        // Choose the appropriate generator function based on the selected generator
        let generator_fn: fn() -> Option<u64> = match generator {
            RandomSource::Rdrand => random::generate_u64_rdrand,
            RandomSource::Os => random::generate_u64_os,
            RandomSource::CpuJitter => random::generate_u64_cpujitter,
            RandomSource::CpuJitterRaw => random::generate_u64_cpujitter_raw,
            _ => unimplemented!(),
        };

        for _ in 0..num_values {
            if let Some(value) = generator_fn() {
                data_format.print_formatted_value(value);
            }
        }

        std::process::exit(0);
    }
    let alphabet: Box<dyn Alphabet> = config.alphabet.parse()?;

    if config.debug {
        println!("Using alphabet: {}", config.alphabet);
        println!("alphabet_count: {}", alphabet.count());
        println!("request bits: {}", config.bits);
    }

    // Find the number of characters needed
    let bits_per_element = alphabet.bits_per_element();
    let num_elements = (config.bits as f64 / bits_per_element as f64).ceil() as u32;

    if config.debug {
        println!("Bits per element: {}", bits_per_element);
        println!("Num of elements: {}", num_elements);
    }

    // Create the password(s)
    for _ in 0..config.count {
        let mut password_string = String::new();

        for i in 0..num_elements {
            // pull out a random value that does not result in modulo bias
            let random_value = {
                loop {
                    let val = generate_u64().unwrap();
                    if val <= (u64::MAX - (alphabet.count() as u64)) {
                        break val;
                    }
                }
            };

            // get the corresponding alphabet element
            let random_index = (random_value % alphabet.count() as u64) as usize;
            let random_element = alphabet.item(random_index).unwrap();
            password_string.push_str(&random_element);
            if i < num_elements - 1 {
                password_string.push_str(&config.delimiter);
            }
        }

        println!("{}", password_string);
    }

    std::process::exit(0);
}
