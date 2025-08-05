use zmanim_core::{GeoLocation, GeoLocationTrait};

use crate::{
    java::geolocation::JavaGeoLocation,
    test_utils::{
        assert_almost_equal_f64, assert_almost_equal_f64_result, create_jvm,
        random_test_geolocation,
    },
};

// Tested Against b3fedc6c2d028f9aecd61e775e56ac253dbb1548
#[test]
fn test_geolocation() {
    let jvm = create_jvm();

    // Test with shuffled locations
    for _ in 0..10000 {
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

        // Compare values between Rust and Java implementations
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
            None,
            &message,
        );
        assert_almost_equal_f64_result(
            &java_geolocation.geodesic_final_bearing(&rust_other_geolocation),
            &rust_geolocation.geodesic_final_bearing(&rust_other_geolocation),
            None,
            &message,
        );
        assert_almost_equal_f64_result(
            &java_geolocation.geodesic_distance(&rust_other_geolocation),
            &rust_geolocation.geodesic_distance(&rust_other_geolocation),
            None,
            &message,
        );
        assert_almost_equal_f64(
            java_geolocation.rhumb_line_bearing(&rust_other_geolocation),
            rust_geolocation.rhumb_line_bearing(&rust_other_geolocation),
            None,
            &message,
        );
        assert_almost_equal_f64(
            java_geolocation.rhumb_line_distance(&rust_other_geolocation),
            rust_geolocation.rhumb_line_distance(&rust_other_geolocation),
            None,
            &message,
        );
    }
}
