use int::Int;

/// Clears all bits below the least significant zero of `x` and sets all other
/// bits.
///
/// If the least significant bit of `x` is 0, it sets all bits.
///
/// # Assembly Instructions
///
/// - [`T1MSKC`](http://support.amd.com/TechDocs/24594.pdf):
///   - Description: Inverse mask from trailing ones
///   - Architecture: x86.
///   - Instruction set: TBM.
///   - Registers: 32/64 bit.
///
/// # Example
///
/// ```
/// use bitintr::x86::tbm::*;
///
/// assert_eq!(t1mskc(0b0101_0111u8), 0b1111_1000u8);
/// assert_eq!(0b0101_0110u8.t1mskc(), 0b1111_1111u8);
/// ```
pub fn t1mskc<T: Int>(x: T) -> T {
    !x | (x.wrapping_add(T::one()))
}

/// Method version of [`t1mskc`](fn.t1mskc.html).
pub trait T1MSKC {
    fn t1mskc(self) -> Self;
}

impl<T: Int> T1MSKC for T {
    fn t1mskc(self) -> T {
        t1mskc(self)
    }
}
