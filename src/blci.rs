//! blci

/// Isolate lowest clear bit
pub trait Blci {
    /// Sets all bits of `self` to 1 except for the least significant zero bit.
    ///
    /// If there is no zero bit in `self`, it sets all bits.
    ///
    /// # Instructions
    ///
    /// - [`BLCI`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Isolate lowest clear bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0000u8.blci(), 0b1111_1110u8);
    /// assert_eq!(0b1111_1111u8.blci(), 0b1111_1111u8);
    /// ```
    fn blci(self) -> Self;
}

macro_rules! impl_blci {
    ($id:ident) => {
        impl Blci for $id {
            #[inline]
            fn blci(self) -> Self {
                self | !(self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_blci: u8, u16, u32, u64, i8, i16, i32, i64);
