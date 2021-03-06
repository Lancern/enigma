//! This module provides implementation of the plug board component in an Enigma machine.
//!
//! The plug board can be regarded as a permutation of runes whose longest cycle is no longer than
//! 2.
//!
//! The plug board is implemented as the [`PlugBoard`] type. The operations available on
//! [`PlugBoard`] is similar to those on [`Reflector`]. Please consume the corresponding
//! documentation for more information.
//!
//! [`PlugBoard`]: struct.PlugBoard.html
//! [`Reflector`]: ../../reflector/struct.Reflector.html
//!

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::math::Permutation;
use crate::utils::{Rune, RUNE_SET_SIZE};

/// Error indicating that the permutation specified to create a PlugBoard is invalid.
#[derive(Clone, Copy, Debug)]
pub struct InvalidPlugBoardPermutationError;

impl Display for InvalidPlugBoardPermutationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid plug board permutation")
    }
}

impl Error for InvalidPlugBoardPermutationError { }

/// A plug board.
///
/// A plug board can be regarded as a rune permutation whose longest cycle is no longer than 2.
#[derive(Clone, Debug)]
pub struct PlugBoard {
    perm: Permutation,
}

impl PlugBoard {
    /// Create a plug board from the specified permutation.
    ///
    /// The specified permutation should meet the following requirements:
    /// - Its size should be `RUNE_SET_SIZE`;
    /// - The length of the longest cycle within the permutation should be no larger than 2.
    pub fn from_perm(perm: Permutation) -> Result<Self, InvalidPlugBoardPermutationError> {
        if perm.n() != RUNE_SET_SIZE {
            return Err(InvalidPlugBoardPermutationError);
        }

        if perm.max_cycle_len() > 2 {
            return Err(InvalidPlugBoardPermutationError);
        }

        Ok(Self { perm })
    }

    /// Create a plug board from the specified permutation, without any sanity checks.
    ///
    /// Users should avoid using this function. Instead, call the `from_perm` function.
    pub unsafe fn from_perm_unchecked(perm: Permutation) -> Self {
        Self { perm }
    }

    /// Map the specified input rune to the output rune.
    pub fn map(&self, input: Rune) -> Rune {
        unsafe {
            Rune::from_value_unchecked(self.perm.map(input.value()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::components::tests::*;

    mod plug_board_tests {
        use super::*;

        #[test]
        fn test_from_perm_valid() {
            let perm = create_test_perm_builder().build();
            assert!(PlugBoard::from_perm(perm).is_ok());
        }

        #[test]
        fn test_from_perm_invalid_size() {
            let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
            assert!(PlugBoard::from_perm(perm).is_err());
        }

        #[test]
        fn test_from_perm_invalid_cycle() {
            let perm = create_test_perm_builder()
                .swap(0, 2)
                .build();
            assert!(PlugBoard::from_perm(perm).is_err());
        }

        #[test]
        fn test_map() {
            let board = PlugBoard::from_perm(create_test_perm_builder().build()).unwrap();
            assert_eq!(board.map(Rune::from_char('a').unwrap()), 'b');
            assert_eq!(board.map(Rune::from_char('c').unwrap()), 'd');
        }
    }
}
