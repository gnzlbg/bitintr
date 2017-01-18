#![feature(cfg_target_feature)]
#![feature(platform_intrinsics)]
#![feature(i128_type)]

//! Portable Bit Manipulation Intrinsics
//!
//! Portable implementation of bitwise manipulation instructions. The intrinsics
//! are:
//!
//! - named after the corresponding CPU instruction,
//! - organized in instruction set modules: `bitintr::{instruction_set}::{intrinsic_name}`, and
//! - implemented for all integer types, with software fallback depending on the
//!   integer type and the instruction sets supported by the target.

mod int;

mod x86;
mod alg;

pub mod abm;
pub mod tbm;
pub mod bmi;
pub mod bmi2;
