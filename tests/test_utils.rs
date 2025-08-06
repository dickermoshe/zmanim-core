use j4rs::{ClasspathEntry, Jvm, JvmBuilder};

use rand::Rng;
use serde::Deserialize;
use zmanim_core::GeoLocationError;

#[derive(Deserialize, Debug)]
pub struct TestGeoLocation {
    pub lat: f64,
    pub lon: f64,
    pub elevation: f64,
}
pub fn create_jvm() -> Jvm {
    JvmBuilder::new()
        .classpath_entry(ClasspathEntry::new("./zmanim-2.6.0-SNAPSHOT.jar"))
        .build()
        .unwrap()
}

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

pub fn random_test_timestamp() -> i64 {
    let mut rng = rand::rng();

    let year_in_millis = 1000 * 60 * 60 * 24 * 365; // 1 year in milliseconds
    let start = year_in_millis * -60; // 60 years ago
    let end = year_in_millis * 60; // 60 years from now
    rng.random_range(start..=end)
}

/// Compare two f64 values using ULP (Unit in the Last Place) difference
/// This is more reliable than epsilon-based comparison for floating-point values
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

fn almost_equal_i64(a: i64, b: i64, diff: u64) -> bool {
    return (a - b).abs() <= diff as i64;
}
pub fn assert_almost_equal_f64(a: f64, b: f64, diff: f64, message: &String) {
    let result = almost_equal_f64(a, b, diff);
    let distance = (a - b).abs();
    if !result {
        println!(
            "Error: {:?}, {:?}, distance: {}, {}",
            a, b, distance, message
        );
    }

    assert!(result);
}

pub fn assert_almost_equal_f64_option(
    a: &Option<f64>,
    b: &Option<f64>,
    diff: f64,
    message: &String,
) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            println!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}

pub fn assert_almost_equal_f64_result(
    a: &Result<f64, GeoLocationError>,
    b: &Result<f64, GeoLocationError>,
    diff: f64,
    message: &String,
) {
    match (a, b) {
        (Ok(a), Ok(b)) => assert_almost_equal_f64(*a, *b, diff, message),
        (Err(_), Err(_)) => (), // Both errors are considered equal
        _ => {
            println!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
pub fn assert_almost_equal_i64(a: i64, b: i64, diff: u64, message: &String) {
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
pub fn assert_almost_equal_i64_option(
    a: &Option<i64>,
    b: &Option<i64>,
    diff: u64,
    message: &String,
) {
    match (a, b) {
        (Some(a), Some(b)) => assert_almost_equal_i64(*a, *b, diff, message),
        (None, None) => (),
        _ => {
            println!("Error: {:?} vs {:?}, {}", a, b, message);
        }
    }
}
