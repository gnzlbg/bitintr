//! Trailing Bit Manipulation (TBM) instruction set: software fallback.

pub mod bextri;
pub mod blcfill;
pub mod blcic;
pub mod blci;
pub mod blcmsk;
pub mod blcs;
pub mod blsfill;
pub mod blsic;
pub mod t1mskc;
pub mod tzmsk;

pub use alg::tbm::bextri::*;
pub use alg::tbm::blcfill::*;
pub use alg::tbm::blcic::*;
pub use alg::tbm::blci::*;
pub use alg::tbm::blcmsk::*;
pub use alg::tbm::blcs::*;
pub use alg::tbm::blsfill::*;
pub use alg::tbm::blsic::*;
pub use alg::tbm::t1mskc::*;
pub use alg::tbm::tzmsk::*;
