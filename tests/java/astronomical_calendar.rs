use j4rs::{Instance, InvocationArg, Jvm, Null};
use zmanim_core::prelude::*;

use crate::java::{calendar::create_calendar, date::create_date, geolocation::JavaGeoLocation};

#[allow(dead_code)]
pub struct AstronomicalCalendar<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> AstronomicalCalendar<'a> {
    #[allow(dead_code)]
    pub fn new(jvm: &'a Jvm, timestamp: i64, geo_location: &dyn GeoLocationTrait) -> Self {
        let geolocation = JavaGeoLocation::new(jvm, geo_location);
        let calendar = create_calendar(jvm, timestamp);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.AstronomicalCalendar",
                &[InvocationArg::try_from(geolocation.instance).unwrap()],
            )
            .unwrap();
        jvm.invoke(
            &instance,
            "setCalendar",
            &[InvocationArg::try_from(calendar).unwrap()],
        )
        .unwrap();
        Self { jvm, instance }
    }
}

#[allow(dead_code)]
impl<'a> AstronomicalCalendarTrait for AstronomicalCalendar<'a> {
    fn get_utc_sunset(&self, zenith: f64) -> Option<f64> {
        let sunrise = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunset",
                &[InvocationArg::try_from(zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Some(self.jvm.to_rust::<f64>(sunrise).unwrap())
    }

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> Option<f64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSeaLevelSunrise",
                &[InvocationArg::try_from(zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> Option<f64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSeaLevelSunset",
                &[InvocationArg::try_from(zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    fn get_utc_sunrise(&self, zenith: f64) -> Option<f64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getUTCSunrise",
                &[InvocationArg::try_from(zenith)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        Some(self.jvm.to_rust::<f64>(result).unwrap())
    }

    fn get_sea_level_sunset(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSeaLevelSunset", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sun_transit(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSunTransit", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_solar_midnight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSolarMidnight", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sunset(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSunset", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sunrise(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSunrise", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sea_level_sunrise(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(&self.instance, "getSeaLevelSunrise", InvocationArg::empty())
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sunrise_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getSunriseOffsetByDegrees",
                &[InvocationArg::try_from(degrees)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sunset_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getSunsetOffsetByDegrees",
                &[InvocationArg::try_from(degrees)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_begin_civil_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getBeginCivilTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_begin_nautical_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getBeginNauticalTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_begin_astronomical_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getBeginAstronomicalTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_end_civil_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getEndCivilTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_end_nautical_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getEndNauticalTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_end_astronomical_twilight(&self) -> Option<i64> {
        let date = self
            .jvm
            .invoke(
                &self.instance,
                "getEndAstronomicalTwilight",
                InvocationArg::empty(),
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &date,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        let result = self
            .jvm
            .invoke(&date, "getTime", InvocationArg::empty())
            .unwrap();
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_temporal_hour(&self) -> Option<i64> {
        let result = self
            .jvm
            .invoke(&self.instance, "getTemporalHour", InvocationArg::empty())
            .unwrap();
        let temporal_hour = self.jvm.to_rust::<i64>(result).unwrap();
        if temporal_hour == i64::MIN {
            None
        } else {
            Some(temporal_hour)
        }
    }

    fn get_temporal_hour_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        let start_date = create_date(self.jvm, start_time as i64);
        let end_date = create_date(self.jvm, end_time as i64);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getTemporalHour",
                &[
                    InvocationArg::try_from(start_date).unwrap(),
                    InvocationArg::try_from(end_date).unwrap(),
                ],
            )
            .unwrap();
        if self
            .jvm
            .check_equals(&result, &InvocationArg::try_from(Null::Long).unwrap())
            .unwrap()
        {
            return None;
        }
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }

    fn get_sun_transit_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        let start_date = create_date(self.jvm, start_time as i64);
        let end_date = create_date(self.jvm, end_time as i64);
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSunTransit",
                &[
                    InvocationArg::try_from(start_date).unwrap(),
                    InvocationArg::try_from(end_date).unwrap(),
                ],
            )
            .unwrap();
        if self
            .jvm
            .check_equals(
                &result,
                &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
            )
            .unwrap()
        {
            return None;
        }
        Some(self.jvm.to_rust::<i64>(result).unwrap())
    }
}
