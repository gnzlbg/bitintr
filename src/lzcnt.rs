//! lzcnt / clz

/// Count leading zeros
pub trait Lzcnt {
    /// Count Leading Zeros.
    ///
    /// See [`lzcnt`](fn.lzcnt.html).
    fn clz(self) -> Self;

    /// Counts the leading most significant zero bits.
    ///
    /// When the operand is zero, it returns its size in bits.
    ///
    /// **Keywords**: Leading Zeros Count, Count Leading Zeros, Bit Scan
    /// Severse, Find Last Set.
    ///
    /// See also [`arm::v7::clz`](../../arm/v7/fn.clz.html).
    ///
    /// # Instructions
    ///
    /// - [`LZCNT`](http://www.felixcloutier.com/x86/LZCNT.html):
    ///   - Description: Count the number of leading zero bits.
    ///   - Architecture: x86.
    ///   - Instruction set: ABM, BMI.
    ///   - Registers: 16/32/64 bit.
    /// - Note: This instruction is officially part of BMI1 but Intel (and
    /// AMD)   CPUs advertise it as being part of ABM.
    ///
    /// - [`CLZ`](https://www.pjrc.com/teensy/beta/
    ///   DDI0403D_arm_architecture_v7m_reference_manual.pdf):
    ///   - Description: Count Leading Zeros.
    ///   - Architecture: ARM.
    ///   - Instruction set: v7.
    ///   - Registers: 8/16/32 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_1010u16.clz(), 9u16);
    /// assert_eq!(0b0101_1010u16.lzcnt(), 9u16);
    /// ```
    fn lzcnt(self) -> Self;
}

macro_rules! impl_lzcnt {
    ($id:ident) => {
        impl Lzcnt for $id {
            #[inline]
            fn lzcnt(self) -> Self {
                self.leading_zeros() as Self
            }

            #[inline]
            fn clz(self) -> Self {
                Self::lzcnt(self)
            }
        }
    };
}

impl_all!(impl_lzcnt: u8, u16, u32, u64, i8, i16, i32, i64);
