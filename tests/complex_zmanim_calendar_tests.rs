mod test_utils;
use test_utils::*;

mod java;
use zmanim_core::prelude::*;

use crate::java::{
    complex_zmanim_calendar::JavaComplexZmanimCalendar, noaa_calculator::JavaNOAACalculator,
};

#[test]
fn test_complex_zmanim_calendar_comprehensive() {
    let jvm = create_jvm();
    let mut passed_tests = 0;
    for _ in 0..DEFAULT_TEST_ITERATIONS {
        let test_case = TestCase::new();

        let geo_location = GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation)
            .expect("Failed to create Rust GeoLocation");
        let java_complex_zmanim_calendar = JavaComplexZmanimCalendar::new(
            &jvm,
            test_case.timestamp,
            &geo_location,
            JavaNOAACalculator::new(&jvm),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
            test_case.ateret_torah_sunset_offset,
        );
        let complex_zmanim_calendar = ComplexZmanimCalendar::new(
            test_case.timestamp,
            geo_location.clone(),
            test_case.use_astronomical_chatzos,
            test_case.use_astronomical_chatzos_for_other_zmanim,
            test_case.candle_lighting_offset,
            test_case.ateret_torah_sunset_offset,
        );

        let message = format!("Passed {passed_tests} - test_case: {:?}", test_case);

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

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_60(),
            &java_complex_zmanim_calendar.get_alos_60(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_72_zmanis(),
            &java_complex_zmanim_calendar.get_alos_72_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_90(),
            &java_complex_zmanim_calendar.get_alos_90(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_96(),
            &java_complex_zmanim_calendar.get_alos_96(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_90_zmanis(),
            &java_complex_zmanim_calendar.get_alos_90_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_96_zmanis(),
            &java_complex_zmanim_calendar.get_alos_96_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_18_degrees(),
            &java_complex_zmanim_calendar.get_alos_18_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_19_degrees(),
            &java_complex_zmanim_calendar.get_alos_19_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_alos_19_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_alos_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_alos_baal_hatanya(),
            &java_complex_zmanim_calendar.get_alos_baal_hatanya(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_misheyakir_11_point_5_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_11_point_5_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_misheyakir_11_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_11_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_misheyakir_10_point_2_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_10_point_2_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_misheyakir_7_point_65_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_7_point_65_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_misheyakir_9_point_5_degrees(),
            &java_complex_zmanim_calendar.get_misheyakir_9_point_5_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_19_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_18_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_18_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes_zmanis(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_72_minutes_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_mga_90_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_mga_90_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_ateret_torah(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_ateret_torah(),
            10,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_shma_baal_hatanya(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_baal_hatanya(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_19_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_18_degrees(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_18_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_mga_72_minutes(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_mga_72_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_ateret_torah(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_ateret_torah(),
            5,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_sof_zman_tfila_baal_hatanya(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_baal_hatanya(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_7_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_7_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_5_point_95_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_5_point_95_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_60(),
            &java_complex_zmanim_calendar.get_tzais_60(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_72_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_72_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_90(),
            &java_complex_zmanim_calendar.get_tzais_90(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_90_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_90_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_96(),
            &java_complex_zmanim_calendar.get_tzais_96(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_96_zmanis(),
            &java_complex_zmanim_calendar.get_tzais_96_zmanis(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_tzais_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_18_degrees(),
            &java_complex_zmanim_calendar.get_tzais_18_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_19_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_19_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_ateret_torah(),
            &java_complex_zmanim_calendar.get_tzais_ateret_torah(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_baal_hatanya(),
            &java_complex_zmanim_calendar.get_tzais_baal_hatanya(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_tzais(),
            &java_complex_zmanim_calendar.get_tzais(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_alos_hashachar(),
            &java_complex_zmanim_calendar.get_alos_hashachar(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.zmanim_calendar.get_chatzos(),
            &java_complex_zmanim_calendar.get_chatzos(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_sof_zman_shma_gra(),
            &java_complex_zmanim_calendar.get_sof_zman_shma_gra(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .get_sof_zman_tfila_gra(),
            &java_complex_zmanim_calendar.get_sof_zman_tfila_gra(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .astronomical_calendar
                .get_sunrise(),
            &java_complex_zmanim_calendar.get_sunrise(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .astronomical_calendar
                .get_sunset(),
            &java_complex_zmanim_calendar.get_sunset(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .astronomical_calendar
                .get_sun_transit(),
            &java_complex_zmanim_calendar.get_sun_transit(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar
                .zmanim_calendar
                .astronomical_calendar
                .get_temporal_hour(),
            &java_complex_zmanim_calendar.get_temporal_hour(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_30_minutes(),
            &java_complex_zmanim_calendar.get_mincha_gedola_30_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_72_minutes(),
            &java_complex_zmanim_calendar.get_mincha_gedola_72_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_16_point_1_degrees(),
            &java_complex_zmanim_calendar.get_mincha_gedola_16_point_1_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_greater_than_30(),
            &java_complex_zmanim_calendar.get_mincha_gedola_greater_than_30(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_ateret_torah(),
            &java_complex_zmanim_calendar.get_mincha_gedola_ateret_torah(),
            10,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_mincha_gedola_baal_hatanya(),
            &java_complex_zmanim_calendar.get_mincha_gedola_baal_hatanya(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_13_point_24_degrees(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_13_point_24_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_58_point_5_minutes(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_58_point_5_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_rt_2_stars(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_rt_2_stars(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_yereim_18_minutes(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_yereim_18_minutes(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_bain_hashmashos_yereim_3_point_05_degrees(),
            &java_complex_zmanim_calendar.get_bain_hashmashos_yereim_3_point_05_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_37_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_37_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_61_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_61_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_4_point_8_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_4_point_8_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_5_point_88_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_5_point_88_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_6_point_45_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_6_point_45_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_7_point_083_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_7_point_083_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_7_point_67_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_7_point_67_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_8_point_5_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_8_point_5_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_9_point_3_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_9_point_3_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_9_point_75_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_9_point_75_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_50(),
            &java_complex_zmanim_calendar.get_tzais_50(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_65_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_65_degrees(),
            1,
            &message,
        );

        assert_almost_equal_i64_option(
            &complex_zmanim_calendar.get_tzais_geonim_3_point_676_degrees(),
            &java_complex_zmanim_calendar.get_tzais_geonim_3_point_676_degrees(),
            1,
            &message,
        );

        passed_tests += 1;
    }
}
