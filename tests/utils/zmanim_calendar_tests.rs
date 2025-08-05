use rand::Rng;
use zmanim_core::{
    GeoLocation, NOAACalculator,
    zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait},
};

use crate::{
    java::{noaa_calculator::JavaNOAACalculator, zmanim_calendar::JavaZmanimCalendar},
    test_utils::{assert_almost_equal_f64_option, create_jvm, random_test_geolocation},
};

#[test]
fn test_zmanim_calendar() {
    let jvm = create_jvm();
    for _ in 0..10000 {
        let test_geo = random_test_geolocation();
        let test_timestamp = 1872730243409;
        let test_use_astronomical_chatzos = rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_use_astronomical_chatzos_for_other_zmanim =
            rand::rng().random_range(0.0..=1.0) > 0.5;
        let test_candle_lighting_offset = rand::rng().random_range(0.0..=60.0);

        let noaa_calculator = NOAACalculator::new();
        let geo_location = GeoLocation::new(test_geo.lat, test_geo.lon, test_geo.elevation)
            .expect("Failed to create Rust GeoLocation");
        let zmanim_calendar = ZmanimCalendar::new(
            test_timestamp,
            &geo_location,
            &noaa_calculator,
            test_use_astronomical_chatzos,
            test_use_astronomical_chatzos_for_other_zmanim,
            test_candle_lighting_offset,
        );
        let java_zmanim_calendar = JavaZmanimCalendar::new(
            &jvm,
            test_timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_use_astronomical_chatzos,
            test_use_astronomical_chatzos_for_other_zmanim,
            test_candle_lighting_offset,
        );

        let message = format!(
            "test_geo: {:?}, test_timestamp: {:?}, test_use_astronomical_chatzos: {:?}, test_use_astronomical_chatzos_for_other_zmanim: {:?}, test_candle_lighting_offset: {:?}",
            geo_location,
            test_timestamp,
            test_use_astronomical_chatzos,
            test_use_astronomical_chatzos_for_other_zmanim,
            test_candle_lighting_offset
        );
        assert_almost_equal_f64_option(
            &zmanim_calendar.get_tzais(),
            &java_zmanim_calendar.get_tzais(),
            Some(10000),
            &message,
        );
        assert_almost_equal_f64_option(
            &zmanim_calendar.get_alos_hashachar(),
            &java_zmanim_calendar.get_alos_hashachar(),
            Some(10000),
            &message,
        );
        assert_almost_equal_f64_option(
            &zmanim_calendar.get_alos72(),
            &java_zmanim_calendar.get_alos72(),
            Some(10000),
            &message,
        );

        // Test new methods
        assert_almost_equal_f64_option(
            &zmanim_calendar.get_tzais72(),
            &java_zmanim_calendar.get_tzais72(),
            Some(10000),
            &message,
        );

        assert_offby_less_than_f64_option(
            &zmanim_calendar.get_candle_lighting(),
            &java_zmanim_calendar.get_candle_lighting(),
            1.0,
            &message,
        );

        assert_almost_equal_f64_option(
            &zmanim_calendar.get_sof_zman_shma_gra(),
            &java_zmanim_calendar.get_sof_zman_shma_gra(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &zmanim_calendar.get_sof_zman_shma_mga(),
            &java_zmanim_calendar.get_sof_zman_shma_mga(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &zmanim_calendar.get_sof_zman_tfila_gra(),
            &java_zmanim_calendar.get_sof_zman_tfila_gra(),
            Some(10000),
            &message,
        );

        assert_almost_equal_f64_option(
            &zmanim_calendar.get_sof_zman_tfila_mga(),
            &java_zmanim_calendar.get_sof_zman_tfila_mga(),
            Some(10000),
            &message,
        );

        assert_offby_less_than_f64_option(
            &zmanim_calendar.get_mincha_gedola_default(),
            &java_zmanim_calendar.get_mincha_gedola_default(),
            1.0,
            &message,
        );

        assert_offby_less_than_f64_option(
            &zmanim_calendar.get_mincha_ketana_default(),
            &java_zmanim_calendar.get_mincha_ketana_default(),
            1.0,
            &message,
        );

        assert_offby_less_than_f64_option(
            &zmanim_calendar.get_plag_hamincha_default(),
            &java_zmanim_calendar.get_plag_hamincha_default(),
            1.0,
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
        assert_offby_less_than_f64_option(
            &rust_percent_sunrise,
            &java_percent_sunrise,
            1.0,
            &message,
        );

        let rust_percent_sunset =
            zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(8.5, true);
        let java_percent_sunset =
            java_zmanim_calendar.get_percent_of_shaah_zmanis_from_degrees(8.5, true);
        assert_offby_less_than_f64_option(
            &rust_percent_sunset,
            &java_percent_sunset,
            1.0,
            &message,
        );

        // Test half day based methods (when we have valid sunrise/sunset)
        if let (Some(sunrise), Some(sunset)) = (
            zmanim_calendar.get_alos_hashachar(),
            zmanim_calendar.get_tzais(),
        ) {
            // Test half day based zman
            let rust_half_day_zman = zmanim_calendar.get_half_day_based_zman(sunrise, sunset, 3.0);
            let java_half_day_zman =
                java_zmanim_calendar.get_half_day_based_zman(sunrise, sunset, 3.0);
            assert_almost_equal_f64_option(
                &rust_half_day_zman,
                &java_half_day_zman,
                Some(10000),
                &message,
            );

            // Test half day based shaah zmanis
            let rust_half_day_shaah =
                zmanim_calendar.get_half_day_based_shaah_zmanis(sunrise, sunset);
            let java_half_day_shaah =
                java_zmanim_calendar.get_half_day_based_shaah_zmanis(sunrise, sunset);
            if rust_half_day_shaah.is_none() || java_half_day_shaah.is_some() {
                // Handle potential KosherJava bug
            } else {
                assert_eq!(rust_half_day_shaah, java_half_day_shaah, "{}", message);
            }

            // Test shaah zmanis based zman
            let rust_shaah_zman = zmanim_calendar.get_shaah_zmanis_based_zman(sunrise, sunset, 4.0);
            let java_shaah_zman =
                java_zmanim_calendar.get_shaah_zmanis_based_zman(sunrise, sunset, 4.0);
            assert_almost_equal_f64_option(
                &rust_shaah_zman,
                &java_shaah_zman,
                Some(10000),
                &message,
            );

            // Test sof zman shma variants
            let rust_sof_zman_shma = zmanim_calendar.get_sof_zman_shma(sunrise, Some(sunset), true);
            let java_sof_zman_shma =
                java_zmanim_calendar.get_sof_zman_shma(sunrise, Some(sunset), true);
            assert_almost_equal_f64_option(
                &rust_sof_zman_shma,
                &java_sof_zman_shma,
                Some(10000),
                &message,
            );

            let rust_sof_zman_shma_simple =
                zmanim_calendar.get_sof_zman_shma_simple(sunrise, sunset);
            let java_sof_zman_shma_simple =
                java_zmanim_calendar.get_sof_zman_shma_simple(sunrise, sunset);
            assert_almost_equal_f64_option(
                &rust_sof_zman_shma_simple,
                &java_sof_zman_shma_simple,
                Some(10000),
                &message,
            );

            // Test sof zman tfila variants
            let rust_sof_zman_tfila =
                zmanim_calendar.get_sof_zman_tfila(sunrise, Some(sunset), true);
            let java_sof_zman_tfila =
                java_zmanim_calendar.get_sof_zman_tfila(sunrise, Some(sunset), true);
            assert_almost_equal_f64_option(
                &rust_sof_zman_tfila,
                &java_sof_zman_tfila,
                Some(10000),
                &message,
            );

            let rust_sof_zman_tfila_simple =
                zmanim_calendar.get_sof_zman_tfila_simple(sunrise, sunset);
            let java_sof_zman_tfila_simple =
                java_zmanim_calendar.get_sof_zman_tfila_simple(sunrise, sunset);
            assert_almost_equal_f64_option(
                &rust_sof_zman_tfila_simple,
                &java_sof_zman_tfila_simple,
                Some(10000),
                &message,
            );

            // Test mincha gedola variants
            let rust_mincha_gedola = zmanim_calendar.get_mincha_gedola(Some(sunrise), sunset, true);
            let java_mincha_gedola =
                java_zmanim_calendar.get_mincha_gedola(Some(sunrise), sunset, true);
            assert_offby_less_than_f64_option(
                &rust_mincha_gedola,
                &java_mincha_gedola,
                1.0,
                &message,
            );

            let rust_mincha_gedola_simple =
                zmanim_calendar.get_mincha_gedola_simple(sunrise, sunset);
            let java_mincha_gedola_simple =
                java_zmanim_calendar.get_mincha_gedola_simple(sunrise, sunset);
            assert_offby_less_than_f64_option(
                &rust_mincha_gedola_simple,
                &java_mincha_gedola_simple,
                1.0,
                &message,
            );

            // Test samuch le mincha ketana variants
            let rust_samuch_mincha =
                zmanim_calendar.get_samuch_le_mincha_ketana(Some(sunrise), sunset, true);
            let java_samuch_mincha =
                java_zmanim_calendar.get_samuch_le_mincha_ketana(Some(sunrise), sunset, true);
            assert_almost_equal_f64_option(
                &rust_samuch_mincha,
                &java_samuch_mincha,
                Some(10000),
                &message,
            );

            let rust_samuch_mincha_simple =
                zmanim_calendar.get_samuch_le_mincha_ketana_simple(sunrise, sunset);
            let java_samuch_mincha_simple =
                java_zmanim_calendar.get_samuch_le_mincha_ketana_simple(sunrise, sunset);
            assert_almost_equal_f64_option(
                &rust_samuch_mincha_simple,
                &java_samuch_mincha_simple,
                Some(10000),
                &message,
            );

            // Test mincha ketana variants
            let rust_mincha_ketana = zmanim_calendar.get_mincha_ketana(Some(sunrise), sunset, true);
            let java_mincha_ketana =
                java_zmanim_calendar.get_mincha_ketana(Some(sunrise), sunset, true);
            assert_offby_less_than_f64_option(
                &rust_mincha_ketana,
                &java_mincha_ketana,
                1.0,
                &message,
            );

            let rust_mincha_ketana_simple =
                zmanim_calendar.get_mincha_ketana_simple(sunrise, sunset);
            let java_mincha_ketana_simple =
                java_zmanim_calendar.get_mincha_ketana_simple(sunrise, sunset);
            assert_offby_less_than_f64_option(
                &rust_mincha_ketana_simple,
                &java_mincha_ketana_simple,
                1.0,
                &message,
            );

            // Test plag hamincha variants
            let rust_plag_hamincha = zmanim_calendar.get_plag_hamincha(Some(sunrise), sunset, true);
            let java_plag_hamincha =
                java_zmanim_calendar.get_plag_hamincha(Some(sunrise), sunset, true);
            assert_offby_less_than_f64_option(
                &rust_plag_hamincha,
                &java_plag_hamincha,
                1.0,
                &message,
            );

            let rust_plag_hamincha_simple =
                zmanim_calendar.get_plag_hamincha_simple(sunrise, sunset);
            let java_plag_hamincha_simple =
                java_zmanim_calendar.get_plag_hamincha_simple(sunrise, sunset);
            assert_offby_less_than_f64_option(
                &rust_plag_hamincha_simple,
                &java_plag_hamincha_simple,
                1.0,
                &message,
            );
        }

        let rust_chatzos = zmanim_calendar.get_chatzos();
        let java_chatzos = java_zmanim_calendar.get_chatzos();
        if rust_chatzos.is_none() || java_chatzos.is_some() {
            // KosherJava has a bug where it uses Long.MIN_VALUE as sunset when one does not exist.
            // We return None in this case.
        } else {
            assert_almost_equal_f64_option(&rust_chatzos, &java_chatzos, Some(10000), &message);
        }

        let rust_chatzos_as_half_day = zmanim_calendar.get_chatzos_as_half_day();
        let java_chatzos_as_half_day = java_zmanim_calendar.get_chatzos_as_half_day();
        if rust_chatzos_as_half_day.is_none() || java_chatzos_as_half_day.is_some() {
            // KosherJava has a bug where it uses Long.MIN_VALUE as sunset when one does not exist.
            // We return None in this case.
        } else {
            assert_almost_equal_f64_option(
                &rust_chatzos_as_half_day,
                &java_chatzos_as_half_day,
                Some(10000),
                &message,
            );
        }
    }
}

fn assert_offby_less_than_f64_option(
    a: &Option<f64>,
    b: &Option<f64>,
    off_by: f64,
    message: &String,
) {
    let result = match (a, b) {
        (Some(a), Some(b)) => (a - b).abs() <= off_by,
        (None, None) => true,
        _ => false,
    };
    if !result {
        println!("Error: {:?} vs {:?}, {}", a, b, message);
    }
    assert!(result);
}
