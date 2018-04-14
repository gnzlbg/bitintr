//! blsfill

/// Fill from lowest set bit
pub trait Blsfill {
    /// Sets all bits of `self` below the least significant one.
    ///
    /// If there is no set bit in `self`, it sets all the bits.
    ///
    /// # Instructions
    ///
    /// - [`BLSFILL`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Fill from lowest set bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0100u8.blsfill(), 0b0101_0111u8);
    /// assert_eq!(0b0000_0000u8.blsfill(), 0b1111_1111u8);
    /// ```
    fn blsfill(self) -> Self;
}

macro_rules! impl_blsfill {
    ($id:ident) => {
        impl Blsfill for $id {
            #[inline]
            fn blsfill(self) -> Self {
                self | (self.wrapping_sub(1))
            }
        }
    };
}

impl_all!(impl_blsfill: u8, u16, u32, u64, i8, i16, i32, i64);
