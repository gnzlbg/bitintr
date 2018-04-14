//! blsi

/// Extract lowest set isolated bit.
pub trait Blsi {
    /// Extract lowest set isolated bit.
    ///
    /// Extracts the lowest set bit of `self` and sets the corresponding bit
    /// in the result (all other bits of the result are zeroed).
    ///
    /// # Instructions
    ///
    /// - [`BLSI`](http://www.felixcloutier.com/x86/BLSI.html):
    ///   - Description: Extract lowest set isolated bit.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b1101_0000u8.blsi(), 0b0001_0000u8);
    /// assert_eq!(0b0100_1000u8.blsi(), 0b0000_1000u8);
    /// ```
    fn blsi(self) -> Self;
}

macro_rules! impl_blsi {
    ($id:ident) => {
        impl Blsi for $id {
            #[inline]
            fn blsi(self) -> Self {
                self & self.wrapping_neg()
            }
        }
    };
}

impl_all!(impl_blsi: u8, u16, u32, u64, i8, i16, i32, i64);
