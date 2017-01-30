use int::Int;

/// Sets all bits of `x` to 1 except for the least significant zero bit.
///
/// If there is no zero bit in `x`, it sets all bits.
///
/// # Assembly instructions
///
/// - [`BLCI`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Isolate lowest clear bit.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blci(0b0101_0000u8), 0b1111_1110u8);
/// assert_eq!(0b1111_1111u8.blci(), 0b1111_1111u8);
/// ```
#[inline] pub fn blci<T: Int>(x: T) -> T {
    x | !(x.wrapping_add(T::one()))
}

/// Method version of [`blci`](fn.blci.html).
pub trait BLCI {
    #[inline] fn blci(self) -> Self;
}

impl<T: Int> BLCI for T {
    #[inline] fn blci(self) -> T {
        blci(self)
    }
}
