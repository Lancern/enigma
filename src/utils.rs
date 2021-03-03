//! This module provides various utilities needed by other modules in the crate.
//!
//! Specifically, this module provides the following components:
//! - Runes
//!
//! # Runes
//!
//! Runes are individual characters that can be processed by the Enigma machine. When encrypting or
//! decrypting messages, the Enigma machine reads a sequence of runes and outputs a sequence of
//! runes with encryption / decryption transformation performed.
//!
//! In this implementation, runes are **case-insensitive** English letters. Runes are defined by the
//! [`Rune`] type.
//!
//! ## Rune Conversions
//!
//! One can create a rune from a `char` or an ASCII character. To create a rune from a `char`, use
//! the `from_char` associate function:
//!
//! ```
//! # use enigma::utils::Rune;
//! #
//! let ch = 'a';
//! assert_eq!(Rune::from_char(ch).unwrap(), 'a');
//! ```
//!
//! Note that if the input `char` is not an English letter, then the conversion will fail:
//!
//! ```
//! # use enigma::utils::Rune;
//! #
//! let ch = '0';
//! assert!(Rune::from_char(ch).is_err());
//! ```
//!
//! To create a rune from an ASCII character represented by a `u8`, use the `from_ascii` associate
//! function:
//!
//! ```
//! # use enigma::utils::Rune;
//! #
//! let ch = b'a';
//! assert_eq!(Rune::from_ascii(ch).unwrap(), 'a');
//! ```
//!
//! ## Rune Internals
//!
//! Internally, runes are represented by a `u8` that indicates the index of the represented English
//! letter. For instance, `'a'` is represented as `0`, `'b'` is represented as `'1'`, etc.. To
//! convert a `Rune` from / to the letter index, use the `from_value` / `value` associate
//! functions:
//!
//! ```
//! # use enigma::utils::Rune;
//! #
//! assert_eq!(Rune::from_value(2).unwrap(), 'c');
//! assert!(Rune::from_value(26).is_err());
//!
//! let rune = Rune::from_char('b').unwrap();
//! assert_eq!(rune.value(), 1);
//! ```
//!
//! ## Rune Operations
//!
//! [`Rune`] implements `Copy`, `Eq` and `Ord`.
//!
//! [`Rune`]: struct.Rune.html
//!

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter, Write};

/// Error indicating that the value of a rune is out of range.
#[derive(Clone, Copy, Debug)]
pub struct RuneOutOfRangeError;

impl Display for RuneOutOfRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("rune value is out of range")
    }
}

impl Error for RuneOutOfRangeError { }

const RUNE_VALUE_MAX: u8 = 25;

/// A rune.
///
/// Runes are individual characters that can be processed by the Enigma machine.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rune {
    value: u8,
}

impl Rune {
    /// Create a rune from the specified English letter index.
    pub fn from_value(value: u8) -> Result<Self, RuneOutOfRangeError> {
        if value > RUNE_VALUE_MAX {
            return Err(RuneOutOfRangeError)
        }

        Ok(Self { value })
    }

    /// Create a rune from the specified English letter index without sanity check.
    ///
    /// Usage of this function is strongly discouraged. Please use the `from_value` function
    /// instead.
    pub unsafe fn from_value_unchecked(value: u8) -> Self {
        Self { value }
    }

    /// Get the index of the English letter represented by this rune.
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Create a rune from the specified character.
    pub fn from_char(mut value: char) -> Result<Self, RuneOutOfRangeError> {
        if !value.is_ascii_alphabetic() {
            return Err(RuneOutOfRangeError)
        }

        if value.is_ascii_lowercase() {
            value = value.to_ascii_uppercase()
        }

        Ok(unsafe { Self::from_value_unchecked(value as u8 - b'A') })
    }

    /// Convert this rune into corresponding English letter character.
    pub fn into_char(self) -> char {
        self.into_ascii() as char
    }

    /// Convert this rune into a one-character-long string that consists of the represented English
    /// letter.
    pub fn into_string(self) -> String {
        String::from(self.into_char())
    }

    /// Convert the specified ASCII character into a rune.
    pub fn from_ascii(value: u8) -> Result<Self, RuneOutOfRangeError> {
        Self::from_char(value as char)
    }

    /// Convert this rune into corresponding English letter in ASCII character.
    pub fn into_ascii(self) -> u8 {
        self.value + b'A'
    }
}

impl Display for Rune {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char((*self).into())
    }
}

impl PartialEq<char> for Rune {
    fn eq(&self, other: &char) -> bool {
       self.into_char() == other.to_ascii_uppercase()
    }
}

impl PartialEq<Rune> for char {
    fn eq(&self, other: &Rune) -> bool {
        self.to_ascii_uppercase() == other.into_char()
    }
}

impl Into<char> for Rune {
    fn into(self) -> char {
        self.into_ascii() as char
    }
}

impl Into<String> for Rune {
    fn into(self) -> String {
        self.into_string()
    }
}

impl TryFrom<char> for Rune {
    type Error = RuneOutOfRangeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_char(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod rune_tests {
        use super::*;

        #[test]
        fn test_from_value_valid() {
            let rune = Rune::from_value(2).unwrap();
            assert!(rune == 'c');
        }

        #[test]
        fn test_from_value_invalid() {
            assert!(Rune::from_value(26).is_err());
            assert!(Rune::from_value(b'c').is_err());
        }

        #[test]
        fn test_value() {
            let rune = Rune::from_value(2).unwrap();
            assert_eq!(rune.value(), 2);
        }

        #[test]
        fn test_from_char_valid() {
            let rune = Rune::from_char('c').unwrap();
            assert_eq!(rune, 'c');

            let rune = Rune::from_char('B').unwrap();
            assert_eq!(rune, 'b');
        }

        #[test]
        fn test_from_char_invalid() {
            assert!(Rune::from_char('2').is_err());
        }

        #[test]
        fn test_into_char() {
            let rune = Rune::from_value(3).unwrap();
            assert_eq!(rune.into_char(), 'D');
        }

        #[test]
        fn test_into_string() {
            let rune = Rune::from_value(2).unwrap();
            assert_eq!(rune.into_string(), String::from("C"));
        }

        #[test]
        fn test_from_ascii_valid() {
            let rune = Rune::from_ascii(b'c').unwrap();
            assert_eq!(rune, 'c');

            let rune = Rune::from_ascii(b'B').unwrap();
            assert_eq!(rune, 'b');
        }

        #[test]
        fn test_from_ascii_invalid() {
            assert!(Rune::from_ascii(b'0').is_err());
        }

        #[test]
        fn test_into_ascii() {
            let rune = Rune::from_value(2).unwrap();
            assert_eq!(rune.into_ascii(), b'C');
        }

        #[test]
        fn test_eq_to_char() {
            let rune = Rune::from_value(2).unwrap();
            assert_eq!(rune, 'C');
            assert_eq!(rune, 'c');
        }
    }
}
