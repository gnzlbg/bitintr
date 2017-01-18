//! Trailing Bit Manipulation (TBM) instruction set.
//!
//! For a quick overview see
//! [wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#TBM_.28Trailing_Bit_Manipulation.29).
//! The reference is [AMD64 Architecture Programmer's Manual, Volume 3:
//! General-Purpose and System
//! Instructions](http://support.amd.com/TechDocs/24594.pdf).
//!
//! It consists of the following instructions:
//!
//! - `BEXTRi`.
//! - `BLCFILL`.
//! - `BLCI`.
//! - `BLCIC`.
//! - `BLCMSK`.
//! - `BLCS`.
//! - `BLSFILL`.
//! - `BLSIC`.
//! - `T1MSKC`.
//! - `TZMSK`.

mod bextri;

pub use tbm::bextri::*;
pub use alg::tbm::blcfill::*;
pub use alg::tbm::blcic::*;
pub use alg::tbm::blci::*;
pub use alg::tbm::blcmsk::*;
pub use alg::tbm::blcs::*;
pub use alg::tbm::blsfill::*;
pub use alg::tbm::blsic::*;
pub use alg::tbm::t1mskc::*;
pub use alg::tbm::tzmsk::*;
