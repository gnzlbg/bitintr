//! Trailing Bit Manipulation (TBM) instruction set.
//!
//! The reference is [AMD64 Architecture Programmer's Manual, Volume 3:
//! General-Purpose and System
//! Instructions](http://support.amd.com/TechDocs/24594.pdf).
//!
//! [Wikipedia](https://en.wikipedia.org/wiki/Bit_Manipulation_Instruction_Sets#TBM_.28Trailing_Bit_Manipulation.29)
//! provides a quick overview of the available instructions.

mod bextr;

pub use self::bextr::*;
pub use alg::x86::tbm::blcfill::*;
pub use alg::x86::tbm::blcic::*;
pub use alg::x86::tbm::blci::*;
pub use alg::x86::tbm::blcmsk::*;
pub use alg::x86::tbm::blcs::*;
pub use alg::x86::tbm::blsfill::*;
pub use alg::x86::tbm::blsic::*;
pub use alg::x86::tbm::t1mskc::*;
pub use alg::x86::tbm::tzmsk::*;
