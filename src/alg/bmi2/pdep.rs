use int::Int;

/// Parallel bits deposit.
///
/// Scatter contiguous low order bits of `x` to the result at the positions
/// specified by the `mask_`.
///
/// All other bits (bits not set in the mask) of the result are set to zero.
///
/// **Keywords**: Parallel bits deposit, scatter bits.
///
/// # Intrinsics (when available BMI2)
///
/// [`PDEP`](http://www.felixcloutier.com/x86/PDEP.html): Parallel bits deposit
/// (supports 32/64 bit registers).
///
/// # Examples
/// ```
/// use bitintr::bmi2::pdep;
/// let n  = 0b1011_1110_1001_0011u16;
///
/// let m0 = 0b0110_0011_1000_0101u16;
/// let s0 = 0b0000_0010_0000_0101u16;
///
/// let m1 = 0b1110_1011_1110_1111u16;
/// let s1 = 0b1110_1001_0010_0011u16;
///
/// assert_eq!(pdep(n, m0), s0);
/// assert_eq!(pdep(n, m1), s1);
/// ```
pub fn pdep<T: Int>(x: T, mask_: T) -> T {
    let mut res = T::zero();
    let mut mask = mask_;
    let mut bb = T::one();
    loop {
        if mask == T::zero() {
            break;
        }
        if (x & bb) != T::zero() {
            res |= mask & mask.wrapping_neg();
        }
        mask &= mask - T::one();
        bb += bb;
    }
    res
}

pub trait PDEP {
    fn pdep(self, Self) -> Self;
}

impl<T: Int> PDEP for T {
    fn pdep(self, y: Self) -> Self {
        pdep(self, y)
    }
}
