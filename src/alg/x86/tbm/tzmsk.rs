use int::Int;

/// Sets all bits below the least significant one of `x` and clears all other
/// bits.
///
/// If the least significant bit of `x` is 1, it returns zero.
///
/// # Assembly Instructions
///
/// - [`TZMSK`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Mask from trailing zeros.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(tzmsk(0b0101_1000u8), 0b0000_0111u8);
/// assert_eq!(0b0101_1001u8.tzmsk(), 0b0000_0000u8);
/// ```
#[inline]
pub fn tzmsk<T: Int>(x: T) -> T {
    !x & (x.wrapping_sub(T::one()))
}

/// Method version of [`tzmsk`](fn.tzmsk.html).
pub trait TZMSK {
    #[inline]
    fn tzmsk(self) -> Self;
}

impl<T: Int> TZMSK for T {
    #[inline]
    fn tzmsk(self) -> T {
        tzmsk(self)
    }
}
