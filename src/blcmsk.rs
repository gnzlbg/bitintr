//! blcmsk

/// Mask from lowest clear bit.
pub trait Blcmsk {
    /// Sets the least significant bit of `self` and clears all bits above that
    /// bit.
    ///
    /// If there is no zero bit in `self`, it sets all the bits.
    ///
    /// # Instructions
    ///
    /// - [`BLCMSK`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Mask from lowest clear bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0001u8.blcmsk(), 0b0000_0011u8);
    /// assert_eq!(0b1111_1111u8.blcmsk(), 0b1111_1111u8);
    /// ```
    fn blcmsk(self) -> Self;
}

macro_rules! impl_blcmsk {
    ($id:ident) => {
        impl Blcmsk for $id {
            #[inline]
            fn blcmsk(self) -> Self {
                self ^ (self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_blcmsk: u8, u16, u32, u64, i8, i16, i32, i64);
