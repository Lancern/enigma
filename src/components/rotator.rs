//! This module implements the rotator mechanics in the Enigma machine.
//!
//! Rotator can be viewed as a rune permutation together with an offset value. When mapping input
//! runes to output runes, the offset value is added to the input rune before it is mapped via the
//! permutation. The offset value will increase by one (advance), effectively changing the
//! permutation. The Enigma machine relies on the rotators to generate a completely different
//! rune mapping each time a rune is to be encrypted / decrypted.
//!
//! # Rotator
//!
//! The rotators are implemented as the [`Rotator`] type. The operations available on a rotator is
//! similar to those on a reflector. Unlike a reflector or a plug board, the length of the longest
//! cycle within the permutation used for creating a rotator can be greater than 2. [`Rotator`] will
//! automatically calculate the inverse permutation used for inverse mapping.
//!
//! To map the input rune with the permutation specified when creating the rotator, call the
//! `map_forward` associate function. To map the input rune with the inverse permutation, call the
//! `map_backward` associate function. These two  associate functions will not automatically advance
//! the internal offset. To advance the internal offset, call the `advance` function.
//!
//! # Rotator Group
//!
//! Each Enigma machine contains 3 rotators. These 3 rotators are grouped together in a way that
//! their offsets are "chained". When advancing offsets, the offset of the first rotator is
//! advanced. If the offset goes from `RUNE_MAX_VALUE` to `0`, then the offset of the second rotator
//! is advanced. The same rule applies for the second and the third rotators in a rotator group.
//!
//! [`Rotator`]: struct.Rotator.html
//!

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::math::Permutation;
use crate::utils::{Rune, RUNE_SET_SIZE};

/// Error indicating that the permutation specified to create a rotator is invalid.
#[derive(Clone, Copy, Debug)]
pub struct InvalidRotatorPermutationError;

impl Display for InvalidRotatorPermutationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid rotator permutation")
    }
}

impl Error for InvalidRotatorPermutationError { }

/// A rotator.
///
/// A plug board can be regarded as a rune permutation whose longest cycle is no longer than 2,
/// together with an offset value to be applied to the input rune before permutation substitution.
#[derive(Clone, Debug)]
pub struct Rotator {
    perm_forward: Permutation,
    perm_backward: Permutation,
    offset: u8,
}

impl Rotator {
    /// Create a new rotator from the specified permutation as its forward permutation and the
    /// specified offset.
    ///
    /// The specified permutation should meet the following requirements:
    /// - Its size should be `RUNE_SET_SIZE`.
    pub fn new(perm: Permutation, offset: u8) -> Result<Self, InvalidRotatorPermutationError> {
        if perm.n() != RUNE_SET_SIZE {
            return Err(InvalidRotatorPermutationError);
        }

        let perm_backward = perm.inverse();

        Ok(Self {
            perm_forward: perm,
            perm_backward,
            offset: offset % RUNE_SET_SIZE,
        })
    }

    /// Create a new rotator from the specified permutation and offset value, without sanity checks.
    ///
    /// Users should avoid using this function. Instead, call the `from_perm` function.
    pub unsafe fn new_unchecked(perm: Permutation, offset: u8) -> Self {
        let perm_backward = perm.inverse();
        Self {
            perm_forward: perm,
            perm_backward,
            offset: offset % RUNE_SET_SIZE,
        }
    }

    /// Map the specified input rune to output rune.
    pub fn map_forward(&self, input: Rune) -> Rune {
        self.map(&self.perm_forward, input)
    }

    pub fn map_backward(&self, input: Rune) -> Rune {
        self.map(&self.perm_backward, input)
    }

    /// Advance the underlying offset value.
    pub fn advance(&mut self) -> bool {
        self.offset = (self.offset + 1) % RUNE_SET_SIZE;
        self.offset != 0
    }

    fn map(&self, perm: &Permutation, input: Rune) -> Rune {
        let input_value = (input.value() + self.offset) % RUNE_SET_SIZE;
        let mut mapped_value = perm.map(input_value);

        if mapped_value >= self.offset {
            mapped_value -= self.offset;
        } else {
            mapped_value = mapped_value + RUNE_SET_SIZE - self.offset;
        }

        unsafe {
            Rune::from_value_unchecked(mapped_value)
        }
    }
}

/// A rotator group that chains the 3 rotators within an Enigma machine.
///
/// When mapping input runes, the input rune is passed into a transformation pipeline formed by the
/// 3 rotators within the group.
///
/// The offsets of the 3 rotators are also chained. When advancing, the offset of the first rotator
/// is advanced. If it rolls back from `RUNE_SET_SIZE - 1` to `0`, then the offset of the second
/// rotator is advanced. This rule applies to the second and third rotator within the group.
#[derive(Clone, Debug)]
pub struct RotatorGroup {
    rotators: [Rotator; 3],
}

impl RotatorGroup {
    /// Create a new rotator group that chains the specified 3 rotators.
    pub fn new(rotators: [Rotator; 3]) -> Self {
        Self { rotators }
    }

    /// Map the input rune to output rune in the forward direction.
    pub fn map_forward(&self, mut input: Rune) -> Rune {
        for r in &self.rotators {
            input = r.map_forward(input);
        }
        input
    }

    /// Map the input rune to output rune in the backward direction.
    pub fn map_backward(&self, mut input: Rune) -> Rune {
        for r in self.rotators.iter().rev() {
            input = r.map_backward(input);
        }
        input
    }

    /// Advance the offsets of the 3 rotators within the group, with the rules described in the
    /// `RotatorGroup` documentation.
    pub fn advance(&mut self) {
        for r in &mut self.rotators {
            if r.advance() {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::components::tests::*;

    fn create_test_perm_builder_shift() -> PermutationBuilder {
        use crate::utils::RUNE_VALUE_MAX;

        let mut builder = PermutationBuilder::new(RUNE_SET_SIZE);
        for i in 0..RUNE_VALUE_MAX {
            builder = builder.swap(i, i + 1);
        }

        builder
    }

    mod rotator_tests {
        use super::*;

        #[test]
        fn test_from_perm_valid() {
            let perm = create_test_perm_builder_shift().build();
            assert!(Rotator::new(perm, 0).is_ok());
        }

        #[test]
        fn test_from_perm_invalid_size() {
            let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
            assert!(Rotator::new(perm, 0).is_err());
        }

        #[test]
        fn test_map_forward() {
            let perm = create_test_perm_builder_shift().build();
            let rotator = Rotator::new(perm, 0).unwrap();
            assert_eq!(rotator.map_forward(Rune::from_char('a').unwrap()), 'b');
            assert_eq!(rotator.map_forward(Rune::from_char('b').unwrap()), 'c');
            assert_eq!(rotator.map_forward(Rune::from_char('c').unwrap()), 'd');
        }

        #[test]
        fn test_map_backward() {
            let perm = create_test_perm_builder_shift().build();
            let rotator = Rotator::new(perm, 0).unwrap();
            assert_eq!(rotator.map_backward(Rune::from_char('a').unwrap()), 'z');
            assert_eq!(rotator.map_backward(Rune::from_char('b').unwrap()), 'a');
            assert_eq!(rotator.map_backward(Rune::from_char('c').unwrap()), 'b');
        }

        #[test]
        fn test_advance_once() {
            let perm = create_test_perm_builder().build();
            let mut rotator = Rotator::new(perm, 0).unwrap();

            rotator.advance();
            assert_eq!(rotator.offset, 1);

            assert_eq!(rotator.map_forward(Rune::from_char('a').unwrap()), 'z');
            assert_eq!(rotator.map_forward(Rune::from_char('b').unwrap()), 'c');

            assert_eq!(rotator.map_backward(Rune::from_char('a').unwrap()), 'z');
            assert_eq!(rotator.map_backward(Rune::from_char('b').unwrap()), 'c');
        }

        #[test]
        fn test_advance_scroll_back() {
            use crate::utils::RUNE_VALUE_MAX;

            let perm = create_test_perm_builder_shift().build();
            let mut rotator = Rotator::new(perm, 0).unwrap();

            for _ in 0..RUNE_VALUE_MAX {
                assert!(rotator.advance());
            }

            assert!(!rotator.advance());
            assert_eq!(rotator.offset, 0);
        }
    }

    mod rotator_group_tests {
        use super::*;
        use crate::utils::RUNE_VALUE_MAX;

        fn create_test_group() -> RotatorGroup {
            let perm = create_test_perm_builder_shift().build();
            RotatorGroup::new([
                Rotator::new(perm.clone(), 0).unwrap(),
                Rotator::new(perm.clone(), 0).unwrap(),
                Rotator::new(perm.clone(), 0).unwrap(),
            ])
        }

        #[test]
        fn test_map_forward() {
            let group = create_test_group();
            assert_eq!(group.map_forward(Rune::from_char('a').unwrap()), 'd');
            assert_eq!(group.map_forward(Rune::from_char('b').unwrap()), 'e');
        }

        #[test]
        fn test_map_backward() {
            let group = create_test_group();
            assert_eq!(group.map_backward(Rune::from_char('a').unwrap()), 'x');
            assert_eq!(group.map_backward(Rune::from_char('b').unwrap()), 'y');
        }

        #[test]
        fn test_advance() {
            let mut group = create_test_group();

            group.advance();
            assert_eq!(group.rotators[0].offset, 1);
            assert_eq!(group.rotators[1].offset, 0);
            assert_eq!(group.rotators[2].offset, 0);

            while group.rotators[0].offset != RUNE_VALUE_MAX {
                group.advance();
            }
            group.advance();
            assert_eq!(group.rotators[0].offset, 0);
            assert_eq!(group.rotators[1].offset, 1);
            assert_eq!(group.rotators[2].offset, 0);

            while group.rotators[1].offset != RUNE_VALUE_MAX ||
                group.rotators[0].offset != RUNE_VALUE_MAX {
                group.advance();
            }
            group.advance();
            assert_eq!(group.rotators[0].offset, 0);
            assert_eq!(group.rotators[1].offset, 0);
            assert_eq!(group.rotators[2].offset, 1);
        }
    }
}
