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

## License

This repository is open-sourced under [MIT License](./LICENSE).
