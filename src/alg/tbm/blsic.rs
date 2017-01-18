use int::Int;

/// Clears least significant bit and sets all other bits.
///
/// If there is no set bit in `x`, it sets all the bits.
///
/// # Intrinsic (when available TBM)
///
/// [`BLSIC`](http://support.amd.com/TechDocs/24592.pdf): Isolate lowest set bit
/// and complement (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blsic;
///
/// assert_eq!(blsic(0b0101_0100u8), 0b1111_1011u8);
/// assert_eq!(blsic(0b0000_0000u8), 0b1111_1111u8);
/// ```
pub fn blsic<T: Int>(x: T) -> T {
    !x | (x.wrapping_sub(T::one()))
}

pub trait BLSIC {
    fn blsic(self) -> Self;
}

impl<T: Int> BLSIC for T {
    fn blsic(self) -> T {
        blsic(self)
    }
}
