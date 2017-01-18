use int::Int;

/// Sets the least significant bit of `x`.
///
/// If there is no zero bit in `x`, it returns `x`.
///
/// # Intrinsic (when available TBM)
///
/// [`BLCS`](http://support.amd.com/TechDocs/24592.pdf): Set lowest clear bit
/// (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blcs;
///
/// assert_eq!(blcs(0b0101_0001u8), 0b0101_0011u8);
/// assert_eq!(blcs(0b1111_1111u8), 0b1111_1111u8);
/// ```
pub fn blcs<T: Int>(x: T) -> T {
    x | (x.wrapping_add(T::one()))
}

pub trait BLCS {
    fn blcs(self) -> Self;
}

impl<T: Int> BLCS for T {
    fn blcs(self) -> T {
        blcs(self)
    }
}
