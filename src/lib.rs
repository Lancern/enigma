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

    /// Map the specified input rune to output rune, but do not advance the rotators.
    pub fn map_rune_static(&self, mut input: Rune) -> Rune {
        input = self.plug.map(input);
        input = self.rotators.map_forward(input);
        input = self.reflector.map(input);
        input = self.rotators.map_backward(input);
        input = self.plug.map(input);

        input
    }

    /// Map the specified input rune to output rune.
    pub fn map_rune(&mut self, input: Rune) -> Rune {
        let ret = self.map_rune_static(input);
        self.advance_rotators();
        ret
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

    /// Manually advance the rotators by one step.
    pub fn advance_rotators(&mut self) {
        self.rotators.advance();
    }
}
