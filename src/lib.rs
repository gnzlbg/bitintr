#![cfg_attr(RUSTC_IS_NIGHTLY, feature(cfg_target_feature))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(platform_intrinsics))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(i128_type))]
#![cfg_attr(RUSTC_IS_NIGHTLY, feature(link_llvm_intrinsics))]

#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]
#![no_std]

//! `bitintr` offers portable bit manipulation intrinsics
//! ([@github](https://github.com/gnzlbg/bitintr),
//! [@crates.io](https://crates.io/crates/bitintr)).
//!
//! The intrinsics are named after their CPU instruction and organized in
//! modules named after their architecture/instruction set:
//! `bitintr::{arch}::{instruction_set}::{instruction_name}`.
//!
//! They are implemented for all integer types _except_ `u128/i128`. Whether a
//! fallback software implementation is used depends on the integer types
//! involved, the instruction sets supported by the target, and/or whether a
//! nightly or stable compiler is used (some optimizations are only available in
//! nightly).
//!
//! ## Example
//!
//! ```rust
//! extern crate bitintr;
//! use bitintr::x86::bmi2::*;
//!
//! fn main() {
//!    // Intrinsics are provided as trait methods:
//!    let method_call = 1.pdep(0);
//!    // And as free functions:
//!    let free_call = pdep(1, 0);
//!    assert_eq!(method_call, free_call);
//! }
//! ```

extern crate core as std;

mod int;
pub use int::Int;

mod alg;
pub mod x86;
pub mod arm;
