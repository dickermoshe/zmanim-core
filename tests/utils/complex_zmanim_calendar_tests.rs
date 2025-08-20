use rand::Rng;
use zmanim_core::{
    GeoLocation,
    astronomical_calendar::AstronomicalCalendarTrait,
    complex_zmanim_calendar::{ComplexZmanimCalendar, ComplexZmanimCalendarTrait},
    zmanim_calendar::ZmanimCalendarTrait,
};

use crate::{
    java::{
        complex_zmanim_calendar::JavaComplexZmanimCalendar, noaa_calculator::JavaNOAACalculator,
    },
    test_utils::{
        assert_almost_equal_f64_option, assert_almost_equal_i64_option, create_jvm,
        random_test_geolocation, random_test_timestamp,
    },
};

#[derive(Debug)]
struct TestCase {
    lat: f64,
    lon: f64,
    elevation: f64,
    timestamp: i64,
    use_astronomical_chatzos: bool,
    use_astronomical_chatzos_for_other_zmanim: bool,
    candle_lighting_offset: f64,
}

impl TestCase {
    fn new() -> Self {
        let test_geo = random_test_geolocation();
        let test_timestamp = random_test_timestamp();
        let test_use_astronomical_chatzos = rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_use_astronomical_chatzos_for_other_zmanim =
            rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_candle_lighting_offset = rand::rng().random_range(0.0..=60.0);
        Self {
            lat: test_geo.lat,
            lon: test_geo.lon,
            elevation: test_geo.elevation,
            timestamp: test_timestamp,
            use_astronomical_chatzos: test_use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim:
                test_use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset: test_candle_lighting_offset,
        }
    }
}

#[test]
fn test_complex_zmanim_calendar_shaah_zmanis() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Shaah Zmanis methods
        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_19_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_18_degrees(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_18_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_60_minutes(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_60_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_72_minutes(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_72_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_72_minutes_zmanis(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_72_minutes_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_90_minutes(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_90_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_shaah_zmanis_baal_hatanya(),
            &java_complex_zmanim_calendar.get_shaah_zmanis_baal_hatanya(),
            1,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_alos() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Alos methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_60(),
            &java_complex_zmanim_calendar.get_alos_60(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_72_zmanis(),
            &java_complex_zmanim_calendar.get_alos_72_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_90(),
            &java_complex_zmanim_calendar.get_alos_90(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_96(),
            &java_complex_zmanim_calendar.get_alos_96(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_90_zmanis(),
            &java_complex_zmanim_calendar.get_alos_90_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_96_zmanis(),
            &java_complex_zmanim_calendar.get_alos_96_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_18_degrees(),
            &java_complex_zmanim_calendar.get_alos_18_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_19_degrees(),
            &java_complex_zmanim_calendar.get_alos_19_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_alos_19_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_alos_16_point_1_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_alos_baal_hatanya(),
            &java_complex_zmanim_calendar.get_alos_baal_hatanya(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_misheyakir() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Misheyakir methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_misheyakir_11_point_5_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_11_point_5_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_misheyakir_11_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_11_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_misheyakir_10_point_2_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_10_point_2_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_misheyakir_7_point_65_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_7_point_65_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_misheyakir_9_point_5_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_9_point_5_degrees(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_sof_zman_shma() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Sof Zman Shma methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_19_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_16_point_1_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_18_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_18_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes_zmanis(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_90_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_90_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_ateret_torah(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_ateret_torah(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_baal_hatanya(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_baal_hatanya(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_sof_zman_tfila() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Sof Zman Tfila methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_19_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_16_point_1_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_18_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_18_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_72_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_72_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_ateret_torah(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_ateret_torah(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_baal_hatanya(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_baal_hatanya(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_tzais() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Tzais methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_7_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_7_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_5_point_95_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_5_point_95_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_60(),
            &java_complex_zmanim_calendar.get_tzais_60(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_72_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_72_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_90(),
            &java_complex_zmanim_calendar.get_tzais_90(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_90_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_90_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_96(),
            &java_complex_zmanim_calendar.get_tzais_96(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_96_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_96_zmanis(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_tzais_16_point_1_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_18_degrees(),
            &java_complex_zmanim_calendar.get_tzais_18_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_19_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_ateret_torah(),
            &java_complex_zmanim_calendar.get_tzais_ateret_torah(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_baal_hatanya(),
            &java_complex_zmanim_calendar.get_tzais_baal_hatanya(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_ateret_torah_offset() {
    let jvm = create_jvm();
    for _ in 0..100 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let mut complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let mut java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test getter
        assert!(
            (complex_zmanim_calendar.get_ateret_torah_sunset_offset()
                - java_complex_zmanim_calendar.get_ateret_torah_sunset_offset())
            .abs()
                < 0.00000001,
            "{}",
            message
        );

        // Test setter
        let new_offset = rand::rng().random_range(20.0..=60.0);
        complex_zmanim_calendar.set_ateret_torah_sunset_offset(new_offset);
        java_complex_zmanim_calendar.set_ateret_torah_sunset_offset(new_offset);

        assert!(
            (complex_zmanim_calendar.get_ateret_torah_sunset_offset()
                - java_complex_zmanim_calendar.get_ateret_torah_sunset_offset())
            .abs()
                < 0.00000001,
            "{}",
            message
        );
        assert!(
            (complex_zmanim_calendar.get_ateret_torah_sunset_offset() - new_offset).abs()
                < 0.00000001,
            "{}",
            message
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_base_traits() {
    let jvm = create_jvm();
    for _ in 0..1_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test inherited ZmanimCalendarTrait methods through composition
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_tzais(),
            &java_complex_zmanim_calendar.get_tzais(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_alos_hashachar(),
            &java_complex_zmanim_calendar.get_alos_hashachar(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_chatzos(),
            &java_complex_zmanim_calendar.get_chatzos(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_sof_zman_shma_gra(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_gra(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_sof_zman_tfila_gra(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_gra(),
            1.0,
            &message,
        );

        // Test inherited AstronomicalCalendarTrait methods through composition
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_astronomical_calendar()
                .get_sunrise(),
            &java_complex_zmanim_calendar.get_sunrise(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_astronomical_calendar()
                .get_sunset(),
            &java_complex_zmanim_calendar.get_sunset(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_astronomical_calendar()
                .get_sun_transit(),
            &java_complex_zmanim_calendar.get_sun_transit(),
            1.0,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_astronomical_calendar()
                .get_temporal_hour(),
            &java_complex_zmanim_calendar.get_temporal_hour(),
            1,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_comprehensive() {
    let jvm = create_jvm();
    for _ in 0..500 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test a comprehensive set of methods across different categories

        // Geonim Tzais methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_37_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_37_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_61_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_61_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_8_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_5_point_88_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_5_point_88_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_6_point_45_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_6_point_45_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_7_point_083_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_7_point_083_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_7_point_67_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_7_point_67_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_8_point_5_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_8_point_5_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_9_point_3_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_9_point_3_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_9_point_75_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_9_point_75_degrees(),
            1.0,
            &message,
        );

        // Test time-based tzais
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_50(),
            &java_complex_zmanim_calendar.get_tzais_50(),
            1.0,
            &message,
        );

        // Test deprecated methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_65_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_65_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_676_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_676_degrees(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_mincha_gedola() {
    let jvm = create_jvm();
    for _ in 0..500 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Mincha Gedola methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_30_minutes(),
            &java_complex_zmanim_calendar.get_mincha_gedola_30_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_72_minutes(),
            &java_complex_zmanim_calendar.get_mincha_gedola_72_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_mincha_gedola_16_point_1_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_greater_than_30(),
            &java_complex_zmanim_calendar.get_mincha_gedola_greater_than_30(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_ateret_torah(),
            &java_complex_zmanim_calendar.get_mincha_gedola_ateret_torah(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_mincha_gedola_baal_hatanya(),
            &java_complex_zmanim_calendar.get_mincha_gedola_baal_hatanya(),
            1.0,
            &message,
        );
    }
}

#[test]
fn test_complex_zmanim_calendar_bain_hashmashos() {
    let jvm = create_jvm();
    for _ in 0..500 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);

        // Test Bain Hashmashos methods
        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_13_point_24_degrees(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_13_point_24_degrees(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_58_point_5_minutes(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_58_point_5_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_2_stars(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_2_stars(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_yereim_18_minutes(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_yereim_18_minutes(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_yereim_3_point_05_degrees(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_yereim_3_point_05_degrees(),
            1.0,
            &message,
        );
    }
}
