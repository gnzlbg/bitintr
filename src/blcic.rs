//! blcic

/// Isolate lowest clear bit
pub trait Blcic {
    /// Sets the least significant bit of `self` and clears all other bits.w
    ///
    /// If there is no zero bit in `self`, it returns zero.
    ///
    /// # Intrinsic (when available TBM)
    ///
    /// - [`BLCIC`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Isolate lowest clear bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0001u8.blcic(), 0b0000_0010u8);
    /// assert_eq!(0b1111_1111u8.blcic(), 0b0000_0000u8);
    /// ```
    fn blcic(self) -> Self;
}

macro_rules! impl_blcic {
    ($id:ident) => {
        impl Blcic for $id {
            #[inline]
            fn blcic(self) -> Self {
                !self & (self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_blcic: u8, u16, u32, u64, i8, i16, i32, i64);
