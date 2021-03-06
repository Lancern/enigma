extern crate enigma;

extern crate clap;
extern crate serde_json;

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use enigma::{Enigma, PlugBoard, Reflector, Rotator, RotatorGroup};
use enigma::math::{Permutation, PermutationBuilder};
use enigma::utils::{RUNE_SET_SIZE, RUNE_VALUE_MAX};

#[derive(Clone, Debug)]
struct InvalidConfigError {
    message: String,
}

impl InvalidConfigError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for InvalidConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid config: {}", self.message))
    }
}

impl Error for InvalidConfigError { }

#[derive(Clone, Debug)]
struct Config {
    plug_board: Vec<[char; 2]>,
    rotators: [(Vec<char>, u8); 3],
    reflector: Vec<[char; 2]>,
}

impl Config {
    fn create_plug_board(&self) -> PlugBoard {
        let perm = match create_permutation_from_swaps(&self.plug_board) {
            Ok(perm) => perm,
            Err(e) => {
                eprintln!("Invalid plug board setting: {}", e);
                std::process::exit(1);
            },
        };

        match PlugBoard::from_perm(perm) {
            Ok(board) => board,
            Err(e) => {
                eprintln!("Invalid plug board setting: {}", e);
                std::process::exit(1);
            },
        }
    }

    fn create_rotator(&self, index: usize) -> Rotator {
        let perm = match create_permutation_from(&self.rotators[index].0) {
            Ok(perm) => perm,
            Err(e) => {
                eprintln!("Invalid rotator setting: {}", e);
                std::process::exit(1);
            },
        };

        let offset = self.rotators[index].1;

        match Rotator::new(perm, offset) {
            Ok(rotator) => rotator,
            Err(e) => {
                eprintln!("Invalid rotator setting: {}", e);
                std::process::exit(1);
            },
        }
    }

    fn create_rotator_group(&self) -> RotatorGroup {
        RotatorGroup::new([
            self.create_rotator(0),
            self.create_rotator(1),
            self.create_rotator(2),
        ])
    }

    fn create_reflector(&self) -> Reflector {
        let perm = match create_permutation_from_swaps(&self.reflector) {
            Ok(perm) => perm,
            Err(e) => {
                eprintln!("Invalid reflector setting: {}", e);
                std::process::exit(1);
            },
        };

        match Reflector::from_perm(perm) {
            Ok(reflector) => reflector,
            Err(e) => {
                eprintln!("Invalid reflector setting: {}", e);
                std::process::exit(1);
            },
        }
    }

    fn create_enigma(&self) -> Enigma {
        let plug_board = self.create_plug_board();
        let rotator_group = self.create_rotator_group();
        let reflector = self.create_reflector();
        Enigma::new(plug_board, rotator_group, reflector)
    }
}

fn create_permutation_from_swaps(swaps: &Vec<[char; 2]>)
    -> Result<Permutation, InvalidConfigError> {
    let mut builder = PermutationBuilder::new(RUNE_SET_SIZE);

    for sw in swaps {
        if !sw[0].is_ascii_alphabetic() {
            return Err(InvalidConfigError::new(
                format!("{} is not an ASCII alphabetic character", sw[0])));
        }
        if !sw[1].is_ascii_alphabetic() {
            return Err(InvalidConfigError::new(
                format!("{} is not an ASCII alphabetic character", sw[1])));
        }

        let lhs = (sw[0].to_ascii_lowercase() - 'a') as u8;
        let rhs = (sw[1].to_ascii_lowercase() - 'a') as u8;
        builder = builder.swap(lhs, rhs);
    }

    Ok(builder.build())
}

fn create_permutation_from(char_perm: &Vec<char>) -> Result<Permutation, InvalidConfigError> {
    let mut perm: Vec<u8> = Vec::with_capacity(char_perm.len());

    for ch in char_perm {
        if !ch.is_ascii_alphabetic() {
            return Err(InvalidConfigError::new(
                format!("{} is not an ASCII alphabetic character", ch)));
        }

        perm.push((ch.to_ascii_lowercase() - 'a') as u8);
    }

    Permutation::from_perm(perm)
        .map_err(|e| InvalidConfigError::new(
            format!("invalid permutation: {}", e)))
}

fn load_config(path: &Path) -> Config {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Cannot read config file: {}", e);
            std::process::exit(1);
        },
    };

    match serde_json::from_str::<Config>(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse config: {}", e);
            std::process::exit(1);
        },
    }
}

fn main() {
    let args = clap::App::new("enigma-cli")
        .version("0.1")
        .author("Sirui Mu <msrlancern@126.com>")
        .about("Enigma emulator")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Path to the Enigma machine configuration file")
            .takes_value(true)
            .required(true))
        .arg(clap::Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Path to the file containing input data")
            .takes_value(true)
            .required(true))
        .arg(clap::Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Path to the file containing output data")
            .takes_value(true)
            .required(true))
        .get_matches();

    let config_path = PathBuf::from(String::from(args.value_of("config").unwrap()));
    let config = load_config(&config_path);
    let mut machine = config.create_enigma();

    let input_file_path = PathBuf::from(String::from(args.value_of("input").unwrap()));
    let input_content = match std::fs::read_to_string(input_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read input file: {}", e);
            std::process::exit(1);
        },
    };

    let output_content = machine.map(&input_content);

    let output_file_path = PathBuf::from(String::from(args.value_of("output").unwrap()));
    match std::fs::write(output_file_path, output_content) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write output file: {}", e);
            std::process::exit(1);
        },
    };

    println!("Transformed contents have been saved to output file.");
}
