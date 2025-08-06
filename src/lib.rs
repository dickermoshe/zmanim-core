// This line is a crate-level attribute that conditionally enables "no_std" mode for the crate.
// If neither the "std" feature nor the "test" configuration is enabled, the crate will not link to the Rust standard library.
// This is useful for supporting embedded or other environments where the standard library is unavailable.
#![no_std]

pub mod astronomical_calendar;
pub mod utils;
pub mod zmanim_calendar;
pub use utils::*;
