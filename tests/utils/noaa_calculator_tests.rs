use crate::{
    java::noaa_calculator::JavaNOAACalculator,
    test_utils::{
        assert_almost_equal_f64, create_jvm, TestCase, DEFAULT_FLOAT_TOLERANCE,
        DEFAULT_TEST_ITERATIONS,
    },
};
use zmanim_core::{GeoLocation, NOAACalculator, NOAACalculatorTrait};

#[test]
fn test_noaa_calculator() {
    let jvm = create_jvm();

    for _ in 0..DEFAULT_TEST_ITERATIONS {
        let test_case = TestCase::new();

        let geo_location =
            GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation).unwrap();

        let java_calculator = JavaNOAACalculator::new(&jvm);
        let rust_calculator = NOAACalculator::new();

        let message = format!("test_case: {:?}", test_case);

        let java_result = java_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_julian_day(test_case.timestamp);
        let rust_result = rust_calculator.get_julian_day(test_case.timestamp);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);
        let julian_day = java_result as f64;

        let java_result = java_calculator.get_julian_centuries_from_julian_day(julian_day);
        let rust_result = rust_calculator.get_julian_centuries_from_julian_day(julian_day);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);
        let julian_centuries = java_result as f64;

        let java_result = java_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_earth_orbit_eccentricity(julian_centuries);
        let rust_result = rust_calculator.get_earth_orbit_eccentricity(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_equation_of_center(julian_centuries);
        let rust_result = rust_calculator.get_sun_equation_of_center(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_true_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_true_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_apparent_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_apparent_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        let rust_result = rust_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_obliquity_correction(julian_centuries);
        let rust_result = rust_calculator.get_obliquity_correction(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_declination(julian_centuries);
        let rust_result = rust_calculator.get_sun_declination(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_equation_of_time(julian_centuries);
        let rust_result = rust_calculator.get_equation_of_time(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_hour_angle(
            test_case.lat,
            test_case.solar_declination,
            test_case.zenith,
            test_case.solar_event,
        );
        let rust_result = rust_calculator.get_sun_hour_angle(
            test_case.lat,
            test_case.solar_declination,
            test_case.zenith,
            test_case.solar_event,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_solar_noon_midnight_utc(
            julian_day,
            test_case.lon,
            test_case.solar_event,
        );
        let rust_result = rust_calculator.get_solar_noon_midnight_utc(
            julian_day,
            test_case.lon,
            test_case.solar_event,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_utc_noon(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_noon(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_utc_midnight(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_midnight(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_sun_rise_set_utc(
            test_case.timestamp,
            test_case.lat,
            test_case.lon,
            test_case.zenith,
            test_case.solar_event,
        );
        let rust_result = rust_calculator.get_sun_rise_set_utc(
            test_case.timestamp,
            test_case.lat,
            test_case.lon,
            test_case.zenith,
            test_case.solar_event,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_elevation_adjustment(test_case.elevation);
        let rust_result = rust_calculator.get_elevation_adjustment(test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_utc_sunrise(
            test_case.timestamp,
            &geo_location,
            test_case.zenith,
            test_case.use_elevation,
        );
        let rust_result = rust_calculator.get_utc_sunrise(
            test_case.timestamp,
            &geo_location,
            test_case.zenith,
            test_case.use_elevation,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_utc_sunset(
            test_case.timestamp,
            &geo_location,
            test_case.zenith,
            test_case.use_elevation,
        );
        let rust_result = rust_calculator.get_utc_sunset(
            test_case.timestamp,
            &geo_location,
            test_case.zenith,
            test_case.use_elevation,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_solar_elevation_azimuth(
            test_case.timestamp,
            &geo_location,
            test_case.is_azimuth,
        );
        let rust_result = rust_calculator.get_solar_elevation_azimuth(
            test_case.timestamp,
            &geo_location,
            test_case.is_azimuth,
        );
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_solar_elevation(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_elevation(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator.get_solar_azimuth(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_azimuth(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);
    }
}
