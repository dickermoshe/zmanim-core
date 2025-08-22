use rand::Rng;
use zmanim_core::{
    GeoLocation,
    zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait},
};

use crate::{
    java::{noaa_calculator::JavaNOAACalculator, zmanim_calendar::JavaZmanimCalendar},
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
    candle_lighting_offset: i64,
}

impl TestCase {
    fn new() -> Self {
        let test_geo = random_test_geolocation();
        let test_timestamp = random_test_timestamp();
        let test_use_astronomical_chatzos = rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_use_astronomical_chatzos_for_other_zmanim =
            rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_candle_lighting_offset = rand::rng().random_range(0..=60 * 1000);
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
fn test_zmanim_calendar() {
    let jvm = create_jvm();
    for _ in 0..10_000 {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let zmanim_calendar = ZmanimCalendar::new(
            test_case.timestamp,
            &geo_location,
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );
        let java_zmanim_calendar = JavaZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
        );

        let message = format!("test_case: {:?}", test_case);
        assert_almost_equal_i64_option(
            &zmanim_calendar.get_tzais(),
            &java_zmanim_calendar.get_tzais(),
            0,
            &message,
        );
        assert_almost_equal_i64_option(
            &zmanim_calendar.get_alos_hashachar(),
            &java_zmanim_calendar.get_alos_hashachar(),
            0,
            &message,
        );
        assert_almost_equal_i64_option(
            &zmanim_calendar.get_alos72(),
            &java_zmanim_calendar.get_alos72(),
            0,
            &message,
        );

        // Test new methods
        assert_almost_equal_i64_option(
            &zmanim_calendar.get_tzais72(),
            &java_zmanim_calendar.get_tzais72(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_candle_lighting(),
            &java_zmanim_calendar.get_candle_lighting(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_sof_zman_shma_gra(),
            &java_zmanim_calendar.get_sof_zman_shma_gra(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_sof_zman_shma_mga(),
            &java_zmanim_calendar.get_sof_zman_shma_mga(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_sof_zman_tfila_gra(),
            &java_zmanim_calendar.get_sof_zman_tfila_gra(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_sof_zman_tfila_mga(),
            &java_zmanim_calendar.get_sof_zman_tfila_mga(),
            0,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_mincha_gedola_default(),
            &java_zmanim_calendar.get_mincha_gedola_default(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_mincha_ketana_default(),
            &java_zmanim_calendar.get_mincha_ketana_default(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &zmanim_calendar.get_plag_hamincha_default(),
            &java_zmanim_calendar.get_plag_hamincha_default(),
            1,
            &message,
        );

        // Test shaah zmanis methods (handle None case for KosherJava bug)
        let rust_shaah_zmanis_gra = zmanim_calendar.get_shaah_zmanis_gra();
        let java_shaah_zmanis_gra = java_zmanim_calendar.get_shaah_zmanis_gra();

        assert_eq!(rust_shaah_zmanis_gra, java_shaah_zmanis_gra, "{}", message);

        let rust_shaah_zmanis_mga = zmanim_calendar.get_shaah_zmanis_mga();
        let java_shaah_zmanis_mga = java_zmanim_calendar.get_shaah_zmanis_mga();

        assert_eq!(rust_shaah_zmanis_mga, java_shaah_zmanis_mga, "{}", message);

        // Test percent of shaah zmanis from degrees
        let rust_percent_sunrise =
            zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(16.1, false);
        let java_percent_sunrise =
            java_zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(16.1, false);
        assert_almost_equal_f64_option(&rust_percent_sunrise, &java_percent_sunrise, 1.0, &message);

        let rust_percent_sunset =
            zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(8.5, true);
        let java_percent_sunset =
            java_zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(8.5, true);
        assert_almost_equal_f64_option(&rust_percent_sunset, &java_percent_sunset, 1.0, &message);

        // Test half day based methods (when we have valid sunrise/sunset)
        if let (Some(sunrise), Some(sunset)) = (
            zmanim_calendar.get_alos_hashachar(),
            zmanim_calendar.get_tzais(),
        ) {
            // Test half day based zman
            let rust_half_day_zman = zmanim_calendar.get_half_day_based_zman(sunrise, sunset, 3.0);
            let java_half_day_zman =
                java_zmanim_calendar.get_half_day_based_zman(sunrise, sunset, 3.0);
            assert_almost_equal_i64_option(&rust_half_day_zman, &java_half_day_zman, 0, &message);

            // Test half day based shaah zmanis
            let rust_half_day_shaah =
                zmanim_calendar.get_half_day_based_shaah_zmanis(sunrise, sunset);
            let java_half_day_shaah =
                java_zmanim_calendar.get_half_day_based_shaah_zmanis(sunrise, sunset);

            assert_eq!(rust_half_day_shaah, java_half_day_shaah, "{}", message);

            // Test shaah zmanis based zman
            let rust_shaah_zman = zmanim_calendar.get_shaah_zmanis_based_zman(sunrise, sunset, 4.0);
            let java_shaah_zman =
                java_zmanim_calendar.get_shaah_zmanis_based_zman(sunrise, sunset, 4.0);
            assert_almost_equal_i64_option(&rust_shaah_zman, &java_shaah_zman, 0, &message);

            // Test sof zman shma variants
            let rust_sof_zman_shma =
                zmanim_calendar._get_sof_zman_shma(sunrise, Some(sunset), true);
            let java_sof_zman_shma =
                java_zmanim_calendar._get_sof_zman_shma(sunrise, Some(sunset), true);
            assert_almost_equal_i64_option(&rust_sof_zman_shma, &java_sof_zman_shma, 0, &message);

            let rust_sof_zman_shma_simple =
                zmanim_calendar.get_sof_zman_shma_simple(sunrise, sunset);
            let java_sof_zman_shma_simple =
                java_zmanim_calendar.get_sof_zman_shma_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_sof_zman_shma_simple,
                &java_sof_zman_shma_simple,
                0,
                &message,
            );

            // Test sof zman tfila variants
            let rust_sof_zman_tfila =
                zmanim_calendar._get_sof_zman_tfila(sunrise, Some(sunset), true);
            let java_sof_zman_tfila =
                java_zmanim_calendar._get_sof_zman_tfila(sunrise, Some(sunset), true);
            assert_almost_equal_i64_option(&rust_sof_zman_tfila, &java_sof_zman_tfila, 0, &message);

            let rust_sof_zman_tfila_simple =
                zmanim_calendar.get_sof_zman_tfila_simple(sunrise, sunset);
            let java_sof_zman_tfila_simple =
                java_zmanim_calendar.get_sof_zman_tfila_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_sof_zman_tfila_simple,
                &java_sof_zman_tfila_simple,
                0,
                &message,
            );

            // Test mincha gedola variants
            let rust_mincha_gedola =
                zmanim_calendar._get_mincha_gedola(Some(sunrise), sunset, true);
            let java_mincha_gedola =
                java_zmanim_calendar._get_mincha_gedola(Some(sunrise), sunset, true);
            assert_almost_equal_i64_option(&rust_mincha_gedola, &java_mincha_gedola, 1, &message);

            let rust_mincha_gedola_simple =
                zmanim_calendar.get_mincha_gedola_simple(sunrise, sunset);
            let java_mincha_gedola_simple =
                java_zmanim_calendar.get_mincha_gedola_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_mincha_gedola_simple,
                &java_mincha_gedola_simple,
                1,
                &message,
            );

            // Test samuch le mincha ketana variants
            let rust_samuch_mincha =
                zmanim_calendar._get_samuch_le_mincha_ketana(Some(sunrise), sunset, true);
            let java_samuch_mincha =
                java_zmanim_calendar._get_samuch_le_mincha_ketana(Some(sunrise), sunset, true);
            assert_almost_equal_i64_option(&rust_samuch_mincha, &java_samuch_mincha, 0, &message);

            let rust_samuch_mincha_simple =
                zmanim_calendar.get_samuch_le_mincha_ketana_simple(sunrise, sunset);
            let java_samuch_mincha_simple =
                java_zmanim_calendar.get_samuch_le_mincha_ketana_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_samuch_mincha_simple,
                &java_samuch_mincha_simple,
                0,
                &message,
            );

            // Test mincha ketana variants
            let rust_mincha_ketana =
                zmanim_calendar._get_mincha_ketana(Some(sunrise), sunset, true);
            let java_mincha_ketana =
                java_zmanim_calendar._get_mincha_ketana(Some(sunrise), sunset, true);
            assert_almost_equal_i64_option(&rust_mincha_ketana, &java_mincha_ketana, 1, &message);

            let rust_mincha_ketana_simple =
                zmanim_calendar.get_mincha_ketana_simple(sunrise, sunset);
            let java_mincha_ketana_simple =
                java_zmanim_calendar.get_mincha_ketana_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_mincha_ketana_simple,
                &java_mincha_ketana_simple,
                1,
                &message,
            );

            // Test plag hamincha variants
            let rust_plag_hamincha =
                zmanim_calendar._get_plag_hamincha(Some(sunrise), sunset, true);
            let java_plag_hamincha =
                java_zmanim_calendar._get_plag_hamincha(Some(sunrise), sunset, true);
            assert_almost_equal_i64_option(&rust_plag_hamincha, &java_plag_hamincha, 1, &message);

            let rust_plag_hamincha_simple =
                zmanim_calendar.get_plag_hamincha_simple(sunrise, sunset);
            let java_plag_hamincha_simple =
                java_zmanim_calendar.get_plag_hamincha_simple(sunrise, sunset);
            assert_almost_equal_i64_option(
                &rust_plag_hamincha_simple,
                &java_plag_hamincha_simple,
                1,
                &message,
            );
        }

        let rust_chatzos = zmanim_calendar.get_chatzos();
        let java_chatzos = java_zmanim_calendar.get_chatzos();

        assert_almost_equal_i64_option(&rust_chatzos, &java_chatzos, 0, &message);

        let rust_chatzos_as_half_day = zmanim_calendar.get_chatzos_as_half_day();
        let java_chatzos_as_half_day = java_zmanim_calendar.get_chatzos_as_half_day();

        assert_almost_equal_i64_option(
            &rust_chatzos_as_half_day,
            &java_chatzos_as_half_day,
            0,
            &message,
        );
        drop(java_zmanim_calendar);
    }
}
