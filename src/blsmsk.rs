//! blsmsk

/// Get mask up to lowest set bit.
pub trait Blsmsk {
    /// Get mask up to lowest set bit.
    ///
    /// Sets all the bits of the result to `1` up to and including the lowest
    /// set bit of `self`.
    ///
    /// If `self` is zero, all the bits of the result are set.
    ///
    /// # Instructions
    ///
    /// - [`BLSMSK`](http://www.felixcloutier.com/x86/BLSMSK.html):
    ///   - Description: Get mask up to lowest set bit.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0011_0000u8.blsmsk(), 0b0001_1111u8);
    /// assert_eq!(0b0000_0000u8.blsmsk(), 0b1111_1111u8);
    /// ```
    fn blsmsk(self) -> Self;
}

macro_rules! impl_blsmsk {
    ($id:ident) => {
        impl Blsmsk for $id {
            #[inline]
            fn blsmsk(self) -> Self {
                self ^ (self.wrapping_sub(1))
            }
        }
    };
}

impl_all!(impl_blsmsk: u8, u16, u32, u64, i8, i16, i32, i64);
