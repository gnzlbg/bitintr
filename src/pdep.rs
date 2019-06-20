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
    /// let n = 0b1011_1110_1001_0011u16;
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

macro_rules! pdep_impl {
    ($ty:ty) => {
        #[inline]
        fn pdep_(value: $ty, mut mask: $ty) -> $ty {
            let mut res = 0;
            let mut bb = 1;
            loop {
                if mask == 0 {
                    break;
                }
                if (value & bb) != 0 {
                    res |= mask & mask.wrapping_neg();
                }
                mask &= mask - 1;
                bb += bb;
            }
            res
        }
    };
    ($ty:ty, $intr:ident) => {
        cfg_if! {
            if  #[cfg(all(
                  any(target_arch = "x86", target_arch = "x86_64"),
                  target_feature = "bmi2"
            ))] {
                #[inline]
                #[target_feature(enable = "bmi2")]
                unsafe fn pdep_(value: $ty, mask: $ty) -> $ty {
                    crate::arch::$intr(
                        value as _,
                        mask as _,
                    ) as _
                }
            } else {
                pdep_impl!($ty);
            }

        }
    };
}

macro_rules! impl_pdep {
    ($id:ident $(,$args:ident)*) => {
        impl Pdep for $id {
            #[inline]
            #[allow(unused_unsafe)]
            fn pdep(self, mask: Self) -> Self {
                pdep_impl!($id $(,$args)*);
                // UNSAFETY: this is always safe, because
                // the unsafe `#[target_feature]` function
                // is only generated when the feature is
                // statically-enabled at compile-time.
                unsafe { pdep_(self, mask) }
           }
        }
    }
}

impl_all!(impl_pdep: u8, u16, i8, i16);

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        impl_pdep!(u32, _pdep_u32);
        impl_pdep!(i32, _pdep_u32);
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                impl_pdep!(u64, _pdep_u64);
                impl_pdep!(i64, _pdep_u64);
            } else {
                impl_all!(impl_pdep: i64, u64);
            }
        }
    } else {
        impl_all!(impl_pdep: u32, i32, i64, u64);
    }
}
