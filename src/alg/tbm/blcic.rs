use int::Int;

/// Sets the least significant bit of `x` and clears all other bits.
///
/// If there is no zero bit in `x`, it returns zero.
///
/// # Intrinsic (when available TBM)
///
/// [`BLCIC`](http://support.amd.com/TechDocs/24592.pdf): Isolate lowest clear
/// bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blcic;
///
/// assert_eq!(blcic(0b0101_0001u8), 0b0000_0010u8);
/// assert_eq!(blcic(0b1111_1111u8), 0b0000_0000u8);
/// ```
pub fn blcic<T: Int>(x: T) -> T {
    !x & (x.wrapping_add(T::one()))
}

pub trait BLCIC {
    fn blcic(self) -> Self;
}

impl<T: Int> BLCIC for T {
    fn blcic(self) -> T {
        blcic(self)
    }
}
