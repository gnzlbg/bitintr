use int::Int;

/// Bit Field Extract.
///
/// See [`x86::bmi::bextr`](fn.bextr.html).
#[inline]
pub fn bextr<T: Int>(source: T, start: T, length: T) -> T {
    (source >> start) & ((T::one() << length) - T::one())
}

pub trait BEXTR {
    #[inline]
    fn bextr(self, Self, Self) -> Self;
}

impl<T: Int> BEXTR for T {
    #[inline]
    fn bextr(self, start: Self, length: Self) -> Self {
        bextr(self, start, length)
    }
}
