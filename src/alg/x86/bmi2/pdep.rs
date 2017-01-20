use int::Int;

/// Parallel bits deposit.
///
/// See [`x86::bmi2::pdep`](fn.pdep.html).
pub fn pdep<T: Int>(x: T, mask_: T) -> T {
    let mut res = T::zero();
    let mut mask = mask_;
    let mut bb = T::one();
    loop {
        if mask == T::zero() {
            break;
        }
        if (x & bb) != T::zero() {
            res = res | (mask & mask.wrapping_neg());
        }
        mask = mask & (mask - T::one());
        bb = bb + bb;
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
