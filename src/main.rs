#![allow(unused_imports)]
#![allow(unused_assignments)]

mod hmac_drbg;

mod random;
use random::{generate_u64, generate_u64_cpujitter, generate_u64_os, generate_u64_rdrand};

mod alphabet;
use alphabet::{
    alphabet_ascii_get_count, alphabet_ascii_get_element, alphabet_assembly_get_count,
    alphabet_assembly_get_element, alphabet_commonsafe_get_count, alphabet_commonsafe_get_element,
    alphabet_normal_get_count, alphabet_normal_get_element,
};
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

fn main() {
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

    let mut alphabet_item: fn(usize) -> Option<String> = |_| None;
    let mut alphabet_count: fn() -> usize = || 0;

    // Match the alphabet count and generator functions to the selected alphabet
    match config.alphabet.as_str() {
        "words-fi" => {
            alphabet_count = alphabet::alphabet_wordsfi_get_count;
            alphabet_item = alphabet::alphabet_wordsfi_get_element;
        }
        "commonsafe" => {
            alphabet_count = alphabet::alphabet_commonsafe_get_count;
            alphabet_item = alphabet::alphabet_commonsafe_get_element;
        }
        "normal" => {
            alphabet_count = alphabet::alphabet_normal_get_count;
            alphabet_item = alphabet::alphabet_normal_get_element;
        }
        "ascii" => {
            alphabet_count = alphabet::alphabet_ascii_get_count;
            alphabet_item = alphabet::alphabet_ascii_get_element;
        }
        "assembly" => {
            alphabet_count = alphabet::alphabet_assembly_get_count;
            alphabet_item = alphabet::alphabet_assembly_get_element;
        }
        _ => {
            print!("Error: Unknown alphabet specified. Exiting");
            std::process::exit(1);
        }
    }

    if config.debug {
        println!("Using alphabet: {}", config.alphabet);
        println!("alphabet_count: {}", alphabet_count());
        println!("request bits: {}", config.bits);
    }

    // Find the number of characters needed
    let bits_per_element = (alphabet_count() as f64).log2();
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
            let mut random_value: Option<u64> = None;
            while random_value.is_none() {
                let val = generate_u64();
                if val.unwrap() <= (u64::MAX - (alphabet_count() as u64)) {
                    random_value = val;
                }
            }

            // get the corresponding alphabet element
            let random_index = (random_value.unwrap() % alphabet_count() as u64) as usize;
            let random_element = alphabet_item(random_index).unwrap();
            password_string.push_str(&random_element);
            if i < num_elements - 1 {
                password_string.push_str(&config.delimiter);
            }
        }

        println!("{}", password_string);
    }

    std::process::exit(0);
}
