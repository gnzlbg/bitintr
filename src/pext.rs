//! pext

/// Parallel bits extract
pub trait Pext {
    /// Parallel bits extract.
    ///
    /// Gathers the bits of `x` specified by the `mask_` into the contiguous
    /// low order bit positions of the result.
    ///
    /// The remaining high-order bits of the result are set to zero.
    ///
    /// **Keywords**: Parallel bits extract, gather bits.
    ///
    /// # Instructionss
    ///
    /// - [`PEXT`](http://www.felixcloutier.com/x86/PEXT.html):
    ///   - Description: Parallel bits extract.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI2.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// let n  = 0b1011_1110_1001_0011u16;
    ///
    /// let m0 = 0b0110_0011_1000_0101u16;
    /// let s0 = 0b0000_0000_0011_0101u16;
    ///
    /// let m1 = 0b1110_1011_1110_1111u16;
    /// let s1 = 0b0001_0111_0100_0011u16;
    ///
    /// assert_eq!(n.pext(m0), s0);
    /// assert_eq!(n.pext(m1), s1);
    /// ```
    fn pext(self, mask: Self) -> Self;
}

macro_rules! empty {
    ($_x:ident, $_y:ident, $_i:ident) => {};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! pext_spec {
    ($x:ident, $y:ident, $intr:ident) => {
        #[cfg(feature = "unstable")]
        #[cfg(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "bmi2"
            )
        )]
        {
            return unsafe {
                ::mem::transmute(::arch::$intr(
                    ::mem::transmute($x),
                    ::mem::transmute($y),
                ))
            };
        }
    };
}

macro_rules! impl_pext {
    ($id:ident, $arch_pext:ident, $intr:ident) => {
        impl Pext for $id {
            #[inline]
            #[allow(unreachable_code)]
            fn pext(self, mut mask: Self) -> Self {
                $arch_pext!(self, mask, $intr);
                let mut res = 0;
                let mut bb = 1;
                loop {
                    if mask == 0 {
                        break;
                    }
                    if self & mask & (mask.wrapping_neg()) != 0 {
                        res = res | bb;
                    }
                    mask = mask & (mask - 1);
                    bb = bb + bb;
                }
                res
            }
        }
    };
    ($id:ident) => {
        impl_pext!($id, empty, empty);
    };
}

impl_all!(impl_pext: u8, u16, i8, i16);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_pext!(u32, pext_spec, _pext_u32);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_pext!(i32, pext_spec, _pext_u32);
#[cfg(target_arch = "x86_64")]
impl_pext!(u64, pext_spec, _pext_u64);
#[cfg(target_arch = "x86_64")]
impl_pext!(i64, pext_spec, _pext_u64);

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl_all!(impl_pext: u32, i32);
#[cfg(not(target_arch = "x86_64"))]
impl_all!(impl_pext: i64, u64);
