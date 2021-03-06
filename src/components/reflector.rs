//! This module provides the implementation of the reflector component within an Enigma machine.
//!
//! Reflector is a very clever design of the Enigma machine. It allows the encryption and decryption
//! process to use the same suite of machanics.
//!
//! In this implementation, reflectors are represented by the [`Reflector`] type. A reflector can be
//! viewed as a Rune permutation that does not have any fixed points. This property ensures that
//! Enigma machine cannot map input runes to the same output runes, which is one of Enigma machine's
//! vulnerabilities. Also, the length of the longest cycle within the permutation should be 2.
//! Finally, the size of the permutation should be equal to `RUNE_SET_SIZE`.
//!
//! Reflectors can be created using the `from_perm` associate function:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::{Permutation, PermutationBuilder};
//! # use enigma::utils::RUNE_SET_SIZE;
//! #
//! let perm = PermutationBuilder::new(RUNE_SET_SIZE)
//!     .swap(0, 1).swap(2, 3).swap(4, 5).swap(6, 7).swap(8, 9)
//!     .swap(10, 11).swap(12, 13).swap(14, 15).swap(16, 17).swap(18, 19)
//!     .swap(20, 21).swap(22, 23).swap(24, 25)
//!     .build();
//! let reflector = Reflector::from_perm(perm).unwrap();
//! ```
//!
//! If the permutation given to `from_perm` does not meet the requirements introduced above,
//! `from_perm` will fail:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::{Permutation, PermutationBuilder};
//! # use enigma::utils::RUNE_SET_SIZE;
//! #
//! // The size of perm is not `RUNE_SET_SIZE`.
//! let perm = Permutation::from_perm(vec![1u8, 0u8, 3u8, 2u8]).unwrap();
//! assert!(Reflector::from_perm(perm).is_err());
//!
//! // perm has fixed point
//! let perm = PermutationBuilder::new(RUNE_SET_SIZE).build();
//! assert!(Reflector::from_perm(perm).is_err());
//!
//! // The length of the longest cycle within perm is not 2.
//! let perm = PermutationBuilder::new(RUNE_SET_SIZE)
//!     .swap(0, 1).swap(2, 3).swap(4, 5).swap(6, 7).swap(8, 9)
//!     .swap(10, 11).swap(12, 13).swap(14, 15).swap(16, 17).swap(18, 19)
//!     .swap(20, 21).swap(22, 23).swap(24, 25)
//!     .swap(0, 2)   // This will introduce a cycle whose length is 4
//!     .build();
//! assert!(Reflector::from_perm(perm).is_err());
//! ```
//!
//! To map (encrypt / decrypt) an input Rune to the corresponding output Rune produced by a
//! reflector, use the `map` associate function:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::{Permutation, PermutationBuilder};
//! # use enigma::utils::{Rune, RUNE_SET_SIZE};
//! #
//! let perm = PermutationBuilder::new(RUNE_SET_SIZE)
//!     .swap(0, 1).swap(2, 3).swap(4, 5).swap(6, 7).swap(8, 9)
//!     .swap(10, 11).swap(12, 13).swap(14, 15).swap(16, 17).swap(18, 19)
//!     .swap(20, 21).swap(22, 23).swap(24, 25)
//!     .build();
//! let reflector = Reflector::from_perm(perm).unwrap();
//!
//! assert_eq!(reflector.map(Rune::from_char('c').unwrap()), 'd');
//! ```
//!
//! [`Reflector`]: struct.Reflector.html

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::math::Permutation;
use crate::utils::{Rune, RUNE_SET_SIZE};

/// Error indicating that the permutation of a reflector is invalid.
#[derive(Clone, Copy, Debug)]
pub struct InvalidReflectorPermutationError;

impl Display for InvalidReflectorPermutationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid reflector permutation")
    }
}

impl Error for InvalidReflectorPermutationError { }

/// A reflector.
///
/// Reflector can be viewed as a Rune permutation that does not have any fixed points.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reflector {
    perm: Permutation,
}

impl Reflector {
    /// Create a new reflector from the specified permutation.
    ///
    /// The specified permutation should have the following two properties:
    /// - It should not have any fixed points.
    /// - The length of the longest cycle within it should be 2.
    ///
    /// This function performs sanity checks against the conditions above. If any of the conditions
    /// are not satisfied, this function will fail.
    pub fn from_perm(perm: Permutation) -> Result<Self, InvalidReflectorPermutationError> {
        if perm.n() != RUNE_SET_SIZE {
            return Err(InvalidReflectorPermutationError);
        }

        // Checks that perm does not have any fixed points.
        for i in 0..perm.n() {
            if perm.map(i) == i {
                return Err(InvalidReflectorPermutationError);
            }
        }

        // Checks that the length of the longest cycle within perm is 2.
        if perm.max_cycle_len() != 2 {
            return Err(InvalidReflectorPermutationError);
        }

        Ok(Self { perm })
    }

    /// Create a new reflector from the specified permutation without sanity checks. Usage of this
    /// function should be avoided. Use the `from_perm` associate function instead.
    pub unsafe fn from_perm_unchecked(perm: Permutation) -> Self {
        Self { perm }
    }

    /// Get the output rune produced by this reflector that corresponds to the specified input rune.
    pub fn map(&self, input: Rune) -> Rune {
        unsafe {
            Rune::from_value_unchecked(self.perm.map(input.value()))
        }
    }
}

impl TryFrom<Permutation> for Reflector {
    type Error = InvalidReflectorPermutationError;

    fn try_from(perm: Permutation) -> Result<Self, Self::Error> {
        Self::from_perm(perm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::components::tests::*;

    mod reflector_tests {
        use super::*;

        #[test]
        fn test_from_perm_valid() {
            let perm = create_test_perm_builder()
                .build();
            assert!(Reflector::from_perm(perm).is_ok());
        }

        #[test]
        fn test_from_perm_invalid_size() {
            let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
            assert!(Reflector::from_perm(perm).is_err());
        }

        #[test]
        fn test_from_perm_invalid_fixed_point() {
            // The permutation should not have any fixed points
            let perm = PermutationBuilder::new(RUNE_SET_SIZE).build();
            assert!(Reflector::from_perm(perm).is_err());
        }

        #[test]
        fn test_from_perm_invalid_cycle() {
            // The length of the longest cycle within the permutation should be 2.
            let perm = create_test_perm_builder()
                .swap(0, 2)
                .build();
            assert!(Reflector::from_perm(perm).is_err());
        }

        #[test]
        fn test_map() {
            let reflector = Reflector::from_perm(
                create_test_perm_builder().build()
            ).unwrap();
            assert_eq!(reflector.map(Rune::from_char('a').unwrap()), 'b');
            assert_eq!(reflector.map(Rune::from_char('b').unwrap()), 'a');
            assert_eq!(reflector.map(Rune::from_char('c').unwrap()), 'd');
            assert_eq!(reflector.map(Rune::from_char('d').unwrap()), 'c');
        }
    }
}
