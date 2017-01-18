use int::Int;

/// Sets all bits of `x` to 1 except for the least significant zero bit.
///
/// If there is no zero bit in `x`, it sets all bits.
///
/// # Intrinsic (when available TBM)
///
/// [`BLCI`](http://support.amd.com/TechDocs/24592.pdf): Isolate lowest clear
/// bit (supports 32/64 bit registers).
///
/// # Example
///
/// ```
/// use bitintr::tbm::blci;
///
/// assert_eq!(blci(0b0101_0000u8), 0b1111_1110u8);
/// assert_eq!(blci(0b1111_1111u8), 0b1111_1111u8);
/// ```
pub fn blci<T: Int>(x: T) -> T {
    x | !(x.wrapping_add(T::one()))
}

pub trait BLCI {
    fn blci(self) -> Self;
}

impl<T: Int> BLCI for T {
    fn blci(self) -> T {
        blci(self)
    }
}
