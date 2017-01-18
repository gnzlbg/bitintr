//! Advanced Bit Manipulation intrinsics: software fallback.

mod popcnt;
mod lzcnt;

pub use alg::abm::popcnt::*;
pub use alg::abm::lzcnt::*;
