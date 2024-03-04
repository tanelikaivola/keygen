//! Generates random passwords and keys.

mod hmac_drbg;

mod random;
use random::Generator;

mod alphabet;
use alphabet::Alphabet;

mod numformat;
use numformat::PrintFormattedValue;

mod cli;
use cli::Parser;

mod bitvector;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    if let Some(rngtest) = cli.rngtest {
        let generator = rngtest;
        let data_size = cli.size;
        let data_format = cli.format;

        let num_values = u64::from(data_size);

        for _ in 0..num_values {
            match generator.generate_u64() {
                Ok(value) => {
                    data_format.print_formatted_value(value);
                }
                Err(e) => return Err(e.into()),
            }
        }

        return Ok(());
    }

    let alphabet: Box<dyn Alphabet> = cli.alphabet.into();

    if cli.debug {
        println!("Using alphabet: {:?}", cli.alphabet);
        println!("alphabet_count: {}", alphabet.count());
        println!("request bits: {}", cli.bits);
    }

    // Find the number of characters needed
    let bits_per_element = alphabet.bits_per_element();
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_lossless,
        clippy::cast_sign_loss
    )]
    let num_elements = (cli.bits as f64 / bits_per_element).ceil() as u32;

    if cli.debug {
        println!("Bits per element: {bits_per_element}");
        println!("Num of elements: {num_elements}");
    }

    // Create the password(s)
    for _ in 0..cli.count {
        let mut password_string = String::new();

        for i in 0..num_elements {
            // pull out a random value that does not result in modulo bias
            let random_value = {
                loop {
                    let val = random::OsRand {}.generate_u64()?;
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
                password_string.push_str(&cli.delimiter);
            }
        }

        println!("{password_string}");
    }

    Ok(())
}
