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
//! - `BEXTRi` (TODO).
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
mod blcfill;
mod blcic;
mod blci;
mod blcmsk;
mod blcs;
mod blsfill;
mod blsic;
mod t1mskc;
mod tzmsk;

pub use tbm::bextri::*;
pub use tbm::blcfill::*;
pub use tbm::blcic::*;
pub use tbm::blci::*;
pub use tbm::blcmsk::*;
pub use tbm::blcs::*;
pub use tbm::blsfill::*;
pub use tbm::blsic::*;
pub use tbm::t1mskc::*;
pub use tbm::tzmsk::*;
