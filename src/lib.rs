#![feature(specialization)]
#![feature(cfg_target_feature)]
#![feature(platform_intrinsics)]
#![feature(i128_type)]

//! Bit Manipulation Intrinsics
//!
//! This crate implements a low-level wrapper over the raw bit manipulation
//! intrinsics supported by modern CPUs. Its purpose is to allow higher-level
//! bit manipulation abstractions to easily use platform-specific intrinsics
//! when available.
//!
//! The intrinsics are named after the CPU instructions, use platform specific
//! functionality when available (detected through `target_feature` flags), and
//! fall back to software emulation otherwise.
//!
//! Currently, following architectures and instruction sets are implemented:
//!
//! - x86:
//!     - ABM: Advanced Bit Manipulation.
//!     - BMI: Bit Manipulation Instruction Set 1.
//!     - BMI2: Bit Manipulation Instruction Set 2 (partial).
//!     - TBM: Trailing Bit Manipulation.

mod int;

mod x86;
mod alg;

pub mod abm;
pub mod tbm;
pub mod bmi;
pub mod bmi2;
