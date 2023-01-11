#![forbid(unsafe_code)]
#![no_std]
#![warn(missing_docs)]
#![deny(missing_docs)]

//! Return the length of a tuple.
//! # Examples
//! Usage:
//! ```
//! # use tuple_length::TupLen;
//! assert_eq!(().len(), 0);
//! assert_eq!((1i8,).len(), 1);
//! assert_eq!((1u16, 2u64).len(), 2);
//! assert_eq!((1i8, 2usize, 3i64).len(), 3);
//! ```
//! # Supported tuple lengths
//! Possible length 8, 16, 32, 64.
//! By default the selected operations are implemented to tuples upto a length of **8 elements**.
//!
//! # Features
//! You can specify the length: features = ["16"] or features = ["32"] or features = ["64"].
//!

extern crate tuple_macro;
use tuple_macro::tuple_length;

/// Implements the len method on a tuple.
pub trait TupLen {
    /// Returns the length of this tuple.
    /// # Examples
    /// Usage:
    /// ```
    /// # use tuple_length::TupLen;
    /// let tuple: (i8, u8, i16, u16, i32, u32, i64, &str) = (-128, 255, -327, 655, -100, 229, -5,
    /// "rust");
    /// assert_eq!(tuple.len(), 8);
    ///
    /// let tuple = ([2u8, 0u8, 2u8, 1u8], 2021u16, vec!["r", "u", "s", "t"]);
    /// assert_eq!(tuple.len(), 3);
    /// ```
    fn len(&self) -> usize;
}

impl TupLen for () {
    #[inline]
    fn len(&self) -> usize {
        0usize
    }
}

tuple_length!();

#[cfg(test)]
mod tuple_len_tests {
    use super::*;

    #[cfg(all(not(feature = "16"), not(feature = "32"), not(feature = "64")))]
    #[test]
    fn features_8_ok() {
        assert_eq!((1i8, 2u8, 3i16, 4u16, 5i32, 6u32, 7i64, 8u64).len(), 8usize);
    }

    #[cfg(all(not(feature = "32"), not(feature = "64"), feature = "16"))]
    #[test]
    fn features_16_ok() {
        assert_eq!(
            (
                1i8, 2u8, 3i16, 4u16, 5i32, 6u32, 7i64, 8u64, 9isize, 10usize, 11i128, 12u128,
                13i8, 14u8, 15i16, 16u16
            )
                .len(),
            16usize
        );
    }

    #[cfg(all(not(feature = "16"), not(feature = "64"), feature = "32"))]
    #[test]
    fn features_32_ok() {
        assert_eq!(
            (
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
                30u8, 31u8, 32u8
            )
                .len(),
            32usize
        );
    }

    #[cfg(all(not(feature = "16"), not(feature = "32"), feature = "64"))]
    #[test]
    fn features_64_ok() {
        assert_eq!(
            (
                1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 11u8, 12u8, 13u8, 14u8, 15u8,
                16u8, 17u8, 18u8, 19u8, 20u8, 21u8, 22u8, 23u8, 24u8, 25u8, 26u8, 27u8, 28u8, 29u8,
                30u8, 31u8, 32u8, 33u8, 34u8, 35u8, 36u8, 37u8, 38u8, 39u8, 40u8, 41u8, 42u8, 43u8,
                44u8, 45u8, 46u8, 47u8, 48u8, 49u8, 50u8, 51u8, 52u8, 53u8, 54u8, 55u8, 56u8, 57u8,
                58u8, 59u8, 60u8, 61u8, 62u8, 63u8, 64u8
            )
                .len(),
            64usize
        );
    }
}

