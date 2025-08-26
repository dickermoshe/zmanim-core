use j4rs::{Instance, InvocationArg, Jvm, Null};
use zmanim_core::prelude::*;

use crate::java::{
    calendar::create_calendar, date::create_date, geolocation::JavaGeoLocation,
    noaa_calculator::JavaNOAACalculator,
};

#[allow(dead_code)]
pub struct JavaZmanimCalendar<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> JavaZmanimCalendar<'a> {
    #[allow(dead_code)]
    pub fn new(
        jvm: &'a Jvm,
        timestamp: i64,
        geo_location: &'a dyn GeoLocationTrait,
        noaa_calculator: JavaNOAACalculator,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: i64,
    ) -> Self {
        let geolocation = JavaGeoLocation::new(jvm, geo_location);
        let calendar = create_calendar(jvm, timestamp);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.ZmanimCalendar",
                &[InvocationArg::try_from(geolocation.instance).unwrap()],
            )
            .unwrap();
        jvm.invoke(
            &instance,
            "setCalendar",
            &[InvocationArg::try_from(calendar).unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setAstronomicalCalculator",
            &[InvocationArg::try_from(noaa_calculator.instance).unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setUseAstronomicalChatzos",
            &[InvocationArg::try_from(use_astronomical_chatzos)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setUseAstronomicalChatzosForOtherZmanim",
            &[
                InvocationArg::try_from(use_astronomical_chatzos_for_other_zmanim)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            ],
        )
        .unwrap();
        jvm.invoke(
            &instance,
            "setCandleLightingOffset",
            &[
                InvocationArg::try_from(candle_lighting_offset as f64 / 1000.0 / 60.0)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
            ],
        )
        .unwrap();

        jvm.invoke(
            &instance,
            "setUseElevation",
            &[InvocationArg::try_from(true)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();

        Self { jvm, instance }
    }
}

#[allow(dead_code)]
impl<'a> ZmanimCalendarTrait for JavaZmanimCalendar<'a> {
    fn get_tzais(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_hashachar(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlosHashachar", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos72(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos72", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getChatzos", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_chatzos_as_half_day(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getChatzosAsHalfDay",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getPercentOfShaahZmanisFromDegrees",
                &[
                    InvocationArg::try_from(degrees)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(sunset)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        let value: f64 = self.jvm.to_rust(result).unwrap();

        if value == 5e-324 {
            None
        } else {
            Some(value)
        }
    }

    fn get_half_day_based_zman(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_half_day as i64);
        let end_date = create_date(&self.jvm, end_of_half_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();
        let end_arg = InvocationArg::try_from(end_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getHalfDayBasedZman",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(hours)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
    ) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_half_day as i64);
        let end_date = create_date(&self.jvm, end_of_half_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();
        let end_arg = InvocationArg::try_from(end_date).unwrap();

        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getHalfDayBasedShaahZmanis",
                &[start_arg, end_arg],
            )
            .unwrap();

        let long_value: i64 = self.jvm.to_rust(result).unwrap();
        if long_value == i64::MIN {
            None
        } else {
            Some(long_value)
        }
    }

    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: i64,
        end_of_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_date = create_date(&self.jvm, end_of_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();
        let end_arg = InvocationArg::try_from(end_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanisBasedZman",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(hours)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn _get_sof_zman_shma(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_arg = end_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let start_arg = InvocationArg::try_from(start_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShma",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_date = create_date(&self.jvm, end_of_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();
        let end_arg = InvocationArg::try_from(end_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanShma", &[start_arg, end_arg])
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_gra(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanShmaGRA", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanShmaMGA", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais72(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais72", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_candle_lighting(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getCandleLighting", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn _get_sof_zman_tfila(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_arg = end_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let start_arg = InvocationArg::try_from(start_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfila",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_date = create_date(&self.jvm, end_of_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();
        let end_arg = InvocationArg::try_from(end_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanTfila", &[start_arg, end_arg])
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_gra(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanTfilaGRA", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSofZmanTfilaMGA", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn _get_mincha_gedola(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        let start_arg = start_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedola",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_mincha_gedola_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let start_date = create_date(&self.jvm, start_of_day as i64);
        let end_date = create_date(&self.jvm, end_of_day as i64);

        let start_arg = InvocationArg::try_from(start_date).unwrap();

        let end_arg = InvocationArg::try_from(end_date).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getMinchaGedola", &[start_arg, end_arg])
                .unwrap(),
        )
    }

    fn get_mincha_gedola_default(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getMinchaGedola", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn _get_samuch_le_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        let start_arg = start_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetana",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_samuch_le_mincha_ketana_simple(
        &self,
        start_of_day: i64,
        end_of_day: i64,
    ) -> Option<i64> {
        let start_arg =
            InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap();

        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetana",
                    &[start_arg, end_arg],
                )
                .unwrap(),
        )
    }

    fn _get_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        let start_arg = start_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetana",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let start_arg =
            InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap();
        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getMinchaKetana", &[start_arg, end_arg])
                .unwrap(),
        )
    }

    fn get_mincha_ketana_default(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getMinchaKetana", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn _get_plag_hamincha(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        let start_arg = start_of_day
            .map(|day| InvocationArg::try_from(create_date(&self.jvm, day as i64)).unwrap())
            .unwrap_or(InvocationArg::try_from(Null::Double).unwrap());

        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha",
                    &[
                        start_arg,
                        end_arg,
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }

    fn get_plag_hamincha_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let start_arg =
            InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap();
        let end_arg = InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap();

        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getPlagHamincha", &[start_arg, end_arg])
                .unwrap(),
        )
    }

    fn get_plag_hamincha_default(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getPlagHamincha", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_gra(&self) -> Option<i64> {
        let result = self
            .jvm
            .invoke(&self.instance, "getShaahZmanisGra", InvocationArg::empty())
            .unwrap();

        let long_value: i64 = self.jvm.to_rust(result).unwrap();
        if long_value == i64::MIN {
            None
        } else {
            Some(long_value)
        }
    }

    fn get_shaah_zmanis_mga(&self) -> Option<i64> {
        let result = self
            .jvm
            .invoke(&self.instance, "getShaahZmanisMGA", InvocationArg::empty())
            .unwrap();

        let long_value: i64 = self.jvm.to_rust(result).unwrap();
        if long_value == i64::MIN {
            None
        } else {
            Some(long_value)
        }
    }
}
#[allow(dead_code)]
fn java_date_to_i64(jvm: &Jvm, value: &Instance) -> Option<i64> {
    if jvm
        .check_equals(
            value,
            &InvocationArg::try_from(Null::Of("java.util.Date")).unwrap(),
        )
        .unwrap()
    {
        return None;
    }

    let result = jvm
        .invoke(&value, "getTime", InvocationArg::empty())
        .unwrap();
    Some(jvm.to_rust::<i64>(result).unwrap())
}
