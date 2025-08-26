use zmanim_core::prelude::*;

mod java;
mod test_utils;
use test_utils::*;

use java::astronomical_calendar::AstronomicalCalendar as JavaAstronomicalCalendar;
use zmanim_core::astronomical_calendar::AstronomicalCalendar as RustAstronomicalCalendar;

#[test]
fn test_astronomical_calendar() {
    let jvm = create_jvm();

    for _ in 0..10_000 {
        let test_case = TestCase::new();
        let rust_geolocation = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");

        let java_calendar =
            JavaAstronomicalCalendar::new(&jvm, test_case.timestamp, &rust_geolocation);
        let rust_calendar = RustAstronomicalCalendar::new(test_case.timestamp, rust_geolocation);

        let message = format!("test_case: {:?}", test_case);
        assert_almost_equal_f64(
            rust_calendar
                .get_utc_sunset(test_case.zenith)
                .expect(&message),
            java_calendar
                .get_utc_sunset(test_case.zenith)
                .expect(&message),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar
                .get_utc_sea_level_sunrise(test_case.zenith)
                .expect(&message),
            java_calendar
                .get_utc_sea_level_sunrise(test_case.zenith)
                .expect(&message),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar
                .get_utc_sunrise(test_case.zenith)
                .expect(&message),
            java_calendar
                .get_utc_sunrise(test_case.zenith)
                .expect(&message),
            0.00000001,
            &message,
        );
        assert_almost_equal_f64(
            rust_calendar
                .get_utc_sea_level_sunset(test_case.zenith)
                .expect(&message),
            java_calendar
                .get_utc_sea_level_sunset(test_case.zenith)
                .expect(&message),
            0.00000001,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sea_level_sunset(),
            &java_calendar.get_sea_level_sunset(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sunset(),
            &java_calendar.get_sunset(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sunrise(),
            &java_calendar.get_sunrise(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sea_level_sunrise(),
            &java_calendar.get_sea_level_sunrise(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sunrise_offset_by_degrees(test_case.zenith),
            &java_calendar.get_sunrise_offset_by_degrees(test_case.zenith),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sunset_offset_by_degrees(test_case.zenith),
            &java_calendar.get_sunset_offset_by_degrees(test_case.zenith),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_begin_civil_twilight(),
            &java_calendar.get_begin_civil_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_begin_nautical_twilight(),
            &java_calendar.get_begin_nautical_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_begin_astronomical_twilight(),
            &java_calendar.get_begin_astronomical_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_end_civil_twilight(),
            &java_calendar.get_end_civil_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_end_nautical_twilight(),
            &java_calendar.get_end_nautical_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_end_astronomical_twilight(),
            &java_calendar.get_end_astronomical_twilight(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_solar_midnight(),
            &java_calendar.get_solar_midnight(),
            0,
            &message,
        );

        let rust_temporal = rust_calendar.get_temporal_hour();
        let java_temporal = java_calendar.get_temporal_hour();
        assert_eq!(
            rust_temporal, java_temporal,
            "Temporal hour mismatch: rust={:?}, java={:?}, {}",
            rust_temporal, java_temporal, message
        );
        let rust_temporal_with_start_and_end_times = rust_calendar
            .get_temporal_hour_with_start_and_end_times(test_case.start_time, test_case.end_time);
        let java_temporal_with_start_and_end_times = java_calendar
            .get_temporal_hour_with_start_and_end_times(test_case.start_time, test_case.end_time);
        assert_almost_equal_i64_option(
            &rust_temporal_with_start_and_end_times,
            &java_temporal_with_start_and_end_times,
            10,
            &message,
        );

        assert_almost_equal_i64_option(
            &rust_calendar.get_sun_transit(),
            &java_calendar.get_sun_transit(),
            0,
            &message,
        );

        let rust_sun_transit_with_start_and_end_times = rust_calendar
            .get_sun_transit_with_start_and_end_times(test_case.start_time, test_case.end_time);
        let java_sun_transit_with_start_and_end_times = java_calendar
            .get_sun_transit_with_start_and_end_times(test_case.start_time, test_case.end_time);
        let result = match (
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
        ) {
            (Some(rust_time), Some(java_time)) => rust_time - java_time,
            _ => 0,
        };
        assert!(
            result.abs() <= 128,
            "Sun transit with start and end times mismatch: rust={:?}, java={:?}, distance: {}, {}",
            rust_sun_transit_with_start_and_end_times,
            java_sun_transit_with_start_and_end_times,
            result.abs(),
            message
        );
        drop(java_calendar.instance);
    }
}
