//! blsr

/// Resets lowest set bit.
pub trait Blsr {
    /// Resets the lowest set bit of `self`.
    ///
    /// # Panics
    ///
    /// If `self` is zero the behavior is undefined. Panics if `-C
    /// debug-assertions=1`.
    ///
    /// # Instructions
    ///
    /// - [`BLSR`](http://www.felixcloutier.com/x86/BLSR.html):
    ///   - Description: Reset lowest set bit.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0011_0000u8.blsr(), 0b0010_0000u8);
    /// ```
    fn blsr(self) -> Self;
}

macro_rules! impl_blsr {
    ($id:ident) => {
        impl Blsr for $id {
            #[inline]
            fn blsr(self) -> Self {
                debug_assert!(self != 0);
                self & (self.wrapping_sub(1))
            }
        }
    };
}

impl_all!(impl_blsr: u8, u16, u32, u64, i8, i16, i32, i64);
