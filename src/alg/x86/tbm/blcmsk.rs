use int::Int;

/// Sets the least significant bit of `x` and clears all bits above that bit.
///
/// If there is no zero bit in `x`, it sets all the bits.
///
/// # Assembly instructions
///
/// - [`BLCMSK`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Mask from lowest clear bit.
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(blcmsk(0b0101_0001u8), 0b0000_0011u8);
/// assert_eq!(0b1111_1111u8.blcmsk(), 0b1111_1111u8);
/// ```
#[inline] pub fn blcmsk<T: Int>(x: T) -> T {
    x ^ (x.wrapping_add(T::one()))
}

/// Method version of [`blcmsk`](fn.blcmsk.html).
pub trait BLCMSK {
    #[inline] fn blcmsk(self) -> Self;
}

impl<T: Int> BLCMSK for T {
    #[inline] fn blcmsk(self) -> T {
        blcmsk(self)
    }
}
