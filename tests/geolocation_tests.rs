mod test_utils;
use test_utils::*;

mod java;
use java::geolocation::JavaGeoLocation;
use zmanim_core::prelude::*;

#[test]
fn test_geolocation() {
    let jvm = create_jvm();

    for _ in 0..DEFAULT_TEST_ITERATIONS {
        let test_geo = random_test_geolocation();
        let other_test_geo = random_test_geolocation();

        let rust_geolocation = GeoLocation::new(test_geo.lat, test_geo.lon, test_geo.elevation)
            .expect("Failed to create Rust GeoLocation");

        let rust_other_geolocation = GeoLocation::new(
            other_test_geo.lat,
            other_test_geo.lon,
            other_test_geo.elevation,
        )
        .expect("Failed to create Rust GeoLocation");

        let java_geolocation = JavaGeoLocation::new(&jvm, &rust_geolocation);

        let message = format!(
            "test_geo: {:?}, other_test_geo: {:?}",
            test_geo, other_test_geo
        );

        assert_eq!(
            java_geolocation.get_latitude(),
            rust_geolocation.get_latitude(),
            "{}",
            message
        );
        assert_eq!(
            java_geolocation.get_longitude(),
            rust_geolocation.get_longitude(),
            "{}",
            message
        );
        assert_eq!(
            java_geolocation.get_elevation(),
            rust_geolocation.get_elevation(),
            "{}",
            message
        );

        assert_almost_equal_f64_result(
            &java_geolocation.geodesic_initial_bearing(&rust_other_geolocation),
            &rust_geolocation.geodesic_initial_bearing(&rust_other_geolocation),
            DEFAULT_FLOAT_TOLERANCE,
            &message,
        );
        assert_almost_equal_f64_result(
            &java_geolocation.geodesic_final_bearing(&rust_other_geolocation),
            &rust_geolocation.geodesic_final_bearing(&rust_other_geolocation),
            DEFAULT_FLOAT_TOLERANCE,
            &message,
        );
        assert_almost_equal_f64_result(
            &java_geolocation.geodesic_distance(&rust_other_geolocation),
            &rust_geolocation.geodesic_distance(&rust_other_geolocation),
            DEFAULT_FLOAT_TOLERANCE,
            &message,
        );
        assert_almost_equal_f64(
            java_geolocation.rhumb_line_bearing(&rust_other_geolocation),
            rust_geolocation.rhumb_line_bearing(&rust_other_geolocation),
            DEFAULT_FLOAT_TOLERANCE,
            &message,
        );
        assert_almost_equal_f64(
            java_geolocation.rhumb_line_distance(&rust_other_geolocation),
            rust_geolocation.rhumb_line_distance(&rust_other_geolocation),
            DEFAULT_FLOAT_TOLERANCE,
            &message,
        );
    }
}
