use clap::{App, Arg, ArgMatches};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");

pub fn get_matches() -> ArgMatches<'static> {
    App::new(PACKAGE_NAME)
        .version(VERSION)
        .about("Generates random passwords and keys.")
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Enable debug mode"),
        )
        .arg(
            Arg::with_name("alphabet")
                .short("a")
                .long("alphabet")
                .value_name("ALPHABET")
                .possible_values(&["words-fi", "commonsafe", "normal", "ascii", "assembly"])
                .help("Specify the alphabet to use for random value generation"),
        )
        .arg(
            Arg::with_name("bits")
                .short("b")
                .long("bits")
                .value_name("BITS")
                .help("Specify the amount of bits for each random value"),
        )
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .value_name("COUNT")
                .help("Number of passwords to generate"),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .requires_all(&["alphabet"])
                .help("Sets the delimiter between each letter or word")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("rngtest")
                .short("r")
                .long("rngtest")
                .value_name("generator")
                .possible_values(&["rdrand", "os", "cpujitter", "cpujitter-raw"])
                .takes_value(true)
                .help("Optional test mode for RNG testing. Will provide raw bytes to stdout.")
                .conflicts_with_all(&["bits", "alphabet", "count"]), // Conflicts with other options
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .long("size")
                .value_name("data size (u64 words)")
                .requires_all(&["rngtest"]) // Requires rngtest if used
                .takes_value(true)
                .help("Specifies the generated data size in u64 words for RNG testing.")
                .conflicts_with_all(&["bits", "alphabet", "count"]), // Conflicts with other options
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .short("f")
                .value_name("format")
                .requires_all(&["rngtest"]) // Requires rngtest if used
                .possible_values(&["raw", "u8", "u16", "u32", "u64"])
                .required_if("rngtest", "generator") // Required if rngtest option is used
                .help("Specifies the data format for RNG testing."),
        )
        .get_matches()
}
