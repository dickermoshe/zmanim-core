use crate::{
    java::noaa_calculator::JavaNOAACalculator,
    test_utils::{assert_almost_equal_f64, create_jvm},
};
use rand::Rng;
use zmanim_core::{GeoLocation, NOAACalculator, NOAACalculatorTrait, SolarEvent};
#[derive(Debug)]
struct TestCase {
    lat: f64,
    lon: f64,
    elevation: f64,
    timestamp: i64,
    zenith: f64,
    use_elevation: bool,
    solar_event: SolarEvent,
    solar_declination: f64,
    is_azimuth: bool,
}

impl TestCase {
    fn new() -> Self {
        let mut rng = rand::rng();
        let lat = rng.random_range(-90.0..=90.0);
        let lon = rng.random_range(-180.0..=180.0);
        let elevation = rng.random_range(0.0..=1000.0);
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

        TestCase {
            lat,
            lon,
            elevation,
            timestamp,
            zenith,
            use_elevation,
            solar_event,
            solar_declination,
            is_azimuth,
        }
    }
}

#[test]
fn test_noaa_calculator() {
    let jvm = create_jvm();

    for _ in 0..10_000 {
        let test_case = TestCase::new();

        let geo_location =
            GeoLocation::new(test_case.lat, test_case.lon, test_case.elevation).unwrap();

        let java_calculator = JavaNOAACalculator::new(&jvm);
        let rust_calculator = NOAACalculator::new();

        let message = format!("test_case: {:?}", test_case);

        let java_result = java_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_julian_day(test_case.timestamp);
        let rust_result = rust_calculator.get_julian_day(test_case.timestamp);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);
        let julian_day = java_result as f64;

        let java_result = java_calculator.get_julian_centuries_from_julian_day(julian_day);
        let rust_result = rust_calculator.get_julian_centuries_from_julian_day(julian_day);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);
        let julian_centuries = java_result as f64;

        let java_result = java_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        let rust_result = rust_calculator.get_sun_geometric_mean_anomaly(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_earth_orbit_eccentricity(julian_centuries);
        let rust_result = rust_calculator.get_earth_orbit_eccentricity(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_sun_equation_of_center(julian_centuries);
        let rust_result = rust_calculator.get_sun_equation_of_center(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_sun_true_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_true_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_sun_apparent_longitude(julian_centuries);
        let rust_result = rust_calculator.get_sun_apparent_longitude(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        let rust_result = rust_calculator.get_mean_obliquity_of_ecliptic(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_obliquity_correction(julian_centuries);
        let rust_result = rust_calculator.get_obliquity_correction(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_sun_declination(julian_centuries);
        let rust_result = rust_calculator.get_sun_declination(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_equation_of_time(julian_centuries);
        let rust_result = rust_calculator.get_equation_of_time(julian_centuries);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_utc_noon(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_noon(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_utc_midnight(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_utc_midnight(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_elevation_adjustment(test_case.elevation);
        let rust_result = rust_calculator.get_elevation_adjustment(test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        let rust_result = rust_calculator.adjust_zenith(test_case.zenith, test_case.elevation);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

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
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_solar_elevation(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_elevation(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        let java_result = java_calculator.get_solar_azimuth(test_case.timestamp, &geo_location);
        let rust_result = rust_calculator.get_solar_azimuth(test_case.timestamp, &geo_location);
        assert_almost_equal_f64(java_result, rust_result, 0.00000001, &message);

        drop(java_calculator);
    }
}
