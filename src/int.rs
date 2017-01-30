//! Generic Integer traits.

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Not, BitAnd, BitOr, BitXor, Shl, Shr};
use std::cmp::{PartialEq, PartialOrd};
use std::mem::size_of;

/// Integer trait used to parametrize algorithms for all integer types.
pub trait Int
    : Sized
    + Copy
    + Add<Output=Self>
    + Sub<Output=Self>
    + Mul<Output=Self>
    + Div<Output=Self>
    + Not<Output=Self>
    + BitAnd<Output=Self>
    + BitOr<Output=Self>
    + BitXor<Output=Self>
    + Shr<Self, Output=Self>
    + Shl<Self, Output=Self>
    + PartialEq + PartialOrd
{
    #[inline(always)] fn one() -> Self;
    #[inline(always)] fn zero() -> Self;
    #[inline(always)] fn byte_size() -> Self;
    #[inline(always)] fn bit_size() -> Self;
    #[inline(always)] fn count_ones(self) -> Self;
    #[inline(always)] fn count_zeros(self) -> Self;
    #[inline(always)] fn leading_zeros(self) -> Self;
    #[inline(always)] fn trailing_zeros(self) -> Self;
    #[inline(always)] fn wrapping_neg(self) -> Self;
    #[inline(always)] fn wrapping_add(self, Self) -> Self;
    #[inline(always)] fn wrapping_sub(self, Self) -> Self;
    #[inline(always)] fn to_u32(self) -> u32;
    #[inline(always)] fn to_u64(self) -> u64;
    #[inline(always)] fn from_u16(u16) -> Self;
    #[inline(always)] fn from_u32(u32) -> Self;
    #[inline(always)] fn from_u64(u64) -> Self;
    #[inline(always)] fn rotate_left(self, u32) -> Self;
    #[inline(always)] fn rotate_right(self, u32) -> Self;
    #[inline(always)] fn swap_bytes(self) -> Self;
    #[inline(always)] fn from_be(self) -> Self;
    #[inline(always)] fn from_le(self) -> Self;
    #[inline(always)] fn to_be(self) -> Self;
    #[inline(always)] fn to_le(self) -> Self;
    #[inline(always)] fn pow(self, exp: u32) -> Self;
}

macro_rules! int_impl {
    ($T:ty) => (
        impl Int for $T {
            #[inline(always)] fn one() -> Self { 1 as Self }
            #[inline(always)] fn zero() -> Self { 0 as Self }

            #[inline(always)] fn byte_size() -> Self {
                size_of::<Self>() as Self
            }

            #[inline(always)] fn bit_size() -> Self {
                (Self::byte_size() * 8) as Self
            }

            #[inline(always)] fn count_ones(self) -> $T {
                self.count_ones() as $T
            }
            #[inline(always)] fn count_zeros(self) -> $T {
                self.count_zeros() as $T
            }
            #[inline(always)] fn leading_zeros(self) -> $T {
                self.leading_zeros() as $T
            }
            #[inline(always)] fn trailing_zeros(self) -> $T {
                self.trailing_zeros() as $T
            }
            #[inline(always)] fn wrapping_neg(self) -> $T {
                self.wrapping_neg() as $T
            }
            #[inline(always)] fn wrapping_add(self, o: Self) -> $T {
                self.wrapping_add(o) as $T
            }
            #[inline(always)] fn wrapping_sub(self, o: Self) -> $T {
                self.wrapping_sub(o) as $T
            }
            #[inline(always)] fn to_u32(self) -> u32 { self as u32 }
            #[inline(always)] fn to_u64(self) -> u64 { self as u64 }
            #[inline(always)] fn from_u16(x: u16) -> Self { x as Self }
            #[inline(always)] fn from_u32(x: u32) -> Self { x as Self }
            #[inline(always)] fn from_u64(x: u64) -> Self { x as Self }

            #[inline(always)] fn rotate_left(self, n: u32) -> Self { (self as $T).rotate_left(n) }
            #[inline(always)] fn rotate_right(self, n: u32) -> Self { (self as $T).rotate_right(n) }
            #[inline(always)] fn swap_bytes(self) -> Self { <$T>::swap_bytes(self) }

            #[inline(always)] fn from_be(self) -> Self {
                <$T>::from_be(self)
            }

            #[inline(always)] fn from_le(self) -> Self {
                <$T>::from_le(self)
            }

            #[inline(always)] fn to_be(self) -> Self {
                <$T>::to_be(self)
            }

            #[inline(always)] fn to_le(self) -> Self {
                <$T>::to_le(self)
            }

            #[inline(always)] fn pow(self, exp: u32) -> Self {
                <$T>::pow(self, exp)
            }

        }
    )
}


int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(usize);

int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(isize);
