use j4rs::{Instance, InvocationArg, Jvm};
use zmanim_core::prelude::*;

use crate::java::{calendar::create_calendar, geolocation::JavaGeoLocation};

#[allow(dead_code)]
pub struct JavaNOAACalculator<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}
#[allow(dead_code)]
impl<'a> JavaNOAACalculator<'a> {
    #[allow(dead_code)]
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
    pub fn get_utc_noon(&self, timestamp: i64, geo_location: &dyn GeoLocationTrait) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    pub fn get_utc_midnight(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
    ) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    pub fn get_utc_sunrise(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    pub fn get_utc_sunset(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    pub fn get_solar_elevation(
        &self,
        timestamp: i64,
        geo_location: &zmanim_core::utils::geolocation::GeoLocation,
    ) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    pub fn get_solar_azimuth(
        &self,
        timestamp: i64,
        geo_location: &zmanim_core::utils::geolocation::GeoLocation,
    ) -> Option<f64> {
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
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }
}
