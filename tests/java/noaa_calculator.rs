use j4rs::{Instance, InvocationArg, Jvm};
use zmanim_core::{GeoLocationTrait, NOAACalculatorTrait};

use crate::java::{
    calendar::create_calendar, geolocation::JavaGeoLocation, solar_event::create_solar_event,
};

pub struct JavaNOAACalculator<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}
impl<'a> JavaNOAACalculator<'a> {
    pub fn new(jvm: &'a Jvm) -> Self {
        Self {
            jvm,
            instance: jvm
                .create_instance(
                    "com.kosherjava.zmanim.util.NOAACalculator",
                    InvocationArg::empty(),
                )
                .unwrap(),
        }
    }
}

impl<'a> NOAACalculatorTrait for JavaNOAACalculator<'a> {
    fn get_julian_centuries_from_julian_day(&self, julian_day: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getJulianCenturiesFromJulianDay",
                &[InvocationArg::try_from(julian_day)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_geometric_mean_longitude(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunGeometricMeanLongitude",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_geometric_mean_anomaly(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunGeometricMeanAnomaly",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_earth_orbit_eccentricity(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getEarthOrbitEccentricity",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_equation_of_center(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunEquationOfCenter",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_true_longitude(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunTrueLongitude",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_apparent_longitude(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunApparentLongitude",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_mean_obliquity_of_ecliptic(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getMeanObliquityOfEcliptic",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_obliquity_correction(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getObliquityCorrection",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_declination(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunDeclination",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_equation_of_time(&self, julian_centuries: f64) -> f64 {
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getEquationOfTime",
                &[InvocationArg::try_from(julian_centuries)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_hour_angle(
        &self,
        latitude: f64,
        solar_declination: f64,
        zenith: f64,
        solar_event: zmanim_core::SolarEvent,
    ) -> f64 {
        let solar_event_instance = create_solar_event(&self.jvm, solar_event);
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunHourAngle",
                &[
                    InvocationArg::try_from(latitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(solar_declination)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(solar_event_instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_solar_noon_midnight_utc(
        &self,
        julian_day: f64,
        longitude: f64,
        solar_event: zmanim_core::SolarEvent,
    ) -> f64 {
        let solar_event_instance = create_solar_event(&self.jvm, solar_event);
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSolarNoonMidnightUTC",
                &[
                    InvocationArg::try_from(julian_day)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(longitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(solar_event_instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_noon(&self, timestamp: i64, geo_location: &dyn GeoLocationTrait) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCNoon",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_midnight(&self, timestamp: i64, geo_location: &dyn GeoLocationTrait) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCMidnight",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_sun_rise_set_utc(
        &self,
        timestamp: i64,
        latitude: f64,
        longitude: f64,
        zenith: f64,
        solar_event: zmanim_core::SolarEvent,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let solar_event_instance = create_solar_event(&self.jvm, solar_event);
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getSunRiseSetUTC",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(latitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(longitude)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(solar_event_instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_elevation_adjustment(&self, elevation: f64) -> f64 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getElevationAdjustment",
                &[InvocationArg::try_from(elevation)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn adjust_zenith(&self, zenith: f64, elevation: f64) -> f64 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "adjustZenith",
                &[
                    InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_sunrise(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunrise",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                    InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(adjust_for_elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_utc_sunset(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunset",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                    InvocationArg::try_from(zenith)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(adjust_for_elevation)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_solar_elevation_azimuth(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        is_azimuth: bool,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSolarElevationAzimuth",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                    InvocationArg::try_from(is_azimuth)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_solar_elevation(
        &self,
        timestamp: i64,
        geo_location: &zmanim_core::utils::geolocation::GeoLocation,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSolarElevation",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_solar_azimuth(
        &self,
        timestamp: i64,
        geo_location: &zmanim_core::utils::geolocation::GeoLocation,
    ) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let geolocation = JavaGeoLocation::new(&self.jvm, geo_location);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSolarAzimuth",
                &[
                    InvocationArg::try_from(calendar).unwrap(),
                    InvocationArg::try_from(geolocation.instance).unwrap(),
                ],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_julian_day(&self, timestamp: i64) -> f64 {
        let calendar = create_calendar(&self.jvm, timestamp);
        let result = self
            .jvm
            .invoke_static(
                "com.kosherjava.zmanim.util.NOAACalculator",
                "getJulianDay",
                &[InvocationArg::try_from(calendar).unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }
}
