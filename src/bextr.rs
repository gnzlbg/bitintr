//! bextr

/// Bit field extract
pub trait Bextr {
    /// Bit field extract.
    ///
    /// Extracts bits in range `[start, start + length)` from the `source` to
    /// the least significant bits of the result.
    ///
    /// Bits `[7,0]` of `range` specify the index to the first bit in the
    /// range to be extracted, and bits `[15,8]` specify the length of the
    /// range.
    ///
    /// Only bits up to `size_of::<T>()*8 - 1` are extracted.
    ///
    /// The extracted bits are written in the result starting from the
    /// least-significant bit. The high-order bits of the result are zeroed.
    ///
    /// # Instructions
    ///
    /// - [`BEXTR`](http://www.felixcloutier.com/x86/BEXTR.html):
    ///   - Description: Bit field extract.
    ///   - Architecture: x86.
    ///   - Instruction set: BMI.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(0b0101_0000u8.bextr(4, 4), 0b0000_0101u8);
    /// ```
    fn bextr(self, start: Self, length: Self) -> Self;

    /// Bit Field Extract (with immediate operand).
    ///
    /// Extracts bits in `range` from the `source` to the least significant
    /// bits of the result. Bits `[7,0]` of `range` specify the index to
    /// the first bit in the range to be extracted, and bits `[15,8]`
    /// specify the length of the range.
    ///
    /// Only bits up to `size_of::<T>()*8 - 1` are extracted.
    ///
    /// The extracted bits are written in the result starting from the
    /// least-significant bit. The high-order bits of the result are zeroed.
    ///
    /// # Instructions
    ///
    /// - [`BEXTR`](https://support.amd.com/TechDocs/24594.pdf):
    ///   - Description: Bit field extract (with immediate).
    ///   - Architecture: x86.
    ///   - Instruction set: TBM.
    ///   - Registers: 32/64 bit.
    ///
    /// # Example
    ///
    /// ```
    /// # use bitintr::*;
    /// assert_eq!(
    ///     0b0000_0000_0101_0000_u16.bextri(0b0100_0000_0100_u32),
    ///     0b0000_0000_0000_0101_u16
    /// );
    /// ```
    fn bextri(self, range: u32) -> Self;
}

macro_rules! bextr_impl {
    ($ty:ty) => {
        #[inline]
        fn bextr_(value: $ty, start: $ty, length: $ty) -> $ty {
            (value >> start) & ((1 << length) - 1)
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
                unsafe fn bextr_(value: $ty, start: $ty, length: $ty) -> $ty {
                    crate::arch::$intr(
                        value as _,
                        start as u32,
                        length as u32,
                    ) as _
                }
            } else {
                bextr_impl!($ty);
            }
        }
    };
}

macro_rules! impl_bextr {
    ($id:ident $(,$args:ident)*) => {
        impl Bextr for $id {
            #[inline]
            #[allow(unused_unsafe)]
            fn bextr(self, start: Self, length: Self) -> Self {
                bextr_impl!($id $(,$args)*);
                // UNSAFETY: this is always safe, because
                // the unsafe `#[target_feature]` function
                // is only generated when the feature is
                // statically-enabled at compile-time.
                unsafe { bextr_(self, start, length) }
            }
            #[inline]
            fn bextri(self, range: u32) -> Self {
                self.bextr((range & 0xff) as Self, (range >> 8) as Self)
            }

        }
    };
}

impl_all!(impl_bextr: u8, u16, i8, i16);

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        impl_bextr!(u32, _bextr_u32);
        impl_bextr!(i32, _bextr_u32);
        cfg_if! {
            if #[cfg(target_arch = "x86_64")] {
                impl_bextr!(u64, _bextr_u64);
                impl_bextr!(i64, _bextr_u64);
            } else {
                impl_all!(impl_bextr: i64, u64);
            }
        }
    } else {
        impl_all!(impl_bextr: u32, i32, i64, u64);
    }
}
