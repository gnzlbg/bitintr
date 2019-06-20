//! tzmsk

/// Mask from trailing zeros
pub trait Tzmsk {
    /// Sets all bits below the least significant one of `self` and clears all
    /// other bits.
    ///
    /// If the least significant bit of `self` is 1, it returns zero.
    ///
    /// # Instructions
    ///
    /// - [`TZMSK`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Mask from trailing zeros.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_1000u8.tzmsk(), 0b0000_0111u8);
    /// assert_eq!(0b0101_1001u8.tzmsk(), 0b0000_0000u8);
    /// ```
    fn tzmsk(self) -> Self;
}

macro_rules! impl_tzmsk {
    ($id:ident) => {
        impl Tzmsk for $id {
            #[inline]
            fn tzmsk(self) -> Self {
                !self & (self.wrapping_sub(1))
            }
        }
    };
}

impl_all!(impl_tzmsk: u8, u16, u32, u64, i8, i16, i32, i64);
