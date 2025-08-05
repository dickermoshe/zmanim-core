use crate::{
    java::noaa_calculator::JavaNOAACalculator,
    test_utils::{assert_almost_equal_f64, create_jvm, random_test_geolocation},
};
use rand::Rng;
use zmanim_core::{GeoLocation, NOAACalculator, NOAACalculatorTrait, SolarEvent};

#[test]
fn test_noaa_calculator() {
    let mut rng = rand::rng();
    let jvm = create_jvm();

    for _ in 0..10000 {
        let test_case = random_test_geolocation();
        let timestamp = rng.random_range(-4102444800000..=4102444800000);
        let zenith = rng.random_range(0.0..=180.0);
        let use_elevation = rng.random_bool(0.5);
        let solar_event = match rng.random_range(0..=3) {
            0 => SolarEvent::Sunrise,
            1 => SolarEvent::Sunset,
            2 => SolarEvent::Noon,
            3 => SolarEvent::Midnight,
            _ => unreachable!(),
        };
        let solar_declination = rng.random_range(-23.0..=23.0);
        let is_azimuth = rng.random_bool(0.5);

        let geo_location =
            GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation).unwrap();

        let java_calculator = JavaNOAACalculator::new(&jvm);
        let rust_calculator = NOAACalculator::new();

        let message = format!(
            "test_case: {:?}, timestamp: {:?}, zenith: {:?}, use_elevation: {:?}, solar_event: {:?}, solar_declination: {:?}, is_azimuth: {:?}",
            test_case, timestamp, zenith, use_elevation, solar_event, solar_declination, is_azimuth
        );

        let java_result = java_calculator.adjust_zenith(zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_julian_day(timestamp);
        let rust_result = rust_calculator.get_julian_day(timestamp);
        assert_almost_equal_f64(java_result, rust_result, None, &message);
        let julian_day = java_result as f64;

        let java_result = java_calculator.get_julian_centuries_from_julian_day(julian_day);
        let rust_result = rust_calculator.get_julian_centuries_from_julian_day(julian_day);
        assert_almost_equal_f64(java_result, rust_result, None, &message);
        let julian_centuries = java_result as f64;

        let java_result = java_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_earth_orbit_eccentricity(julian_centuries);
        let rust_result = rust_calculator.get_earth_orbit_eccentricity(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_equation_of_center(julian_centuries);
        let rust_result = rust_calculator.get_sun_equation_of_center(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_true_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_true_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_apparent_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_apparent_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        let rust_result = rust_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_obliquity_correction(julian_centuries);
        let rust_result = rust_calculator.get_obliquity_correction(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        // let java_result = java_calculator.get_sun_declination(julian_centuries);
        // let rust_result = rust_calculator.get_sun_declination(julian_centuries);
        // assert_almost_equal_f64(java_result, rust_result);

        let java_result = java_calculator.get_equation_of_time(julian_centuries);
        let rust_result = rust_calculator.get_equation_of_time(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_hour_angle(
            test_case.lat,
            solar_declination,
            zenith,
            solar_event,
        );
        let rust_result = rust_calculator.get_sun_hour_angle(
            test_case.lat,
            solar_declination,
            zenith,
            solar_event,
        );
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result =
            java_calculator.get_solar_noon_midnight_utc(julian_day, test_case.lon, solar_event);
        let rust_result =
            rust_calculator.get_solar_noon_midnight_utc(julian_day, test_case.lon, solar_event);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_utc_noon(timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_noon(timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_utc_midnight(timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_midnight(timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_sun_rise_set_utc(
            timestamp,
            test_case.lat,
            test_case.lon,
            zenith,
            solar_event,
        );
        let rust_result = rust_calculator.get_sun_rise_set_utc(
            timestamp,
            test_case.lat,
            test_case.lon,
            zenith,
            solar_event,
        );
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_elevation_adjustment(test_case.elevation);
        let rust_result = rust_calculator.get_elevation_adjustment(test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.adjust_zenith(zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result =
            java_calculator.get_utc_sunrise(timestamp, &geo_location, zenith, use_elevation);
        let rust_result =
            rust_calculator.get_utc_sunrise(timestamp, &geo_location, zenith, use_elevation);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result =
            java_calculator.get_utc_sunset(timestamp, &geo_location, zenith, use_elevation);
        let rust_result =
            rust_calculator.get_utc_sunset(timestamp, &geo_location, zenith, use_elevation);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result =
            java_calculator.get_solar_elevation_azimuth(timestamp, &geo_location, is_azimuth);
        let rust_result =
            rust_calculator.get_solar_elevation_azimuth(timestamp, &geo_location, is_azimuth);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_solar_elevation(timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_elevation(timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, None, &message);

        let java_result = java_calculator.get_solar_azimuth(timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_azimuth(timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, None, &message);
    }
}
