use int::Int;

/// Sets all bits below the least significant one of `x` and clears all other
/// bits.
///
/// If the least significant bit of `x` is 1, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// [`TZMSK`](http://support.amd.com/TechDocs/24592.pdf): Mask from trailing
/// zeros.
///
/// # Example
///
/// ```
/// use bitintr::tbm::tzmsk;
///
/// assert_eq!(tzmsk(0b0101_1000u8), 0b0000_0111u8);
/// assert_eq!(tzmsk(0b0101_1001u8), 0b0000_0000u8);
/// ```
pub fn tzmsk<T: Int>(x: T) -> T {
    !x & (x.wrapping_sub(T::one()))
}

pub trait TZMSK {
    fn tzmsk(self) -> Self;
}

impl<T: Int> TZMSK for T {
    fn tzmsk(self) -> T {
        tzmsk(self)
    }
}
