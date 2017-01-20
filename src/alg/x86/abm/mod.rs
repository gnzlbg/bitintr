//! Advanced Bit Manipulation (ABM) instructions: software fallback.

mod popcnt;
mod lzcnt;

pub use self::popcnt::*;
pub use self::lzcnt::*;
