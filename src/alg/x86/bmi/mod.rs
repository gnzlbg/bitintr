//! Bit Manipulation Instruction (BMI) Set 1.0: software fallback.

pub mod andn;
pub mod bextr;
pub mod blsi;
pub mod blsmsk;
pub mod blsr;
pub mod tzcnt;

pub use self::andn::*;
pub use self::bextr::*;
pub use self::blsi::*;
pub use self::blsmsk::*;
pub use self::blsr::*;
pub use self::tzcnt::*;
