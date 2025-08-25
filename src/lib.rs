#![cfg_attr(not(feature = "headers"), no_std)]

pub mod astronomical_calendar;
pub mod complex_zmanim_calendar;
pub mod hebrew_calendar;
pub mod utils;
pub mod zmanim_calendar;
pub use utils::*;

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("zmanim_core.h")?
        .generate()
}
