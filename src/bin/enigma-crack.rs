extern crate enigma;

extern crate clap;
extern crate serde;
extern crate serde_json;

use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::iter::FromIterator;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};

use enigma::{Enigma, Reflector, Rotator, RotatorGroup, PlugBoard};
use enigma::math::{Permutation, PermutationBuilder};
use enigma::utils::{Rune, RUNE_SET_SIZE};

use serde::Deserialize;

#[derive(Clone, Debug)]
struct InvalidConfigError;

impl Display for InvalidConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid configuration")
    }
}

impl Error for InvalidConfigError { }

#[derive(Clone, Debug, Deserialize)]
struct Config {
    rotators: [String; 3],
    reflector: Vec<[char; 2]>,
    secret_headers: String,
}

impl Config {
    fn create_rotator(&self, index: usize) -> Rotator {
        let perm = match create_permutation_from_string(&self.rotators[index]) {
            Ok(perm) => perm,
            Err(e) => {
                eprintln!("invalid rotator setting: {}", e);
                std::process::exit(1);
            },
        };

        match Rotator::new(perm, 0) {
            Ok(rotator) => rotator,
            Err(e) => {
                eprintln!("invalid rotator setting: {}", e);
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
                eprintln!("invalid reflector setting: {}", e);
                std::process::exit(1);
            },
        };

        match Reflector::from_perm(perm) {
            Ok(reflector) => reflector,
            Err(e) => {
                eprintln!("invalid reflector setting: {}", e);
                std::process::exit(1);
            },
        }
    }

    fn create_enigma(&self) -> Enigma {
        let plug_board = PlugBoard::from_perm(
            PermutationBuilder::new(RUNE_SET_SIZE).build()
        ).unwrap();
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

        let lhs = sw[0].to_ascii_lowercase() as u8 - 'a' as u8;
        let rhs = sw[1].to_ascii_lowercase() as u8 - 'a' as u8;
        builder = builder.swap(lhs, rhs);
    }

    Ok(builder.build())
}

fn create_permutation_from_string(s: &str) -> Result<Permutation, InvalidConfigError> {
    let mut perm: Vec<u8> = Vec::with_capacity(s.len());

    for ch in s.chars() {
        if !ch.is_ascii_alphabetic() {
            return Err(InvalidConfigError::new(
                format!("{} is not an ASCII alphabetic character", ch)));
        }

        perm.push(ch.to_ascii_lowercase() as u8 - 'a' as u8);
    }

    Permutation::from_perm(perm)
        .map_err(|e| InvalidConfigError::new(
            format!("invalid permutation: {}", e)))
}

fn load_config(path: &Path) -> Config {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read configuration file: {}", e);
            std::process::exit(1);
        },
    };

    match serde_json::from_str::<Config>(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to parse configuration: {}", e);
            std::process::exit(1);
        },
    }
}

type CycleDecomposition = Vec<usize>;

fn get_cycle_decomposition(perm: &Permutation) -> CycleDecomposition {
    let mut ret = Vec::new();
    let mut visited = vec![false; perm.len()];

    for i in 0..perm.len() {
        if visited[i] {
            continue;
        }

        let mut cycle_len = 1;
        let mut j = perm[i] as usize;
        while j != i {
            visited[j] = true;
            cycle_len += 1;
            j = perm[j] as usize;
        }

        ret.push(cycle_len);
    }

    ret
}

fn get_all_rotator_reflector_perm(config: &Config) -> Vec<Permutation> {
    let num_perms = (RUNE_SET_SIZE as usize) * (RUNE_SET_SIZE as usize) * (RUNE_SET_SIZE as usize);
    let mut machine = config.create_enigma();
    let mut ret = Vec::with_capacity(num_perms);

    for _ in 0..num_perms {
        let mut perm_vec = Vec::with_capacity(RUNE_SET_SIZE as usize);
        for i in 0..RUNE_SET_SIZE {
            let mapped = machine.map_rune_static(Rune::from_value(mapped).unwrap()).value();
            perm_vec.push(mapped);
        }

        let perm = Permutation::from_perm(perm_vec).unwrap();
        ret.push(perm);

        machine.advance_rotators();
    }

    ret
}

fn combine_permutations(lhs: &Permutation, rhs: &Permutation) -> Permutation {
    let mut ret = Vec::with_capacity(RUNE_SET_SIZE as usize);
    for i in 0..RUNE_SET_SIZE {
        let output = rhs.map(lhs.map(i));
        ret[i as usize] = output;
    }

    Permutation::from_perm(ret).unwrap()
}

fn load_secret_headers(path: &Path) -> Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open secret header file: {}", e);
            std::process::exit(1);
        },
    };
    let mut reader = BufReader::new(file);
    let mut ret = Vec::new();
    for ln in reader.lines().map(|l| l.unwrap().trim()) {
        if ln.len() != 6 {
            eprintln!("Failed to parse secret header file: some line has a length other than 6");
            std::process::exit(1);
        }

        let mut s = ln.to_ascii_lowercase();
        for ch in s.chars() {
            if !ch.is_ascii_alphabetic() {
                eprintln!("Failed to parse secret header file: non-alphabetic characters found");
                std::process::exit(1);
            }
        }

        ret.push(s);
    }

    ret
}

fn get_secret_permutations(secret_headers: &Vec<String>) -> [Permutation; 3] {
    let mut secret_perm_buf = [
        vec![-1i8; RUNE_SET_SIZE as usize],
        vec![-1i8; RUNE_SET_SIZE as usize],
        vec![-1i8; RUNE_SET_SIZE as usize],
    ];
    for secret in &secret_headers {
        let secret_bytes = secret.as_bytes();
        for i in 0..3usize {
            let input = secret_bytes[i];
            let output = secret_bytes[i + 3];

            if secret_perm_buf[i][input as usize] != -1 {
                if secret_perm_buf[i][input as usize] != output as i8 {
                    eprintln!("Error: conflicting secret permutation");
                    std::process::exit(1);
                }
            }

            secret_perm_buf[i][input as usize] = output;
        }
    }

    for p in &mut secret_perm_buf {
        if p.contains(&-1i8) {
            eprintln!("Error: too little secrets to crack");
            std::process::exit(1);
        }
    }

    secret_perm_buf.iter().map(
        |buf| Permutation::from(buf.iter().map(|x| *x as u8).collect()).unwrap()
    ).collect()
}

fn is_cycle_decomposition_eq(lhs: &CycleDecomposition, rhs: &CycleDecomposition) -> bool {
    HashSet::from_iter(lhs.iter().copied()) == HashSet::from_iter(rhs.iter().copied())
}

fn main() {
    let args = clap::App::new("enigma-crack")
        .about("Crack Enigma machine with Marian Rejewski's method")
        .arg(clap::Arg::with_name("config")
            .short("c")
            .long("config")
            .takes_value(true)
            .required(true)
            .help("Path to the configuration file"))
        .get_matches();

    let config = load_config(&PathBuf::from(String::from(args.value_of("config").unwrap())));
    let secret_headers = load_secret_headers(&PathBuf::from(&config.secret_headers));

    println!("Generating all permutations and their corresponding cycle decomposition");
    let all_perms = get_all_rotator_reflector_perm(&config);

    let mut merged_perms = Vec::with_capacity(all_perms.len());
    for i in 0..all_perms.len() {
        let j = (i + 3) % all_perms.len();
        merged_perms.push(combine_permutations(&all_perms[i], &all_perms[j]));
    }

    let merged_perms_decomp: Vec<CycleDecomposition> =
        merged_perms.iter().map(|p| get_cycle_decomposition(p)).collect();

    println!("Analyzing cycles in secret headers");
    let secret_perms = get_secret_permutations(&secret_headers);
    let secret_perms_decomp: [CycleDecomposition; 3] = secret_perms.iter().map(
        |p| get_cycle_decomposition(p)
    ).collect();

    println!("Matching existing cycles dictionary");
    let mut possible_settings = Vec::new();
    for i in 0..merged_perms.len() {
        let j = (i + 1) % merged_perms.len();
        let k = (i + 2) % merged_perms.len();
        if is_cycle_decomposition_eq(&merged_perms_decomp[i], &secret_perms_decomp[0]) &&
            is_cycle_decomposition_eq(&merged_perms_decomp[j], &secret_perms_decomp[1]) &&
            is_cycle_decomposition_eq(&merged_perms_decomp[k], &secret_perms_decomp[2]) {
            possible_settings.push(i);
            println!("Found possible settings: {}", i);
        }
    }

    println!("All possible settings: {:?}", possible_settings);
}
