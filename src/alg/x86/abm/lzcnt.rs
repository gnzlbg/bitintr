use int::Int;

/// Counts the leading most significant zero bits.
///
/// When the operand is zero, it returns its size in bits.
///
/// **Keywords**: Leading Zeros Count, Count Leading Zeros, Bit Scan Severse,
/// Find Last Set.
///
/// See also [`arm::v7::clz`](../../arm/v7/fn.clz.html).
///
/// # Assembly Instructions
///
/// - [`LZCNT`](http://www.felixcloutier.com/x86/LZCNT.html):
///   - Description: Count the number of leading zero bits.
///   - Architecture: x86.
///   - Instruction set: ABM, BMI.
///   - Registers: 16/32/64 bit.
///   - Note: This instruction is officially part of BMI1 but Intel (and AMD)
///   CPUs advertise it as being part of ABM.
///
/// - [`CLZ`](https://www.pjrc.com/teensy/beta/DDI0403D_arm_architecture_v7m_reference_manual.pdf):
///   - Description: Count Leading Zeros.
///   - Architecture: ARM.
///   - Instruction set: v7.
///   - Registers: 8/16/32 bit.
///
/// # Example
/// ```
/// use bitintr::x86::abm::*;
///
/// assert_eq!(lzcnt(0b0101_1010u16), 9u16);
/// assert_eq!(0b0101_1010u16.lzcnt(), 9u16);
/// ```
#[inline]
pub fn lzcnt<T: Int>(x: T) -> T {
    x.leading_zeros() // TODO: software emulation
}

/// Method version of [`lzcnt`](fn.lzcnt.html).
pub trait LZCNT {
    #[inline]
    fn lzcnt(self) -> Self;
}

impl<T: Int> LZCNT for T {
    #[inline]
    fn lzcnt(self) -> T {
        lzcnt(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn lzcnt_test() {
        {
            use alg::x86::abm::lzcnt::*;
            assert_eq!(lzcnt(0b0101_1010u16), 9u16);
            assert_eq!(LZCNT::lzcnt(0b0101_1010u16), 9u16);
            assert_eq!(0b0101_1010u16.lzcnt(), 9u16);
        }
    }
}
