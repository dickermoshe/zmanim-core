use core::f64;
use core::f64::consts::PI;
use libm::{atan, atan2, cos, log, sin, sqrt, tan};
#[cfg(feature = "uniffi")]
use std::sync::Arc;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
#[derive(Debug, Clone, PartialEq, Copy)]
#[repr(C)]
pub struct GeoLocation {
    pub latitude: f64,

    pub longitude: f64,

    pub elevation: f64,
}

#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
pub enum Formula {
    Distance = 0,
    InitialBearing = 1,
    FinalBearing = 2,
}

pub trait GeoLocationTrait {
    fn geodesic_initial_bearing(&self, location: &GeoLocation) -> Option<f64>;
    fn geodesic_final_bearing(&self, location: &GeoLocation) -> Option<f64>;
    fn geodesic_distance(&self, location: &GeoLocation) -> Option<f64>;
    fn rhumb_line_bearing(&self, location: &GeoLocation) -> f64;
    fn rhumb_line_distance(&self, location: &GeoLocation) -> f64;
    fn vincenty_inverse_formula(&self, location: &GeoLocation, formula: Formula) -> Option<f64>;
    fn get_latitude(&self) -> f64;
    fn get_longitude(&self) -> f64;
    fn get_elevation(&self) -> f64;
}

impl GeoLocation {
    pub fn new(latitude: f64, longitude: f64, elevation: f64) -> Option<Self> {
        if !(-90.0..=90.0).contains(&latitude) {
            return None;
        }
        if !(-180.0..=180.0).contains(&longitude) {
            return None;
        }
        if elevation < 0.0 {
            return None;
        }

        Some(GeoLocation {
            latitude,
            longitude,
            elevation,
        })
    }
}
#[cfg(feature = "uniffi")]
#[uniffi::export]
pub fn new_geolocation(latitude: f64, longitude: f64, elevation: f64) -> Option<Arc<GeoLocation>> {
    GeoLocation::new(latitude, longitude, elevation).map(Arc::new)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl GeoLocationTrait for GeoLocation {
    fn geodesic_initial_bearing(&self, location: &GeoLocation) -> Option<f64> {
        self.vincenty_inverse_formula(location, Formula::InitialBearing)
    }

    fn geodesic_final_bearing(&self, location: &GeoLocation) -> Option<f64> {
        self.vincenty_inverse_formula(location, Formula::FinalBearing)
    }

    fn geodesic_distance(&self, location: &GeoLocation) -> Option<f64> {
        self.vincenty_inverse_formula(location, Formula::Distance)
    }

    fn vincenty_inverse_formula(&self, location: &GeoLocation, formula: Formula) -> Option<f64> {
        let major_semi_axis = 6378137.0;
        let minor_semi_axis = 6356752.3142;
        let f = 1.0 / 298.257223563;
        let l = (location.longitude - self.longitude).to_radians();
        let u1 = atan((1.0 - f) * tan(self.latitude.to_radians()));
        let u2 = atan((1.0 - f) * tan(location.latitude.to_radians()));
        let sin_u1 = sin(u1);
        let cos_u1 = cos(u1);
        let sin_u2 = sin(u2);
        let cos_u2 = cos(u2);

        let mut lambda = l;
        let mut lambda_p = 2.0 * PI;
        let mut iter_limit = 20;
        let mut sin_lambda = 0.0;
        let mut cos_lambda = 0.0;
        let mut sin_sigma = 0.0;
        let mut cos_sigma = 0.0;
        let mut sigma = 0.0;
        #[allow(unused_assignments)]
        let mut sin_alpha = 0.0;
        let mut cos_sq_alpha = 0.0;
        let mut cos2_sigma_m = 0.0;

        while (lambda - lambda_p).abs() > 1e-12 && iter_limit > 0 {
            sin_lambda = sin(lambda);
            cos_lambda = cos(lambda);
            sin_sigma = sqrt(
                (cos_u2 * sin_lambda) * (cos_u2 * sin_lambda)
                    + (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda)
                        * (cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda),
            );

            if sin_sigma == 0.0 {
                return Some(0.0);
            }

            cos_sigma = sin_u1 * sin_u2 + cos_u1 * cos_u2 * cos_lambda;
            sigma = atan2(sin_sigma, cos_sigma);
            sin_alpha = cos_u1 * cos_u2 * sin_lambda / sin_sigma;
            cos_sq_alpha = 1.0 - sin_alpha * sin_alpha;
            cos2_sigma_m = cos_sigma - 2.0 * sin_u1 * sin_u2 / cos_sq_alpha;

            if cos2_sigma_m.is_nan() {
                cos2_sigma_m = 0.0;
            }

            let c = f / 16.0 * cos_sq_alpha * (4.0 + f * (4.0 - 3.0 * cos_sq_alpha));
            lambda_p = lambda;
            lambda = l
                + (1.0 - c)
                    * f
                    * sin_alpha
                    * (sigma
                        + c * sin_sigma
                            * (cos2_sigma_m
                                + c * cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)));

            iter_limit -= 1;
        }

        if iter_limit == 0 {
            return None;
        }

        let u_sq = cos_sq_alpha
            * (major_semi_axis * major_semi_axis - minor_semi_axis * minor_semi_axis)
            / (minor_semi_axis * minor_semi_axis);
        let a = 1.0 + u_sq / 16384.0 * (4096.0 + u_sq * (-768.0 + u_sq * (320.0 - 175.0 * u_sq)));
        let b = u_sq / 1024.0 * (256.0 + u_sq * (-128.0 + u_sq * (74.0 - 47.0 * u_sq)));
        let delta_sigma = b
            * sin_sigma
            * (cos2_sigma_m
                + b / 4.0
                    * (cos_sigma * (-1.0 + 2.0 * cos2_sigma_m * cos2_sigma_m)
                        - b / 6.0
                            * cos2_sigma_m
                            * (-3.0 + 4.0 * sin_sigma * sin_sigma)
                            * (-3.0 + 4.0 * cos2_sigma_m * cos2_sigma_m)));
        let distance = minor_semi_axis * a * (sigma - delta_sigma);

        let fwd_az = atan2(
            cos_u2 * sin_lambda,
            cos_u1 * sin_u2 - sin_u1 * cos_u2 * cos_lambda,
        )
        .to_degrees();

        let rev_az = atan2(
            cos_u1 * sin_lambda,
            -sin_u1 * cos_u2 + cos_u1 * sin_u2 * cos_lambda,
        )
        .to_degrees();

        match formula {
            Formula::Distance => Some(distance),
            Formula::InitialBearing => Some(fwd_az),
            Formula::FinalBearing => Some(rev_az),
        }
    }

    fn rhumb_line_bearing(&self, location: &GeoLocation) -> f64 {
        let mut d_lon = (location.longitude - self.longitude).to_radians();
        let d_phi = log(tan(location.latitude.to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.latitude.to_radians() / 2.0 + PI / 4.0));

        if d_lon.abs() > PI {
            d_lon = if d_lon > 0.0 {
                -(2.0 * PI - d_lon)
            } else {
                2.0 * PI + d_lon
            };
        }

        atan2(d_lon, d_phi).to_degrees()
    }

    fn rhumb_line_distance(&self, location: &GeoLocation) -> f64 {
        let earth_radius = 6378137.0;
        let d_lat = location.latitude.to_radians() - self.latitude.to_radians();
        let mut d_lon = (location.longitude.to_radians() - self.longitude.to_radians()).abs();
        let d_phi = log(tan(location.latitude.to_radians() / 2.0 + PI / 4.0))
            - log(tan(self.latitude.to_radians() / 2.0 + PI / 4.0));
        let mut q = d_lat / d_phi;

        if !q.is_finite() {
            q = cos(self.latitude.to_radians());
        }

        if d_lon > PI {
            d_lon = 2.0 * PI - d_lon;
        }

        let d = sqrt(d_lat * d_lat + q * q * d_lon * d_lon);
        d * earth_radius
    }

    fn get_latitude(&self) -> f64 {
        self.latitude
    }

    fn get_longitude(&self) -> f64 {
        self.longitude
    }

    fn get_elevation(&self) -> f64 {
        self.elevation
    }
}
