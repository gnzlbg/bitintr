//! `bitintr` offers portable bit manipulation intrinsics
//! ([@github](https://github.com/gnzlbg/bitintr),
//! [@crates.io](https://crates.io/crates/bitintr)).
//!
//! The intrinsics are named after their CPU instruction and organized in
//! traits of the same name. These traits are implemented for all integer types
//! _except_ `u128/i128`.
//!
//! The `std::arch` intrinsics are used when the required features are enabled
//! in the target. You might manually enable features via `-C
//! target-feature=+...` and/or `-C target-cpu=...`.
#![no_std]
#![cfg_attr(bitintr_nightly, feature(stdsimd))]

use core::{marker, mem};

mod arch {
    #[cfg(target_arch = "x86")]
    pub use core::arch::x86::*;

    #[cfg(target_arch = "x86_64")]
    pub use core::arch::x86_64::*;

    #[cfg(target_arch = "arm")]
    pub use core::arch::arm::*;

    #[cfg(target_arch = "aarch64")]
    pub use core::arch::aarch64::*;
}

#[macro_use]
mod macros;

mod rev;
pub use self::rev::Rev;

mod rbit;
pub use self::rbit::Rbit;

mod lzcnt;
pub use self::lzcnt::Lzcnt;

mod popcnt;
pub use self::popcnt::Popcnt;

mod cls;
pub use self::cls::Cls;

mod pdep;
pub use self::pdep::Pdep;

mod pext;
pub use self::pext::Pext;

mod bzhi;
pub use self::bzhi::Bzhi;

mod mulx;
pub use self::mulx::Mulx;

mod andn;
pub use self::andn::Andn;

mod bextr;
pub use self::bextr::Bextr;

mod blsi;
pub use self::blsi::Blsi;

mod blsic;
pub use self::blsic::Blsic;

mod blsmsk;
pub use self::blsmsk::Blsmsk;

mod blsr;
pub use self::blsr::Blsr;

mod tzcnt;
pub use self::tzcnt::Tzcnt;

mod blcfill;
pub use self::blcfill::Blcfill;

mod blci;
pub use self::blci::Blci;

mod blcic;
pub use self::blcic::Blcic;

mod blcmsk;
pub use self::blcmsk::Blcmsk;

mod blcs;
pub use self::blcs::Blcs;

mod blsfill;
pub use self::blsfill::Blsfill;

mod t1mskc;
pub use self::t1mskc::T1mskc;

mod tzmsk;
pub use self::tzmsk::Tzmsk;
