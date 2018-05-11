//! blcs

/// Set lowest clear bit
pub trait Blcs {
    /// Sets the least significant bit of `self`.
    ///
    /// If there is no zero bit in `self`, it returns `self`.
    ///
    /// # Instructions
    ///
    /// - [`BLCS`](http://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Set lowest clear bit.
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0001u8.blcs(), 0b0101_0011u8);
    /// assert_eq!(0b1111_1111u8.blcs(), 0b1111_1111u8);
    /// ```
    fn blcs(self) -> Self;
}

macro_rules! impl_blcs {
    ($id:ident) => {
        impl Blcs for $id {
            #[inline]
            fn blcs(self) -> Self {
                self | (self.wrapping_add(1))
            }
        }
    };
}

impl_all!(impl_blcs: u8, u16, u32, u64, i8, i16, i32, i64);
