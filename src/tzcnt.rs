//! tzcnt

/// Counts trailing zero bits
pub trait Tzcnt {
    /// Counts the number of trailing least significant zero bits.
    ///
    /// When the source operand is 0, it returns its size in bits.
    ///
    /// This is equivalent to searching for the least significant set bit and
    /// returning its index.
    ///
    /// **Keywords**: Count trailing zeros, Bit scan forward, find first set.
    ///
    /// # Instructions
    ///
    /// - [`TZCNT`](http://www.felixcloutier.com/x86/TZCNT.html):
    ///   - Description: Count the number of trailing zero bits.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 16/32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b1001_0000_u16.tzcnt(), 4_u16);
    /// assert_eq!(0b0000_0000_u32.tzcnt(), 32_u32);
    /// assert_eq!(0b0000_0001_u64.tzcnt(), 0_u64);
    /// ```
    fn tzcnt(self) -> Self;
}

macro_rules! impl_tzcnt {
    ($id:ident) => {
        impl Tzcnt for $id {
            #[inline]
            fn tzcnt(self) -> Self {
                self.trailing_zeros() as Self
            }
        }
    };
}

impl_all!(impl_tzcnt: u8, u16, u32, u64, i8, i16, i32, i64);
