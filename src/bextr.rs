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

macro_rules! empty {
    ($_x:ident, $_y:ident, $_z:ident, $_i:ident) => {};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! bextr_spec {
    ($x:ident, $y:ident, $z:ident, $intr:ident) => {
        #[cfg(feature = "unstable")]
        #[cfg(
            all(
                any(target_arch = "x86", target_arch = "x86_64"),
                target_feature = "bmi"
            )
        )]
        {
            return unsafe {
                ::mem::transmute(::arch::$intr(
                    ::mem::transmute($x),
                    ::mem::transmute($y),
                    ::mem::transmute($z),
                ))
            };
        }
    };
}

macro_rules! impl_bextr {
    ($id:ident, $arch_pdep:ident, $intr:ident) => {
        impl Bextr for $id {
            #[inline]
            fn bextr(self, start: Self, length: Self) -> Self {
                $arch_pdep!(self, start, length, $intr);
                (self >> start) & ((1 << length) - 1)
            }
            #[inline]
            fn bextri(self, range: u32) -> Self {
                self.bextr((range & 0xff) as Self, (range >> 8) as Self)
            }
        }
    };
    ($id:ident) => {
        impl_bextr!($id, empty, empty);
    };
}

impl_all!(impl_bextr: u8, u16, i8, i16);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_bextr!(u32, bextr_spec, _bextr_u32);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_bextr!(i32, bextr_spec, _bextr_u32);
#[cfg(target_arch = "x86_64")]
impl_bextr!(u64, bextr_spec, _bextr_u64);
#[cfg(target_arch = "x86_64")]
impl_bextr!(i64, bextr_spec, _bextr_u64);

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl_all!(impl_bextr: u32, i32);
#[cfg(not(target_arch = "x86_64"))]
impl_all!(impl_bextr: i64, u64);
