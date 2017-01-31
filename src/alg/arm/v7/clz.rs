use int::Int;
use alg;

/// Count Leading Zeros.
///
/// See [`abm::lzcnt`](../../x86/abm/fn.lzcnt.html).
///
/// # Example
///
/// ```
/// use bitintr::arm::v7::*;
///
/// assert_eq!(clz(0b0101_1010u16), 9u16);
/// assert_eq!(0b0101_1010u16.clz(), 9u16);
/// ```
#[inline]
pub fn clz<T: Int>(x: T) -> T {
    alg::x86::abm::lzcnt(x)
}

/// Method version of [`clz`](fn.clz.html).
pub trait CLZ {
    #[inline]
    fn clz(self) -> Self;
}

impl<T: Int> CLZ for T {
    #[inline]
    fn clz(self) -> T {
        clz(self)
    }
}
