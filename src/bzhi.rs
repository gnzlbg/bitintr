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

macro_rules! empty {
    ($_x:ident, $_y:ident, $_i:ident) => {};
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
macro_rules! bzhi_spec {
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
                    $y as u32,
                ))
            };
        }
    };
}

macro_rules! impl_bzhi {
    ($id:ident, $arch_bzhi:ident, $intr:ident) => {
        impl Bzhi for $id {
            #[inline]
            #[allow(unreachable_code)]
            fn bzhi(self, bit_position: u32) -> Self {
                debug_assert!(
                    bit_position < (::mem::size_of::<Self>() * 8) as u32
                );
                $arch_bzhi!(self, bit_position, $intr);
                self & ((1 << bit_position) - 1)
            }
        }
    };
    ($id:ident) => {
        impl_bzhi!($id, empty, empty);
    };
}

impl_all!(impl_bzhi: u8, u16, i8, i16);

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_bzhi!(u32, bzhi_spec, _bzhi_u32);
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
impl_bzhi!(i32, bzhi_spec, _bzhi_u32);
#[cfg(target_arch = "x86_64")]
impl_bzhi!(u64, bzhi_spec, _bzhi_u64);
#[cfg(target_arch = "x86_64")]
impl_bzhi!(i64, bzhi_spec, _bzhi_u64);

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
impl_all!(impl_bzhi: u32, i32);
#[cfg(not(target_arch = "x86_64"))]
impl_all!(impl_bzhi: i64, u64);
