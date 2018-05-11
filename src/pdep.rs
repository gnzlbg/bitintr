//! pdep

/// Parallel bits deposit
pub trait Pdep {
    /// Parallel bits deposit.
    ///
    /// Scatter contiguous low order bits of `x` to the result at the positions
    /// specified by the `mask`.
    ///
    /// All other bits (bits not set in the `mask`) of the result are set to
    /// zero.
    ///
    /// **Keywords**: Parallel bits deposit, scatter bits.
    ///
    /// # Instructions
    ///
    /// - [`PDEP`](http://www.felixcloutier.com/x86/PDEP.html):
    ///   - Description: Parallel bits deposit.
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
    /// let s0 = 0b0000_0010_0000_0101u16;
    ///
    /// let m1 = 0b1110_1011_1110_1111u16;
    /// let s1 = 0b1110_1001_0010_0011u16;
    ///
    /// assert_eq!(n.pdep(m0), s0);
    /// assert_eq!(n.pdep(m1), s1);
    /// ```
    fn pdep(self, mask: Self) -> Self;
}

macro_rules! empty {
    ($_x:ident, $_y:ident, $_i:ident) => {};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! pdep_spec {
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

macro_rules! impl_pdep {
    ($id:ident, $arch_pdep:ident, $intr:ident) => {
        impl Pdep for $id {
            #[inline]
            #[allow(unreachable_code)]
            fn pdep(self, mut mask: Self) -> Self {
                $arch_pdep!(self, mask, $intr);
                let mut res = 0;
                let mut bb = 1;
                loop {
                    if mask == 0 {
                        break;
                    }
                    if (self & bb) != 0 {
                        res = res | (mask & mask.wrapping_neg());
                    }
                    mask = mask & (mask - 1);
                    bb = bb + bb;
                }
                res
            }
        }
    };
    ($id:ident) => {
        impl_pdep!($id, empty, empty);
    };
}

impl_all!(impl_pdep: u8, u16, i8, i16);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_pdep!(u32, pdep_spec, _pdep_u32);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_pdep!(i32, pdep_spec, _pdep_u32);
#[cfg(target_arch = "x86_64")]
impl_pdep!(u64, pdep_spec, _pdep_u64);
#[cfg(target_arch = "x86_64")]
impl_pdep!(i64, pdep_spec, _pdep_u64);

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl_all!(impl_pdep: u32, i32);
#[cfg(not(target_arch = "x86_64"))]
impl_all!(impl_pdep: i64, u64);
