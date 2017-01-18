/// ! Generic Integer traits.

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Not, BitAnd, BitOr, BitXor, Shl, Shr};
use std::ops::{AddAssign, BitAndAssign, BitOrAssign};
use std::cmp::{PartialEq, PartialOrd};
use std::mem::size_of;

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
    + AddAssign
    + BitAndAssign
    + BitOrAssign
    + PartialEq + PartialOrd
{
    fn one() -> Self;
    fn zero() -> Self;
    fn bit_size() -> Self;
    fn count_ones(self) -> Self;
    fn count_zeros(self) -> Self;
    fn leading_zeros(self) -> Self;
    fn trailing_zeros(self) -> Self;
    fn wrapping_neg(self) -> Self;
    fn wrapping_add(self, Self) -> Self;
    fn wrapping_sub(self, Self) -> Self;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;
    fn from_u16(u16) -> Self;
    fn from_u32(u32) -> Self;
    fn from_u64(u64) -> Self;
}

macro_rules! int_impl {
    ($T:ty) => (
        impl Int for $T {
            fn one() -> Self { 1 as Self }
            fn zero() -> Self { 0 as Self }

            fn bit_size() -> Self {
                (size_of::<Self>() * 8) as Self
            }

            fn count_ones(self) -> $T {
                self.count_ones() as $T
            }
            fn count_zeros(self) -> $T {
                self.count_zeros() as $T
            }
            fn leading_zeros(self) -> $T {
                self.leading_zeros() as $T
            }
            fn trailing_zeros(self) -> $T {
                self.trailing_zeros() as $T
            }
            fn wrapping_neg(self) -> $T {
                self.wrapping_neg() as $T
            }
            fn wrapping_add(self, o: Self) -> $T {
                self.wrapping_add(o) as $T
            }
            fn wrapping_sub(self, o: Self) -> $T {
                self.wrapping_sub(o) as $T
            }
            fn to_u32(self) -> u32 { self as u32 }
            fn to_u64(self) -> u64 { self as u64 }
            fn from_u16(x: u16) -> Self { x as Self }
            fn from_u32(x: u32) -> Self { x as Self }
            fn from_u64(x: u64) -> Self { x as Self }

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


// BUGBUG: this assumes that USize >= 16 and <= 64
pub trait IntF16T64: Int {}

macro_rules! signed_or_unsigned_from_16_to_64_and_size_impl {
    ($T:ty) => (impl IntF16T64 for $T {})
}

signed_or_unsigned_from_16_to_64_and_size_impl!(u16);
signed_or_unsigned_from_16_to_64_and_size_impl!(u32);
signed_or_unsigned_from_16_to_64_and_size_impl!(u64);
signed_or_unsigned_from_16_to_64_and_size_impl!(usize);

signed_or_unsigned_from_16_to_64_and_size_impl!(i16);
signed_or_unsigned_from_16_to_64_and_size_impl!(i32);
signed_or_unsigned_from_16_to_64_and_size_impl!(i64);
signed_or_unsigned_from_16_to_64_and_size_impl!(isize);


// BUGBUG: this assumes that USize >= 32 and <= 64
pub trait IntF32T64: Int {}

macro_rules! signed_or_unsigned_from_32_to_64_and_size_impl {
    ($T:ty) => (impl IntF32T64 for $T {})
}

signed_or_unsigned_from_32_to_64_and_size_impl!(u32);
signed_or_unsigned_from_32_to_64_and_size_impl!(u64);
signed_or_unsigned_from_32_to_64_and_size_impl!(usize);

signed_or_unsigned_from_32_to_64_and_size_impl!(i32);
signed_or_unsigned_from_32_to_64_and_size_impl!(i64);
signed_or_unsigned_from_32_to_64_and_size_impl!(isize);
