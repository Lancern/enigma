//! This crate provides an implementation of the Enigma machine.
//!

pub mod components;
pub mod math;
pub mod utils;

pub use crate::components::*;
pub use crate::utils::Rune;

// An Enigma machine.
pub struct Enigma {
    plug: PlugBoard,
    rotators: RotatorGroup,
    reflector: Reflector,
}

impl Enigma {
    /// Create a new Enigma machine with its components.
    pub fn new(plug: PlugBoard, rotators: RotatorGroup, reflector: Reflector) -> Self {
        Self { plug, rotators, reflector }
    }

    /// Map the specified input rune to output rune.
    pub fn map_rune(&mut self, mut input: Rune) -> Rune {
        input = self.plug.map(input);
        input = self.rotators.map_forward(input);
        input = self.reflector.map(input);
        input = self.rotators.map_backward(input);
        input = self.plug.map(input);

        self.rotators.advance();

        input
    }

    /// Map all runes within the specified string to output rune and returns all output runes as a
    /// string.
    pub fn map_str(&mut self, s: &str) -> String {
        let mut output = String::new();
        for ch in s.chars() {
            match Rune::from_char(ch) {
                Ok(rune) => output.push(self.map_rune(rune).into_char()),
                _ => (),
            };
        }
        output
    }
}
