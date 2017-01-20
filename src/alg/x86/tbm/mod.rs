//! Trailing Bit Manipulation (TBM) instructions: software fallback.

pub mod bextr;
pub mod blcfill;
pub mod blcic;
pub mod blci;
pub mod blcmsk;
pub mod blcs;
pub mod blsfill;
pub mod blsic;
pub mod t1mskc;
pub mod tzmsk;

pub use self::bextr::*;
pub use self::blcfill::*;
pub use self::blcic::*;
pub use self::blci::*;
pub use self::blcmsk::*;
pub use self::blcs::*;
pub use self::blsfill::*;
pub use self::blsic::*;
pub use self::t1mskc::*;
pub use self::tzmsk::*;
