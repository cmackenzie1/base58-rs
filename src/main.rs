use std::env;
use std::io::{self, Read, Write};
use std::process;

use b58::{Alphabet, DecodeError, decode_with_alphabet, encode_with_alphabet};

fn print_usage() {
    eprintln!("base58 - Base58 encoding and decoding utility");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    base58 [OPTIONS]");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("    -d, --decode                 Decode Base58 input (default: encode)");
    eprintln!(
        "    -a, --alphabet <ALPHABET>    Specify alphabet (bitcoin, ripple, flickr) [default: bitcoin]"
    );
    eprintln!("    -h, --help                   Show this help message");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("    printf 'Hello, World!' | base58");
    eprintln!("    printf '72k1xXWG59fYdzSNoA' | base58 -d");
    eprintln!("    base58 --alphabet ripple < input.txt");
    eprintln!("    base58 -d --alphabet bitcoin < encoded.txt");
}

fn parse_alphabet(alphabet_str: &str) -> Result<Alphabet, String> {
    match alphabet_str.to_lowercase().as_str() {
        "bitcoin" | "btc" => Ok(Alphabet::Bitcoin),
        "ripple" | "xrp" => Ok(Alphabet::Ripple),
        "flickr" => Ok(Alphabet::Flickr),
        _ => Err(format!(
            "Unknown alphabet: {alphabet_str}. Valid options: bitcoin, ripple, flickr"
        )),
    }
}

fn read_stdin() -> Result<Vec<u8>, io::Error> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut decode_mode = false;
    let mut alphabet = Alphabet::Bitcoin;
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-d" | "--decode" => decode_mode = true,
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-a" | "--alphabet" => {
                if i + 1 >= args.len() {
                    eprintln!("Error: --alphabet requires a value");
                    process::exit(1);
                }
                i += 1;
                match parse_alphabet(&args[i]) {
                    Ok(a) => alphabet = a,
                    Err(e) => {
                        eprintln!("Error: {e}");
                        process::exit(1);
                    }
                }
            }
            arg if arg.starts_with("-") => {
                eprintln!("Error: Unknown option: {arg}");
                print_usage();
                process::exit(1);
            }
            _ => {
                eprintln!("Error: Unexpected argument: {}", args[i]);
                process::exit(1);
            }
        }
        i += 1;
    }

    let input = match read_stdin() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading input: {e}");
            process::exit(1);
        }
    };

    if decode_mode {
        let input_str = match String::from_utf8(input) {
            Ok(s) => s.trim().to_string(),
            Err(e) => {
                eprintln!("Error: Input is not valid UTF-8: {e}");
                process::exit(1);
            }
        };

        match decode_with_alphabet(&input_str, alphabet) {
            Ok(decoded) => {
                if let Err(e) = io::stdout().write_all(&decoded) {
                    eprintln!("Error writing output: {e}");
                    process::exit(1);
                }
            }
            Err(DecodeError::InvalidCharacter(c)) => {
                eprintln!("Error: Invalid character '{c}' in Base58 input");
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    } else {
        let result = encode_with_alphabet(&input, alphabet);
        println!("{result}");
    }
}
