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
    /// let n = 0b1011_1110_1001_0011u16;
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

macro_rules! pext_impl {
    ($ty:ty) => {
        #[inline]
        fn pext_(value: $ty, mut mask: $ty) -> $ty {
            let mut res = 0;
            let mut bb = 1;
            loop {
                if mask == 0 {
                    break;
                }
                if value & mask & (mask.wrapping_neg()) != 0 {
                    res |= bb;
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
                unsafe fn pext_(value: $ty, mask: $ty) -> $ty {
                    crate::arch::$intr(
                        value as _,
                        mask as _,
                    ) as _
                }
            } else {
                pext_impl!($ty);
            }
        }
    };
}

macro_rules! impl_pext {
    ($id:ident $(,$args:ident)*) => {
        impl Pext for $id {
            #[inline]
            #[allow(unused_unsafe)]
            fn pext(self, mask: Self) -> Self {
                pext_impl!($id $(,$args)*);
                // UNSAFETY: this is always safe, because
                // the unsafe `#[target_feature]` function
                // is only generated when the feature is
                // statically-enabled at compile-time.
                unsafe { pext_(self, mask) }
            }
        }
    }
}

impl_all!(impl_pext: u8, u16, i8, i16);

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        impl_pext!(u32, _pext_u32);
        impl_pext!(i32, _pext_u32);
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                impl_pext!(u64, _pext_u64);
                impl_pext!(i64, _pext_u64);
            } else {
                impl_all!(impl_pext: i64, u64);
            }
        }
    } else {
        impl_all!(impl_pext: u32, i32, i64, u64);
    }
}
