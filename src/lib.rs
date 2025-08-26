#![cfg_attr(not(feature = "uniffi"), no_std)]

#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();

pub mod astronomical_calendar;
pub mod complex_zmanim_calendar;
pub mod hebrew_calendar;
pub mod utils;
pub mod zmanim_calendar;

pub mod prelude {
    pub use crate::astronomical_calendar::*;
    pub use crate::complex_zmanim_calendar::*;
    pub use crate::hebrew_calendar::*;
    pub use crate::utils::*;
    pub use crate::zmanim_calendar::*;
}
