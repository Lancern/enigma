extern crate enigma;

extern crate clap;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Config {
    rotators: [String; 3],
    reflector: Vec<[char; 2]>,
    secret_headers: String,
}

fn main() {

}
