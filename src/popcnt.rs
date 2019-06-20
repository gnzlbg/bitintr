//! popcnt

/// Count bits set.
pub trait Popcnt {
    /// Counts the bits that are set.
    ///
    /// **Keywords**: Population count, count ones, Hamming weight, Sideways
    /// sum.
    ///
    /// # Instructions
    ///
    /// - [`POPCNT`](http://www.felixcloutier.com/x86/POPCNT.html):
    ///   - Description: Population Count.
    ///   - Architecture: x86.
    ///   - Instruction set: ABM, SSE 4.2.
    ///   - Registers: 16/32/64 bit.
    /// - Note: Intel considers it part of SSE4.2 but advertises it with its
    /// own   CPUID flag.
    ///
    /// # Example
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_1010u16.popcnt(), 4);
    /// ```
    fn popcnt(self) -> Self;
}

macro_rules! impl_popcnt {
    ($id:ident) => {
        impl Popcnt for $id {
            #[inline]
            fn popcnt(self) -> Self {
                self.count_ones() as Self
            }
        }
    };
}

impl_all!(impl_popcnt: u8, u16, u32, u64, i8, i16, i32, i64);
