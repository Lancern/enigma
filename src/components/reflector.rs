//! This module provides the implementation of the reflector component within an Enigma machine.
//!
//! Reflector is a very clever design of the Enigma machine. It allows the encryption and decryption
//! process to use the same suite of machanics.
//!
//! In this implementation, reflectors are represented by the [`Reflector`] type. A reflector can be
//! viewed as a Rune permutation that does not have any fixed points. This property ensures that
//! Enigma machine cannot map input runes to the same output runes, which is one of Enigma machine's
//! vulnerabilities. Also, the length of the longest cycle within the permutation should be 2.
//!
//! Reflectors can be created using the `from_perm` associate function:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::Permutation;
//! #
//! let reflector = Reflector::from_perm(
//!     Permutation::from_perm(vec![2u8, 3u8, 0u8, 1u8]).unwrap()
//! ).unwrap();
//! ```
//!
//! If the permutation given to `from_perm` does not meet the requirements introduced above,
//! `from_perm` will fail:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::Permutation;
//! #
//! // perm has fixed point
//! let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
//! assert!(Reflector::from_perm(perm).is_err());
//!
//! // The length of the longest cycle within perm is not 2.
//! let perm = Permutation::from_perm(vec![2u8, 3u8, 1u8, 0u8]).unwrap();
//! assert!(Reflector::from_perm(perm).is_err());
//! ```
//!
//! To map (encrypt / decrypt) an input Rune to the corresponding output Rune produced by a
//! reflector, use the `map` associate function:
//!
//! ```
//! # use enigma::components::reflector::Reflector;
//! # use enigma::math::Permutation;
//! # use enigma::utils::Rune;
//! #
//! let reflector = Reflector::from_perm(
//!     Permutation::from_perm(vec![2u8, 3u8, 0u8, 1u8]).unwrap()
//! ).unwrap();
//!
//! assert_eq!(reflector.map(Rune::from_char('b').unwrap()), 'd');
//! ```
//!
//! [`Reflector`]: struct.Reflector.html

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::math::Permutation;
use crate::utils::Rune;

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

    mod reflector_tests {
        use super::*;

        #[test]
        fn test_from_perm_valid() {
            let perm = Permutation::from_perm(vec![2u8, 3u8, 0u8, 1u8]).unwrap();
            assert!(Reflector::from_perm(perm).is_ok());
        }

        #[test]
        fn test_from_perm_invalid() {
            // The permutation should not have any fixed points
            let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
            assert!(Reflector::from_perm(perm).is_err());

            // The length of the longest cycle within the permutation should be 2.
            let perm = Permutation::from_perm(vec![2u8, 3u8, 1u8, 0u8]).unwrap();
            assert!(Reflector::from_perm(perm).is_err());
        }

        #[test]
        fn test_map() {
            let reflector = Reflector::from_perm(
                Permutation::from_perm(vec![2u8, 3u8, 0u8, 1u8]).unwrap()
            ).unwrap();
            assert_eq!(reflector.map(Rune::from_char('a').unwrap()), 'c');
            assert_eq!(reflector.map(Rune::from_char('b').unwrap()), 'd');
            assert_eq!(reflector.map(Rune::from_char('c').unwrap()), 'a');
            assert_eq!(reflector.map(Rune::from_char('d').unwrap()), 'b');
        }
    }
}
