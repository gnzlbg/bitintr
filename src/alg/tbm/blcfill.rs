use int::Int;

/// Clears all bits below the least significant zero bit of `x`.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// [`BLCFILL`](http://support.amd.com/TechDocs/24592.pdf): Fill from lowest clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blcfill;
///
/// assert_eq!(blcfill(0b0101_0111u8), 0b0101_0000u8);
/// assert_eq!(blcfill(0b1111_1111u8), 0u8);
/// ```
pub fn blcfill<T: Int>(x: T) -> T {
    x & (x.wrapping_add(T::one()))
}

pub trait BLCFILL {
    fn blcfill(self) -> Self;
}

impl<T: Int> BLCFILL for T {
    fn blcfill(self) -> T {
        blcfill(self)
    }
}
