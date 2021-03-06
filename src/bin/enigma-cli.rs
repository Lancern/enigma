extern crate enigma;

extern crate clap;
extern crate serde_json;

use std::path::Path;

struct Config {
    plug_board: Vec<[char; 2]>,
    rotators: [Vec<char>; 3],
    reflector: Vec<[char; 2]>,
}

fn load_config(path: &Path) -> Config {
    unimplemented!()
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

    unimplemented!()
}
