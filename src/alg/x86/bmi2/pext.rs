use int::Int;

/// Parallel bits extract.
///
/// See [`x86::bmi2::pdep`](fn.pdep.html).
#[inline]
pub fn pext<T: Int>(x: T, mask_: T) -> T {
    let mut res = T::zero();
    let mut mask = mask_;
    let mut bb = T::one();
    loop {
        if mask == T::zero() {
            break;
        }
        if x & mask & (mask.wrapping_neg()) != T::zero() {
            res = res | bb;
        }
        mask = mask & (mask - T::one());
        bb = bb + bb;
    }
    res
}

pub trait PEXT {
    #[inline]
    fn pext(self, Self) -> Self;
}

impl<T: Int> PEXT for T {
    #[inline]
    fn pext(self, y: Self) -> Self {
        pext(self, y)
    }
}
