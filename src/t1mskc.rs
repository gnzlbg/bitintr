//! t1mskc

/// Inverse mask from trailing ones
pub trait T1mskc {
    /// Clears all bits below the least significant zero of `self` and sets all
    /// other bits.
    ///
    /// If the least significant bit of `self` is 0, it sets all bits.
    ///
    /// # Instructions
    ///
    /// - [`T1MSKC`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Inverse mask from trailing ones
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0111u8.t1mskc(), 0b1111_1000u8);
    /// assert_eq!(0b0101_0110u8.t1mskc(), 0b1111_1111u8);
    /// ```
    fn t1mskc(self) -> Self;
}

macro_rules! impl_t1mskc {
    ($id:ident) => {
        impl T1mskc for $id {
            #[inline]
            fn t1mskc(self) -> Self {
                !self | (self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_t1mskc: u8, u16, u32, u64, i8, i16, i32, i64);
