use int::Int;

/// Sets the least significant bit of `x` and clears all bits above that bit.
///
/// If there is no zero bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// [`BLCMSK`](http://support.amd.com/TechDocs/24592.pdf): Mask from lowest
/// clear bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blcmsk;
///
/// assert_eq!(blcmsk(0b0101_0001u8), 0b0000_0011u8);
/// assert_eq!(blcmsk(0b1111_1111u8), 0b1111_1111u8);
/// ```
pub fn blcmsk<T: Int>(x: T) -> T {
    x ^ (x.wrapping_add(T::one()))
}

pub trait BLCMSK {
    fn blcmsk(self) -> Self;
}

impl<T: Int> BLCMSK for T {
    fn blcmsk(self) -> T {
        blcmsk(self)
    }
}
