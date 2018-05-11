//! blcfill

/// Fill from lowest clear bit
pub trait Blcfill {
    /// Clears all bits below the least significant zero bit of `self`.
    ///
    /// If there is no zero bit in `self`, it returns zero.
    ///
    /// # Instructions
    ///
    /// - [`BLCFILL`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Fill from lowest clear bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0111u8.blcfill(), 0b0101_0000u8);
    /// assert_eq!(0b1111_1111u8.blcfill(), 0u8);
    /// ```
    fn blcfill(self) -> Self;
}

macro_rules! impl_blcfill {
    ($id:ident) => {
        impl Blcfill for $id {
            #[inline]
            fn blcfill(self) -> Self {
                self & (self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_blcfill: u8, u16, u32, u64, i8, i16, i32, i64);
