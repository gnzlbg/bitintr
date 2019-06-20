//! bzhi

/// Zero high bits
pub trait Bzhi {
    /// Zero the high bits of `self` at position >= `bit_position`.
    ///
    /// # Panics
    ///
    /// If `bit_position >= bit_size()` and `-C debug-assertions=1`.
    ///
    /// # Instructions
    ///
    /// - [`BZHI`](http://www.felixcloutier.com/x86/BZHI.html):
    ///   - Description: Zero high bits starting with specified bit position.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI2.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// let n = 0b1111_0010_u32;
    /// let s = 0b0001_0010_u32;
    /// assert_eq!(n.bzhi(5), s);
    /// assert_eq!(n.bzhi(5), s);
    /// ```
    fn bzhi(self, bit_position: u32) -> Self;
}

macro_rules! bzhi_impl {
    ($ty:ty) => {
        #[inline]
        fn bzhi_(value: $ty, bit_position: u32) -> $ty {
            debug_assert!(
                bit_position < (crate::mem::size_of::<$ty>() * 8) as u32
            );
            value & ((1 << bit_position) - 1)
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
                unsafe fn bzhi_(value: $ty, bit_position: u32) -> $ty {
                    crate::arch::$intr(
                        value as _,
                        bit_position,
                    ) as _
                }
            } else {
                bzhi_impl!($ty);
            }
        }
    };
}

macro_rules! impl_bzhi {
    ($id:ident $(,$args:ident)*) => {
        impl Bzhi for $id {
            #[inline]
            #[allow(unused_unsafe)]
            fn bzhi(self, bit_position: u32) -> Self {
                bzhi_impl!($id $(,$args)*);
                // UNSAFETY: this is always safe, because
                // the unsafe `#[target_feature]` function
                // is only generated when the feature is
                // statically-enabled at compile-time.
                unsafe { bzhi_(self, bit_position) }
            }
        }
    };
}

impl_all!(impl_bzhi: u8, u16, i8, i16);

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        impl_bzhi!(u32, _bzhi_u32);
        impl_bzhi!(i32, _bzhi_u32);
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                impl_bzhi!(u64, _bzhi_u64);
                impl_bzhi!(i64, _bzhi_u64);
            } else {
                impl_all!(impl_bzhi: i64, u64);
            }
        }
    } else {
        impl_all!(impl_bzhi: u32, i32, i64, u64);
    }
}
