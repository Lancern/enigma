//! This module provides math abstractions needed by other modules of this crate.
//!
//! Specifically, this module provides the following components:
//! - Permutations
//!
//! # Permutations
//!
//! A permutation of size `n` is a rearrangement of the array `[0, 1, 2, ..., n-1]`. Compared to the
//! sorted array, each number is mapped to another number within range and all images of all numbers
//! are distinct. The sorted array is called identity permutation.
//!
//! Permutations are represented by the [`Permutation`] type. To effectively build a permutation,
//! you can use the [`PermutationBuilder`] type. The [`PermutationBuilder`] type builds a
//! permutation iteratively by swapping elements within the permutation.
//!
//! ## Cycles
//!
//! Permutations can be decomposed into
//! [cycles](https://en.wikipedia.org/wiki/Permutation#Cycle_notation). You can use the
//! `max_cycle_len` associate function to calculate the length of the longest cycle within a
//! permutation:
//!
//! ```
//! # use enigma::math::Permutation;
//! #
//! let perm = Permutation::from_perm(vec![0u8, 2u8, 3u8, 1u8]).unwrap();
//! assert_eq!(perm.max_cycle_len(), 3);
//! ```
//!
//! [`Permutation`]: struct.Permutation.html
//! [`PermutationBuilder`]: struct.PermutationBuilder.html

use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error that indicates a permutation is invalid.
#[derive(Debug)]
pub struct InvalidPermutationError;

impl Display for InvalidPermutationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid permutation")
    }
}

impl Error for InvalidPermutationError { }

/// A permutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permutation {
    perm: Vec<u8>,
}

impl Permutation {
    /// Create an identity permutation of the specified size.
    pub fn identity(n: u8) -> Self {
        let perm = identity_perm(n);
        unsafe { Self::from_perm_unchecked(perm) }
    }

    /// Create a new permutation from the specified rearranged array. This function fails if the
    /// specified array does not form a permutation.
    pub fn from_perm(perm: Vec<u8>) -> Result<Self, InvalidPermutationError> {
        if perm.len() > std::u8::MAX as usize {
            return Err(InvalidPermutationError);
        }

        let n = perm.len() as u8;

        // Ensures that all numbers in the permutation are < n.
        for x in &perm {
            if *x >= n {
                return Err(InvalidPermutationError);
            }
        }

        // Ensures that all numbers in the permutation are distinct.
        for i in 0..perm.len() {
            for j in i+1..perm.len() {
                if perm[i] == perm[j] {
                    return Err(InvalidPermutationError);
                }
            }
        }

        Ok(Self { perm })
    }

    /// Create a new permutation from the specified rearranged array without sanity checks. Usage
    /// of this function is strongly discouraged and one should use the `from_perm` associate
    /// function instead.
    pub unsafe fn from_perm_unchecked(perm: Vec<u8>) -> Self {
        Self { perm }
    }

    /// Get the size of the permutation, e.g. the number of elements in the permutation.
    pub fn n(&self) -> u8 {
        self.perm.len() as u8
    }

    /// Get the mapped-to number of the specified element within this permutation.
    ///
    /// This function panics if element is greater than or equal to `n()`.
    pub fn map(&self, element: u8) -> u8 {
        self.perm[element as usize]
    }

    /// Calculates the length of the longest cycle in the specified permutation.
    pub fn max_cycle_len(&self) -> usize {
        let mut visited: Vec<bool> = vec![false; self.perm.len()];
        let mut max_len = 0usize;

        for i in 0..self.perm.len() {
            if visited[i] {
                continue;
            }

            let mut current_len = 0usize;
            let mut j = i;
            while !visited[j] {
                visited[j] = true;
                current_len += 1;
                j = self.perm[j] as usize;
            }

            max_len = std::cmp::max(max_len, current_len);
        }

        max_len
    }
}

impl TryFrom<Vec<u8>> for Permutation {
    type Error = InvalidPermutationError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_perm(value)
    }
}

/// Build permutations with successive swapping operations.
#[derive(Clone, Debug)]
pub struct PermutationBuilder {
    perm: Vec<u8>,
}

impl PermutationBuilder {
    /// Create a new permutation builder that buildes a permutation of the specified size.
    pub fn new(n: u8) -> Self {
        Self {
            perm: identity_perm(n),
        }
    }

    /// Get the size of the permutation under construction.
    pub fn n(&self) -> u8 {
        self.perm.len() as u8
    }

    /// Swap the value at the two specified index in the permutation.
    pub fn swap(mut self, i: u8, j: u8) -> Self {
        let i_idx = i as usize;
        let j_idx = j as usize;

        let tmp = self.perm[i_idx];
        self.perm[i_idx] = self.perm[j_idx];
        self.perm[j_idx] = tmp;

        self
    }

    /// Get the built permutation.
    pub fn build(self) -> Permutation {
        unsafe {
            Permutation::from_perm_unchecked(self.perm)
        }
    }
}

/// Generate an identity permutation of the specified length.
fn identity_perm(n: u8) -> Vec<u8> {
    let mut perm = Vec::with_capacity(n as usize);
    for i in 0..n {
        perm.push(i);
    }
    perm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_perm() {
        assert_eq!(identity_perm(4), vec![0u8, 1u8, 2u8, 3u8]);
    }

    mod permutation_tests {
        use super::*;

        #[test]
        fn test_from_perm_valid() {
            let perm = Permutation::from_perm(vec![0u8, 2u8, 1u8, 3u8]).unwrap();
            assert_eq!(perm, unsafe { Permutation::from_perm_unchecked(vec![0u8, 2u8, 1u8, 3u8]) });
        }

        #[test]
        fn test_from_perm_invalid_range_err() {
            assert!(Permutation::from_perm(vec![0u8, 2u8, 4u8, 3u8]).is_err());
        }

        #[test]
        fn test_from_perm_invalid_duplicate() {
            assert!(Permutation::from_perm(vec![0u8, 2u8, 3u8, 2u8]).is_err());
        }

        #[test]
        fn test_map() {
            let perm = Permutation::from_perm(vec![0u8, 2u8, 1u8, 3u8]).unwrap();
            assert_eq!(perm.map(0), 0);
            assert_eq!(perm.map(1), 2);
            assert_eq!(perm.map(2), 1);
            assert_eq!(perm.map(3), 3);
        }

        #[test]
        fn test_max_cycle_len() {
            let perm = Permutation::from_perm(vec![0u8, 1u8, 2u8, 3u8]).unwrap();
            assert_eq!(perm.max_cycle_len(), 1);

            let perm = Permutation::from_perm(vec![0u8, 2u8, 1u8, 3u8]).unwrap();
            assert_eq!(perm.max_cycle_len(), 2);

            let perm = Permutation::from_perm(vec![1u8, 2u8, 3u8, 0u8]).unwrap();
            assert_eq!(perm.max_cycle_len(), 4);
        }
    }

    mod permutation_builder_tests {
        use super::*;

        #[test]
        fn test_initial_identity() {
            let perm = PermutationBuilder::new(4).build();
            assert_eq!(perm, Permutation::identity(4));
        }

        #[test]
        fn test_swap() {
            let perm = PermutationBuilder::new(4)
                .swap(1, 2)
                .swap(2, 3)
                .build();
            assert_eq!(perm, Permutation::from_perm(vec![0u8, 2u8, 3u8, 1u8]).unwrap());
        }
    }
}
