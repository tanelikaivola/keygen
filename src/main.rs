//! Generates random passwords and keys.

mod hmac_drbg;

mod random;
use random::generate_u64;

mod alphabet;
use alphabet::Alphabet;

mod numformat;
use numformat::PrintFormattedValue;

mod randomsource;
use randomsource::RandomSource;

mod cli;
use cli::Parser;
mod config;
use config::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: Config = cli::Cli::parse().into();

    if let Some((generator, data_size, data_format)) = &config.rngtest {
        let num_values = u64::from(*data_size);

        // Choose the appropriate generator function based on the selected generator
        let generator_fn: fn() -> Result<u64, _> = match generator {
            RandomSource::Rdrand => random::generate_u64_rdrand,
            RandomSource::Os => random::generate_u64_os,
            RandomSource::Cpujitter => random::generate_u64_cpujitter,
            RandomSource::CpujitterRaw => random::generate_u64_cpujitter_raw,
            RandomSource::Combined => unimplemented!(),
        };

        for _ in 0..num_values {
            match generator_fn() {
                Ok(value) => {
                    data_format.print_formatted_value(value);
                }
                Err(e) => return Err(e.into()),
            }
        }

        return Ok(());
    }

    let alphabet: Box<dyn Alphabet> = config.alphabet.into();

    if config.debug {
        println!("Using alphabet: {:?}", config.alphabet);
        println!("alphabet_count: {}", alphabet.count());
        println!("request bits: {}", config.bits);
    }

    // Find the number of characters needed
    let bits_per_element = alphabet.bits_per_element();
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_lossless,
        clippy::cast_sign_loss
    )]
    let num_elements = (config.bits as f64 / bits_per_element).ceil() as u32;

    if config.debug {
        println!("Bits per element: {bits_per_element}");
        println!("Num of elements: {num_elements}");
    }

    // Create the password(s)
    for _ in 0..config.count {
        let mut password_string = String::new();

        for i in 0..num_elements {
            // pull out a random value that does not result in modulo bias
            let random_value = {
                loop {
                    let val = generate_u64()?;
                    if val <= (u64::MAX - (alphabet.count() as u64)) {
                        break val;
                    }
                }
            };

            // get the corresponding alphabet element
            let random_index = usize::try_from(random_value % alphabet.count() as u64)?;
            let random_element = alphabet.item(random_index)?;
            password_string.push_str(&random_element);
            if i < num_elements - 1 {
                password_string.push_str(&config.delimiter);
            }
        }

        println!("{password_string}");
    }

    Ok(())
}
