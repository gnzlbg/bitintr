//! blsic

/// Isolate lowest set bit and complement
pub trait Blsic {
    /// Clears least significant bit and sets all other bits.
    ///
    /// If there is no set bit in `self`, it sets all the bits.
    ///
    /// # Instructions
    ///
    /// - [`BLSIC`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Isolate lowest set bit and complement.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0100u8.blsic(), 0b1111_1011u8);
    /// assert_eq!(0b0000_0000u8.blsic(), 0b1111_1111u8);
    /// ```
    fn blsic(self) -> Self;
}

macro_rules! impl_blsic {
    ($id:ident) => {
        impl Blsic for $id {
            #[inline]
            fn blsic(self) -> Self {
                !self | (self.wrapping_sub(1))
            }
        }
    };
}

impl_all!(impl_blsic: u8, u16, u32, u64, i8, i16, i32, i64);
