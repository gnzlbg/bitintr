use int::Int;

/// Counts the leading most significant zero bits.
///
/// When the operand is zero, it returns its size in bits.
///
/// **Keywords**: Count leading zeros, bit scan reverse, find last set.
///
/// # Intrinsic (when available: ABM / SSE4.2)
///
/// [`LZCNT`](http://www.felixcloutier.com/x86/LZCNT.html): Count the number of
/// leading zero bits (supports 16/32/64 bit registers).
///
/// Note: This instruction is officialy part of BMI1 but Intel and AMD CPUs
/// advertise it as being part of ABM.
///
/// # Example
/// ```
/// use bitintr::abm::lzcnt;
///
/// assert_eq!(lzcnt(0b0101_1010u16), 9u16);
/// ```
pub fn lzcnt<T: Int>(x: T) -> T {
    x.leading_zeros() // TODO: software emulation
}

pub trait LZCNT {
    fn lzcnt(self) -> Self;
}

impl<T: Int> LZCNT for T {
    fn lzcnt(self) -> T { lzcnt(self) }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lzcnt_test() {
        {
            use alg::abm::lzcnt::*;
            assert_eq!(lzcnt(0b0101_1010u16), 9u16);
            assert_eq!(LZCNT::lzcnt(0b0101_1010u16), 9u16);
            assert_eq!(0b0101_1010u16.lzcnt(), 9u16);
        }
    }
}
