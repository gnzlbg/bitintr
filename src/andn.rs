//! andn

/// Logical and not
pub trait Andn {
    /// Bitwise logical `AND` of inverted `self` with `y`.
    ///
    /// # Instructions
    ///
    /// - [`ANDN`](http://www.felixcloutier.com/x86/ANDN.html):
    ///   - Description: Logical and not.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0.andn(0), 0);
    /// assert_eq!(0.andn(1), 1);
    /// assert_eq!(1.andn(0), 0);
    /// assert_eq!(1.andn(1), 0);
    ///
    /// assert_eq!(0b0000_0000u8.andn(0b0000_0000u8), 0b0000_0000u8);
    /// assert_eq!(0b0000_0000u8.andn(0b1111_1111u8), 0b1111_1111u8);
    /// assert_eq!(0b1111_1111u8.andn(0b0000_0000u8), 0b0000_0000u8);
    /// assert_eq!(0b1111_1111u8.andn(0b1111_1111u8), 0b0000_0000u8);
    ///
    /// assert_eq!(0b0100_0000u8.andn(0b0101_1101u8), 0b0001_1101u8);
    /// ```
    fn andn(self, y: Self) -> Self;
}

macro_rules! impl_andn {
    ($id:ident) => {
        impl Andn for $id {
            #[inline]
            fn andn(self, y: Self) -> Self {
                !self & y
            }
        }
    };
}

impl_all!(impl_andn: u8, u16, u32, u64, i8, i16, i32, i64);
