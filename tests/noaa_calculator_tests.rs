use zmanim_core::prelude::*;

mod test_utils;
use test_utils::*;
mod java;
use java::noaa_calculator::JavaNOAACalculator;

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

        let java_result = java_calculator
            .get_utc_noon(test_case.timestamp, &geo_location)
            .expect(&message);
        let rust_result = rust_calculator
            .get_utc_noon(test_case.timestamp, &geo_location)
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator
            .get_utc_midnight(test_case.timestamp, &geo_location)
            .expect(&message);
        let rust_result = rust_calculator
            .get_utc_midnight(test_case.timestamp, &geo_location)
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator
            .get_utc_sunrise(
                test_case.timestamp,
                &geo_location,
                test_case.zenith,
                test_case.use_elevation,
            )
            .expect(&message);
        let rust_result = rust_calculator
            .get_utc_sunrise(
                test_case.timestamp,
                &geo_location,
                test_case.zenith,
                test_case.use_elevation,
            )
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator
            .get_utc_sunset(
                test_case.timestamp,
                &geo_location,
                test_case.zenith,
                test_case.use_elevation,
            )
            .expect(&message);
        let rust_result = rust_calculator
            .get_utc_sunset(
                test_case.timestamp,
                &geo_location,
                test_case.zenith,
                test_case.use_elevation,
            )
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator
            .get_solar_elevation(test_case.timestamp, &geo_location)
            .expect(&message);
        let rust_result = rust_calculator
            .get_solar_elevation(test_case.timestamp, &geo_location)
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);

        let java_result = java_calculator
            .get_solar_azimuth(test_case.timestamp, &geo_location)
            .expect(&message);
        let rust_result = rust_calculator
            .get_solar_azimuth(test_case.timestamp, &geo_location)
            .expect(&message);
        assert_almost_equal_f64(java_result, rust_result, DEFAULT_FLOAT_TOLERANCE, &message);
    }
}
