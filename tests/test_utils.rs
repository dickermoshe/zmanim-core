use j4rs::{ClasspathEntry, Jvm, JvmBuilder};

use rand::Rng;
use serde::Deserialize;
use zmanim_core::prelude::*;

// Test configuration constants
#[allow(dead_code)]
pub const DEFAULT_TEST_ITERATIONS: usize = 1000;
#[allow(dead_code)]
pub const DEFAULT_FLOAT_TOLERANCE: f64 = 0.00000001;
#[allow(dead_code)]
pub const DEFAULT_INT_TOLERANCE: u64 = 0;
#[allow(dead_code)]
pub const MIN_TIMESTAMP_YEAR_OFFSET: i64 = -50; // 500 years ago to 500 years from now

#[allow(dead_code)]
pub const MAX_TIMESTAMP_YEAR_OFFSET: i64 = 500; // 500 years ago to 500 years from now

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct TestGeoLocation {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}
#[allow(dead_code)]
pub fn create_jvm() -> Jvm {
    JvmBuilder::new()
        .classpath_entry(ClasspathEntry::new("./zmanim-2.6.0-SNAPSHOT.jar"))
        .build()
        .unwrap()
}

#[allow(dead_code)]
pub fn random_test_geolocation() -> TestGeoLocation {
    let mut rng = rand::rng();
    let lat = rng.random_range(-90.0..=90.0);
    let lon = rng.random_range(-180.0..=180.0);
    let elevation = rng.random_range(0.0..=1000.0);
    TestGeoLocation {
        lat,
        lon,
        elevation,
    }
}

#[allow(dead_code)]
pub fn random_test_timestamp() -> i64 {
    let mut rng = rand::rng();

    let year_in_millis = 1000 * 60 * 60 * 24 * 365; // 1 year in milliseconds
    let start = year_in_millis * MIN_TIMESTAMP_YEAR_OFFSET; // N years ago
    let end = year_in_millis * MAX_TIMESTAMP_YEAR_OFFSET; // N years from now
    rng.random_range(start..=end)
}

/// Compare two f64 values using ULP (Unit in the Last Place) difference
/// This is more reliable than epsilon-based comparison for floating-point values
/// If that fails, we use a more lenient approach.
#[allow(dead_code)]
fn almost_equal_f64(a: f64, b: f64, diff: f64) -> bool {
    if a == b {
        return true;
    }
    if a.is_nan() && b.is_nan() {
        return true;
    }

    // Handle NaN and infinity cases
    if a.is_nan() || b.is_nan() || a.is_infinite() || b.is_infinite() {
        return false;
    }

    // Convert to integer representation
    let a_bits = a.to_bits();
    let b_bits = b.to_bits();

    // Handle sign differences - only equal if both are very close to zero
    if (a_bits >> 63) != (b_bits >> 63) {
        return a.abs() < f64::EPSILON && b.abs() < f64::EPSILON;
    }

    // Calculate ULP difference
    let ulp_diff = if a_bits > b_bits {
        a_bits - b_bits
    } else {
        b_bits - a_bits
    };
    ulp_diff <= 1_000_000 || (a - b).abs() < diff
}

#[allow(dead_code)]
fn almost_equal_i64(a: i64, b: i64, diff: u64) -> bool {
    // Prevent overflow by checking if diff fits in i64 range
    if diff > i64::MAX as u64 {
        return false;
    }
    (a - b).abs() <= diff as i64
}
#[allow(dead_code)]
pub fn assert_almost_equal_f64(a: f64, b: f64, diff: f64, message: &str) {
    let result = almost_equal_f64(a, b, diff);
    let distance = (a - b).abs();
    assert!(
        result,
        "Error: {:?}, {:?}, distance: {}, {}",
        a, b, distance, message
    );
}

#[allow(dead_code)]
pub fn assert_almost_equal_f64_option(a: &Option<f64>, b: &Option<f64>, diff: f64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

#[allow(dead_code)]
pub fn assert_almost_equal_f64_result(a: &Option<f64>, b: &Option<f64>, diff: f64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
#[allow(dead_code)]
pub fn assert_almost_equal_i64(a: i64, b: i64, diff: u64, message: &str) {
    let result = almost_equal_i64(a, b, diff);
    let distance = (a - b).abs();
    if !result {
        println!(
            "Error: {:?}, {:?}, distance: {}, {}",
            a, b, distance, message
        );
    }
    assert!(result);
}
#[allow(dead_code)]
pub fn assert_almost_equal_i64_option(a: &Option<i64>, b: &Option<i64>, diff: u64, message: &str) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_i64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            assert!(false, "Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct TestCase {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
    pub timestamp: i64,
    pub zenith: f64,
    pub start_time: i64,
    pub end_time: i64,
    pub use_astronomical_chatzos: bool,
    pub use_astronomical_chatzos_for_other_zmanim: bool,
    pub candle_lighting_offset: i64,
    pub ateret_torah_sunset_offset: i64,
    pub solar_event: SolarEvent,
    pub solar_declination: f64,
    pub use_elevation: bool,
    pub is_azimuth: bool,
    pub julian_day: f64,
}

impl TestCase {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let test_geo = random_test_geolocation();
        let test_timestamp = random_test_timestamp();
        let random_zenith = rand::rng().random_range(0.0..=180.0);
        let random_start_time =
            rand::rng().random_range(-1000000000000000000..=1000000000000000000);
        let random_end_time = rand::rng().random_range(random_start_time..=1000000000000000000);
        let test_use_astronomical_chatzos = rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_use_astronomical_chatzos_for_other_zmanim =
            rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_candle_lighting_offset = rand::rng().random_range(0..=60 * 1000);
        let ateret_torah_sunset_offset = rand::rng().random_range(0..=60 * 1000);
        let solar_event = match rand::rng().random_range(0..=3) {
            0 => SolarEvent::Sunrise,
            1 => SolarEvent::Sunset,
            2 => SolarEvent::Noon,
            3 => SolarEvent::Midnight,
            _ => unreachable!(),
        };
        let solar_declination = rand::rng().random_range(-23.0..=23.0);
        let use_elevation = rand::rng().random_range(0.0..=1.0) > 0.5;
        let is_azimuth = rand::rng().random_range(0.0..=1.0) > 0.5;
        let julian_day = rand::rng().random_range(2415045.0..=2488045.0); // 1900-2100

        Self {
            lat: test_geo.lat,
            lon: test_geo.lon,
            elevation: test_geo.elevation,
            timestamp: test_timestamp,
            zenith: random_zenith,
            start_time: random_start_time,
            end_time: random_end_time,
            use_astronomical_chatzos: test_use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim:
                test_use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset: test_candle_lighting_offset,
            ateret_torah_sunset_offset,
            solar_event,
            solar_declination,
            use_elevation,
            is_azimuth,
            julian_day,
        }
    }
}
