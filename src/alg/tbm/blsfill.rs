use int::Int;

/// Sets all bits of `x` below the least significant one.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// [`BLSFILL`](http://support.amd.com/TechDocs/24592.pdf): Fill from lowest set
/// bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blsfill;
///
/// assert_eq!(blsfill(0b0101_0100u8), 0b0101_0111u8);
/// assert_eq!(blsfill(0b0000_0000u8), 0b1111_1111u8);
/// ```
pub fn blsfill<T: Int>(x: T) -> T {
    x | (x.wrapping_sub(T::one()))
}

pub trait BLSFILL {
    fn blsfill(self) -> Self;
}

impl<T: Int> BLSFILL for T {
    fn blsfill(self) -> T {
        blsfill(self)
    }
}

