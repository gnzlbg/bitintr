//! Software fallback algorithms
//!
//! This module provides the algorithms for a software emulation of the
//! intrinsics. LLVM is able to recognize most of these algorithms and translate
//! them to a single assembly instruction. .

pub mod abm;
pub mod tbm;
pub mod bmi;
pub mod bmi2;
