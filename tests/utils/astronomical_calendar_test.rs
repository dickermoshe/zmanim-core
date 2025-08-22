use rand::Rng;
use zmanim_core::{
    GeoLocation,
    astronomical_calendar::{
        AstronomicalCalendar as RustAstronomicalCalendar, AstronomicalCalendarTrait,
    },
};

use crate::{
    java::astronomical_calendar::AstronomicalCalendar as JavaAstronomicalCalendar,
    test_utils::{
        assert_almost_equal_f64, assert_almost_equal_f64_option, assert_almost_equal_i64_option,
        create_jvm, random_test_geolocation, random_test_timestamp,
    },
};

#[derive(Debug)]
struct TestCase {
    lat: f64,
    lon: f64,
    elevation: f64,
    timestamp: i64,
    zenith: f64,
    start_time: f64,
    end_time: f64,
}

impl TestCase {
    fn new() -> Self {
        let test_geo = random_test_geolocation();
        let test_timestamp = random_test_timestamp();
        let random_zenith = rand::rng().random_range(0.0..=180.0);
        let random_start_time = rand::rng().random_range(0.0..=1000000000000000000.0);
        let random_end_time = rand::rng().random_range(random_start_time..=1000000000000000000.0);
        Self {
            lat: test_geo.lat,
            lon: test_geo.lon,
            elevation: test_geo.elevation,
            timestamp: test_timestamp,
            zenith: random_zenith,
            start_time: random_start_time,
            end_time: random_end_time,
        }
    }
}

#[test]
fn test_astronomical_calendar() {
    let jvm = create_jvm();

    // Test with shuffled locations
    for _ in 0..10_000 {
        let test_case = TestCase::new();
        let rust_geolocation = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");

        let rust_calendar = RustAstronomicalCalendar::new(test_case.timestamp, &rust_geolocation);
        let java_calendar =
            JavaAstronomicalCalendar::new(&jvm, test_case.timestamp, &rust_geolocation);

        let message = format!("test_case: {:?}", test_case);
        assert_almost_equal_f64(
            rust_calendar.get_utc_sunset(test_case.zenith),
            java_calendar.get_utc_sunset(test_case.zenith),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sea_level_sunrise(test_case.zenith),
            java_calendar.get_utc_sea_level_sunrise(test_case.zenith),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sunrise(test_case.zenith),
            java_calendar.get_utc_sunrise(test_case.zenith),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar.get_utc_sea_level_sunset(test_case.zenith),
            java_calendar.get_utc_sea_level_sunset(test_case.zenith),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sea_level_sunset(),
            &java_calendar.get_sea_level_sunset(),
            0.00000001,
            &message,
        );

        // Test basic sunset/sunrise methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sunset(),
            &java_calendar.get_sunset(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sunrise(),
            &java_calendar.get_sunrise(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sea_level_sunrise(),
            &java_calendar.get_sea_level_sunrise(),
            0.00000001,
            &message,
        );

        // Test offset methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sunrise_offset_by_degrees(test_case.zenith),
            &java_calendar.get_sunrise_offset_by_degrees(test_case.zenith),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_sunset_offset_by_degrees(test_case.zenith),
            &java_calendar.get_sunset_offset_by_degrees(test_case.zenith),
            0.00000001,
            &message,
        );

        // Test twilight methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_civil_twilight(),
            &java_calendar.get_begin_civil_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_nautical_twilight(),
            &java_calendar.get_begin_nautical_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_begin_astronomical_twilight(),
            &java_calendar.get_begin_astronomical_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_civil_twilight(),
            &java_calendar.get_end_civil_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_nautical_twilight(),
            &java_calendar.get_end_nautical_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_end_astronomical_twilight(),
            &java_calendar.get_end_astronomical_twilight(),
            0.00000001,
            &message,
        );

        assert_almost_equal_f64_option(
            &rust_calendar.get_solar_midnight(),
            &java_calendar.get_solar_midnight(),
            0.00000001,
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
                test_case.start_time as f64,
                test_case.end_time as f64,
            );
        let java_temporal_with_start_and_end_times = java_calendar
            .get_temporal_hour_with_start_and_end_times(
                test_case.start_time as f64,
                test_case.end_time as f64,
            );
        assert_almost_equal_i64_option(
            &rust_temporal_with_start_and_end_times,
            &java_temporal_with_start_and_end_times,
            10,
            &message,
        );

        // Test transit and temporal methods
        assert_almost_equal_f64_option(
            &rust_calendar.get_sun_transit(),
            &java_calendar.get_sun_transit(),
            0.00000001,
            &message,
        );

        let rust_sun_transit_with_start_and_end_times = rust_calendar
            .get_sun_transit_with_start_and_end_times(
                test_case.start_time as f64,
                test_case.end_time as f64,
            );
        let java_sun_transit_with_start_and_end_times = java_calendar
            .get_sun_transit_with_start_and_end_times(
                test_case.start_time as f64,
                test_case.end_time as f64,
            );
        let result = match (
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
        ) {
            (Some(rust_time), Some(java_time)) => rust_time - java_time,
            _ => 0.0,
        };
        assert!(
            result.abs() <= 128.0,
            "Sun transit with start and end times mismatch: rust={:?}, java={:?}, distance: {}, {}",
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
            result.abs(),
            message
        );
        drop(java_calendar.instance);
    }
}
