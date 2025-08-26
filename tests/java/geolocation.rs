use j4rs::{Instance, InvocationArg, Jvm};
use zmanim_core::{
    prelude::*,
    utils::geolocation::{Formula, GeoLocationError},
};

use crate::java::timezone::create_timezone;

pub struct JavaGeoLocation<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> JavaGeoLocation<'a> {
    pub fn new(jvm: &'a Jvm, geolocation: &dyn GeoLocationTrait) -> Self {
        let utc = create_timezone(jvm, "UTC");
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.util.GeoLocation",
                &[
                    InvocationArg::try_from("Name").unwrap(),
                    InvocationArg::try_from(geolocation.get_latitude())
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(geolocation.get_longitude())
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(geolocation.get_elevation())
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(utc).unwrap(),
                ],
            )
            .unwrap();
        Self { jvm, instance }
    }

    pub fn invoke_with_geolocation(
        &self,
        method_name: &str,
        other_geolocation: &GeoLocation,
    ) -> f64 {
        let java_other = JavaGeoLocation::new(&self.jvm, other_geolocation);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                method_name,
                &[InvocationArg::try_from(java_other.instance).unwrap()],
            )
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }
}

impl<'a> GeoLocationTrait for JavaGeoLocation<'a> {
    fn geodesic_initial_bearing(&self, location: &GeoLocation) -> Result<f64, GeoLocationError> {
        let result = self.invoke_with_geolocation("getGeodesicInitialBearing", location);
        if result.is_nan() {
            Err(GeoLocationError::FormulaError)
        } else {
            Ok(result)
        }
    }

    fn geodesic_final_bearing(&self, location: &GeoLocation) -> Result<f64, GeoLocationError> {
        let result = self.invoke_with_geolocation("getGeodesicFinalBearing", location);
        if result.is_nan() {
            Err(GeoLocationError::FormulaError)
        } else {
            Ok(result)
        }
    }

    fn geodesic_distance(&self, location: &GeoLocation) -> Result<f64, GeoLocationError> {
        let result = self.invoke_with_geolocation("getGeodesicDistance", location);

        if result.is_nan() {
            Err(GeoLocationError::FormulaError)
        } else {
            Ok(result)
        }
    }

    fn rhumb_line_bearing(&self, location: &GeoLocation) -> f64 {
        self.invoke_with_geolocation("getRhumbLineBearing", location)
    }

    fn rhumb_line_distance(&self, location: &GeoLocation) -> f64 {
        self.invoke_with_geolocation("getRhumbLineDistance", location)
    }

    fn vincenty_inverse_formula(
        &self,
        location: &GeoLocation,
        formula: Formula,
    ) -> Result<f64, GeoLocationError> {
        let java_other = JavaGeoLocation::new(&self.jvm, location);
        let formula_arg = match formula {
            Formula::Distance => 0,
            Formula::InitialBearing => 1,
            Formula::FinalBearing => 2,
        };
        let instance = self
            .jvm
            .invoke(
                &self.instance,
                "vincentyInverseFormula",
                &[
                    InvocationArg::try_from(java_other.instance).unwrap(),
                    InvocationArg::try_from(formula_arg)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        let result = self.jvm.to_rust::<f64>(instance).unwrap();
        if result.is_nan() {
            Err(GeoLocationError::FormulaError)
        } else {
            Ok(result)
        }
    }

    fn get_latitude(&self) -> f64 {
        let result = self
            .jvm
            .invoke(&self.instance, "getLatitude", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_longitude(&self) -> f64 {
        let result = self
            .jvm
            .invoke(&self.instance, "getLongitude", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }

    fn get_elevation(&self) -> f64 {
        let result = self
            .jvm
            .invoke(&self.instance, "getElevation", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust::<f64>(result).unwrap()
    }
}
