use rand::Rng;
use zmanim_core::{
    GeoLocation, NOAACalculator,
    astronomical_calendar::{
        AstronomicalCalendar as RustAstronomicalCalendar, AstronomicalCalendarTrait,
    },
};

use crate::{
    java::astronomical_calendar::AstronomicalCalendar as JavaAstronomicalCalendar,
    test_utils::{
        assert_almost_equal_f64, assert_almost_equal_f64_option, create_jvm,
        random_test_geolocation, random_test_timestamp,
    },
};

// Tested Against b3fedc6c2d028f9aecd61e775e56ac253dbb1548
#[test]
fn test_astronomical_calendar() {
    let jvm = create_jvm();

    // Test with shuffled locations
    for _ in 0..10000 {
        let test_geo = random_test_geolocation();
        let test_timestamp = random_test_timestamp();
        let random_zenith = rand::rng().random_range(0.0..=180.0);
        let random_start_time = rand::rng().random_range(0.0..=1000000000000000000.0);
        let random_end_time = rand::rng().random_range(random_start_time..=1000000000000000000.0);

        let rust_geolocation = GeoLocation::new(test_geo.lat, test_geo.lon, test_geo.elevation)
            .expect("Failed to create Rust GeoLocation");

        let noaa_calculator = NOAACalculator::new();
        let rust_calendar =
            RustAstronomicalCalendar::new(test_timestamp, &rust_geolocation, &noaa_calculator);
        let java_calendar = JavaAstronomicalCalendar::new(&jvm, test_timestamp, &rust_geolocation);

        let message = format!(
            "test_geo: {:?}, test_timestamp: {:?}, random_zenith: {:?}, random_start_time: {:?}, random_end_time: {:?}",
            test_geo, test_timestamp, random_zenith, random_start_time, random_end_time
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sunset(random_zenith),
            java_calendar.get_utc_sunset(random_zenith),
            Some(100_000),
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sea_level_sunrise(random_zenith),
            java_calendar.get_utc_sea_level_sunrise(random_zenith),
            Some(10000),
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sunrise(random_zenith),
            java_calendar.get_utc_sunrise(random_zenith),
            Some(10000),
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sea_level_sunset(random_zenith),
            java_calendar.get_utc_sea_level_sunset(random_zenith),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sea_level_sunset(),
            &java_calendar.get_sea_level_sunset(),
            Some(10000),
            &message,
        );

        // Test basic sunset/sunrise methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sunset(),
            &java_calendar.get_sunset(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sunrise(),
            &java_calendar.get_sunrise(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sea_level_sunrise(),
            &java_calendar.get_sea_level_sunrise(),
            Some(10000),
            &message,
        );

        // Test offset methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sunrise_offset_by_degrees(random_zenith),
            &java_calendar.get_sunrise_offset_by_degrees(random_zenith),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sunset_offset_by_degrees(random_zenith),
            &java_calendar.get_sunset_offset_by_degrees(random_zenith),
            Some(10000),
            &message,
        );

        // Test twilight methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_civil_twilight(),
            &java_calendar.get_begin_civil_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_nautical_twilight(),
            &java_calendar.get_begin_nautical_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_astronomical_twilight(),
            &java_calendar.get_begin_astronomical_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_civil_twilight(),
            &java_calendar.get_end_civil_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_nautical_twilight(),
            &java_calendar.get_end_nautical_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_astronomical_twilight(),
            &java_calendar.get_end_astronomical_twilight(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_solar_midnight(),
            &java_calendar.get_solar_midnight(),
            Some(10000),
            &message,
        );

        // Test temporal hour - need special handling for Option<i64>
        let rust_temporal = rust_calendar.get_temporal_hour();
        let java_temporal = java_calendar.get_temporal_hour();
        assert_eq!(
            rust_temporal, java_temporal,
            "Temporal hour mismatch: rust={:?}, java={:?}, {}",
            rust_temporal, java_temporal, message
        );
        let rust_temporal_with_start_and_end_times = rust_calendar
            .get_temporal_hour_with_start_and_end_times(
                random_start_time as f64,
                random_end_time as f64,
            );
        let java_temporal_with_start_and_end_times = java_calendar
            .get_temporal_hour_with_start_and_end_times(
                random_start_time as f64,
                random_end_time as f64,
            );
        almost_equal_i64_option(
            &rust_temporal_with_start_and_end_times,
            &java_temporal_with_start_and_end_times,
            10.0,
            &message,
        );

        // Test transit and temporal methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sun_transit(),
            &java_calendar.get_sun_transit(),
            Some(10000),
            &message,
        );

        let rust_sun_transit_with_start_and_end_times = rust_calendar
            .get_sun_transit_with_start_and_end_times(
                random_start_time as f64,
                random_end_time as f64,
            );
        let java_sun_transit_with_start_and_end_times = java_calendar
            .get_sun_transit_with_start_and_end_times(
                random_start_time as f64,
                random_end_time as f64,
            );
        let result = match (
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
        ) {
            (Some(rust_time), Some(java_time)) => rust_time - java_time,
            _ => 0.0,
        };
        assert!(
            result.abs() < 129.0,
            "Sun transit with start and end times mismatch: rust={:?}, java={:?}, distance: {}, {}",
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
            result.abs(),
            message
        );
    }
}

fn almost_equal_i64_option(a: &Option<i64>, b: &Option<i64>, max_diff: f64, message: &String) {
    if a.is_some() && b.is_some() {
        let diff = a.unwrap() - b.unwrap();
        assert!(
            // 200 milliseconds is the maximum difference between the two implementations
            (diff.abs() as f64) < max_diff,
            "Temporal hour with start and end times mismatch: rust={:?}, java={:?}, distance: {}, {}",
            a,
            b,
            diff.abs(),
            message
        );
    } else {
        assert_eq!(a, b);
    }
}
