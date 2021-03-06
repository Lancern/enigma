//! This module implements the core components within an Enigma machine, include plug boards,
//! rotators and reflectors.
//!

pub mod plug_board;
pub mod reflector;
pub mod rotator;

pub use plug_board::{InvalidPlugBoardPermutationError, PlugBoard};
pub use reflector::{InvalidReflectorPermutationError, Reflector};
pub use rotator::{InvalidRotatorPermutationError, Rotator};

#[cfg(test)]
mod tests {
    pub use crate::math::PermutationBuilder;
    pub use crate::utils::RUNE_SET_SIZE;

    pub fn create_test_perm_builder() -> PermutationBuilder {
        PermutationBuilder::new(RUNE_SET_SIZE)
            .swap(0, 1).swap(2, 3).swap(4, 5).swap(6, 7).swap(8, 9)
            .swap(10, 11).swap(12, 13).swap(14, 15).swap(16, 17).swap(18, 19)
            .swap(20, 21).swap(22, 23).swap(24, 25)
    }
}
