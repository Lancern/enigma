# enigma

![MIT License](https://img.shields.io/github/license/Lancern/enigma)

This repository contains:

- An Enigma machine emulator;
- A crack procedure that cracks Enigma machine on modern computers.

## Build

To build this project, you need an available Rust toolchain version 1.47+. To
install Rust toolchain, please refer to [rustup](https://rustup.rs/).

Clone the repository to `$ENIGMA_DIR`:

```bash
git clone git@github.com:Lancern/enigma.git $ENIGMA_DIR
cd $ENIGMA_DIR
```

Then build using `cargo`:

```bash
cargo build --release --features binary --bin enigma-cli
```

After successful build, the enigma emulator program `enigma-cli` will be
available under `target/release`.

## Usage

### Enigma Machine Configuration

The Enigma machine's initial configuration is given in a JSON formatted text
file. An example configuration is given in 
[config.example.json](./config.example.json). The full specification is listed
in [docs/Configuration.md](docs/Configuration.md).

### Run Enigma Emulator

CLI usage:

```bash
enigma-cli -c /path/to/config.json \
    -i /path/to/input/text/file.txt \
    -o /path/to/output/text/file.txt
```

There are several constraints on the input file:

- The content of the file should be a valid UTF-8 encoded text string;
- The file should only contain ASCII English letter characters and whitespace
  characters.

### Run Enigma Machine Crack Procedure

> Under construction

## License

This repository is open-sourced under [MIT License](./LICENSE).
