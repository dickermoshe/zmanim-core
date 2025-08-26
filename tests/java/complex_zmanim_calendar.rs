use j4rs::{Instance, InvocationArg, Jvm, Null};
use zmanim_core::{
    prelude::{AstronomicalCalendarTrait, ComplexZmanimCalendarTrait, ZmanimCalendarTrait},
    utils::GeoLocationTrait,
};

use crate::java::{
    calendar::create_calendar, date::create_date, geolocation::JavaGeoLocation,
    noaa_calculator::JavaNOAACalculator,
};

#[allow(dead_code)]
pub struct JavaComplexZmanimCalendar<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> JavaComplexZmanimCalendar<'a> {
    #[allow(dead_code)]
    pub fn new(
        jvm: &'a Jvm,
        timestamp: i64,
        geo_location: &'a dyn GeoLocationTrait,
        noaa_calculator: JavaNOAACalculator,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: i64,
        ateret_torah_sunset_offset: i64,
    ) -> Self {
        let geolocation = JavaGeoLocation::new(jvm, geo_location);
        let calendar = create_calendar(jvm, timestamp);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.ComplexZmanimCalendar",
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
            "setAteretTorahSunsetOffset",
            &[
                InvocationArg::try_from(ateret_torah_sunset_offset as f64 / 1000.0 / 60.0)
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
impl<'a> ComplexZmanimCalendarTrait for JavaComplexZmanimCalendar<'a> {
    fn get_shaah_zmanis_19_point_8_degrees(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_18_degrees(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis18Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_26_degrees(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis26Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_16_point_1_degrees(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_60_minutes(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis60Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_72_minutes(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_72_minutes_zmanis(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_90_minutes(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis90Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_90_minutes_zmanis(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis90MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_96_minutes_zmanis(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis96MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_ateret_torah(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanisAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanisAlos16Point1ToTzais3Point8",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanisAlos16Point1ToTzais3Point7",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_96_minutes(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis96Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_120_minutes(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis120Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_120_minutes_zmanis(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanis120MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_baal_hatanya(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(
                    &self.instance,
                    "getShaahZmanisBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_alos_60(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos60", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_72_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos72Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_96(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos96", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_90_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos90Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_96_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos96Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_90(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos90", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_120(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos120", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_120_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos120Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_26_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos26Degrees", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_18_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos18Degrees", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_19_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlos19Degrees", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_alos_19_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getAlos19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_alos_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getAlos16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_alos_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getAlosBaalHatanya", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_misheyakir_11_point_5_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMisheyakir11Point5Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_misheyakir_11_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMisheyakir11Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_misheyakir_10_point_2_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMisheyakir10Point2Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_misheyakir_7_point_65_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMisheyakir7Point65Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_misheyakir_9_point_5_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMisheyakir9Point5Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_19_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_18_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA18Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_72_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_90_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA90Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_90_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA90MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_96_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA96Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_96_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA96MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_3_hours_before_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShma3HoursBeforeChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_120_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA120Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_alos_16_point_1_to_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaAlos16Point1ToSunset",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaAlos16Point1ToTzaisGeonim7Point083Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_kol_eliyahu(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaKolEliyahu",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA18DegreesToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA16Point1DegreesToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA90MinutesToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaMGA72MinutesToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaGRASunriseToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_19_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_18_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA18Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_72_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_90_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA90Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_90_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA90MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_96_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA96Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_96_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA96MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_mga_120_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaMGA120Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_2_hours_before_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfila2HoursBeforeChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaGRASunriseToFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_mincha_gedola_30_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedola30Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedola72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedola16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_ahavat_shalom(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaAhavatShalom",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_greater_than_30(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaGreaterThan30",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_baal_hatanya_greater_than_30(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaBaalHatanyaGreaterThan30",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedolaGRAFixedLocalChatzos30Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_mincha_ketana_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetana16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_ketana_ahavat_shalom(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetanaAhavatShalom",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_ketana_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetana72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_ketana_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetanaAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_ketana_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetanaBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetanaGRAFixedLocalChatzosToSunset",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_plag_hamincha_60_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha60Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_90_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha90Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_96_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha96Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_96_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha96MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_90_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha90MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_72_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_19_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_26_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha26Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_18_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha18Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_alos_to_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagAlosToSunset",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagAlos16Point1ToTzaisGeonim7Point083Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_ahavat_shalom(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagAhavatShalom",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHaminchaAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHaminchaBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_120_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha120MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_120_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha120Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHaminchaGRAFixedLocalChatzosToSunset",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hashmashos_rt_13_point_24_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosRT13Point24Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_rt_58_point_5_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosRT58Point5Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosRT13Point5MinutesBefore7Point083Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_rt_2_stars(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosRT2Stars",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_18_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim18Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_3_point_05_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim3Point05Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_16_point_875_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim16Point875Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_2_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim2Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_13_point_5_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim13Point5Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_bain_hashmashos_yereim_2_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHashmashosYereim2Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_3_point_7_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim3Point7Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_3_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim3Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_5_point_95_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim5Point95Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_3_point_65_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim3Point65Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_3_point_676_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim3Point676Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_4_point_61_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim4Point61Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_4_point_37_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim4Point37Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_5_point_88_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim5Point88Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_4_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim4Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_6_point_45_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim6Point45Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim7Point083Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_7_point_67_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim7Point67Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_8_point_5_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim8Point5Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_9_point_3_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim9Point3Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_geonim_9_point_75_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisGeonim9Point75Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_60(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais60", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_72_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais72Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_90_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais90Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_96_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais96Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_90(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais90", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_120(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais120", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_120_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais120Zmanis", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzais16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_26_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais26Degrees", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_18_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais18Degrees", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_19_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzais19Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_tzais_96(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais96", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_50(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getTzais50", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_tzais_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getTzaisBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_fixed_local_chatzos(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getFixedLocalChatzos",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_shma_fixed_local(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShmaFixedLocal",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_tfila_fixed_local(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilaFixedLocal",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_kidush_levana_between_moldos(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }
    fn get_sof_zman_kidush_levana_between_moldos_default(&self) -> Option<i64> {
        None
    }
    fn get_sof_zman_kidush_levana_15_days(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }
    fn get_sof_zman_kidush_levana_15_days_default(&self) -> Option<i64> {
        None
    }
    fn get_tchilas_zman_kidush_levana_3_days(&self) -> Option<i64> {
        None
    }
    fn get_tchilas_zman_kidush_levana_3_days_with_times(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }
    fn get_zman_molad(&self) -> Option<i64> {
        None
    }
    fn get_tchilas_zman_kidush_levana_7_days(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }
    fn get_tchilas_zman_kidush_levana_7_days_default(&self) -> Option<i64> {
        None
    }

    fn get_sof_zman_achilas_chametz_gra(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanAchilasChametzGRA",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_achilas_chametz_mga_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanAchilasChametzMGA72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanAchilasChametzMGA72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_achilas_chametz_mga_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanAchilasChametzMGA16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_biur_chametz_gra(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanBiurChametzGRA",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_biur_chametz_mga_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanBiurChametzMGA72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_biur_chametz_mga_72_minutes_zmanis(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanBiurChametzMGA72MinutesZmanis",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_biur_chametz_mga_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanBiurChametzMGA16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_achilas_chametz_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanAchilasChametzBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_sof_zman_biur_chametz_baal_hatanya(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanBiurChametzBaalHatanya",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_samuch_le_mincha_ketana_gra(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetanaGRA",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_samuch_le_mincha_ketana_16_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetana16Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }
    fn get_samuch_le_mincha_ketana_72_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetana72Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosrt_13_point_24_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosRT13Point24Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosrt_58_point_5_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosRT58Point5Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosRT13Point5MinutesBefore7Point083Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosrt_2_stars(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosRT2Stars",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_18_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim18Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_3_point_05_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim3Point05Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_16_point_875_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim16Point875Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_2_point_8_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim2Point8Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_13_point_5_minutes(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim13Point5Minutes",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_bain_hasmashosyereim_2_point_1_degrees(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBainHasmashosYereim2Point1Degrees",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_sof_zman_tfilah_ateret_torah(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfilahAteretTorah",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_fixed_local_chatzos_based_zmanim(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getFixedLocalChatzosBasedZmanim",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_of_half_day as i64))
                            .unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_of_half_day as i64))
                            .unwrap(),
                        InvocationArg::try_from(hours)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap(),
        )
    }
}

impl<'a> ZmanimCalendarTrait for JavaComplexZmanimCalendar<'a> {
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
        let double_value: f64 = self.jvm.to_rust(result).unwrap();
        if double_value.is_nan() {
            None
        } else {
            Some(double_value)
        }
    }

    fn get_half_day_based_zman(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getHalfDayBasedZman",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_half_day as i64))
                        .unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_half_day as i64))
                        .unwrap(),
                    InvocationArg::try_from(hours)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
    }

    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getHalfDayBasedShaahZmanis",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_half_day as i64))
                        .unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_half_day as i64))
                        .unwrap(),
                ],
            )
            .unwrap();
        java_long_to_i64(&self.jvm, result)
    }

    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: i64,
        end_of_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getShaahZmanisBasedZman",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                    InvocationArg::try_from(hours)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
    }

    fn _get_sof_zman_shma(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64> {
        if let Some(end_day) = end_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanShma",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64))
                            .unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_sof_zman_shma_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSofZmanShmaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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
        if let Some(end_day) = end_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getSofZmanTfila",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64))
                            .unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_sof_zman_tfila_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSofZmanTfilaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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
        if let Some(start_day) = start_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaGedola",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_day as i64)).unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_mincha_gedola_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getMinchaGedolaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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
        if let Some(start_day) = start_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getSamuchLeMinchaKetana",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_day as i64)).unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_samuch_le_mincha_ketana_simple(
        &self,
        start_of_day: i64,
        end_of_day: i64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSamuchLeMinchaKetanaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
    }

    fn _get_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        if let Some(start_day) = start_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getMinchaKetana",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_day as i64)).unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getMinchaKetanaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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
        if let Some(start_day) = start_of_day {
            let result = self
                .jvm
                .invoke(
                    &self.instance,
                    "getPlagHamincha",
                    &[
                        InvocationArg::try_from(create_date(&self.jvm, start_day as i64)).unwrap(),
                        InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                        InvocationArg::try_from(synchronous)
                            .unwrap()
                            .into_primitive()
                            .unwrap(),
                    ],
                )
                .unwrap();
            java_date_to_i64(&self.jvm, &result)
        } else {
            None
        }
    }

    fn get_plag_hamincha_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getPlagHaminchaSimple",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_of_day as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_of_day as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(&self.instance, "getShaahZmanisGra", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_shaah_zmanis_mga(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(&self.instance, "getShaahZmanisMGA", InvocationArg::empty())
                .unwrap(),
        )
    }
}

impl<'a> AstronomicalCalendarTrait for JavaComplexZmanimCalendar<'a> {
    fn get_utc_sunset(&self, zenith: f64) -> Option<f64> {
        let result = self
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

    fn get_sea_level_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSeaLevelSunset", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sunset(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSunset", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sunrise(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSunrise", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sea_level_sunrise(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSeaLevelSunrise", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sunrise_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSunriseOffsetByDegrees",
                    &[InvocationArg::try_from(degrees)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap(),
        )
    }

    fn get_sunset_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getSunsetOffsetByDegrees",
                    &[InvocationArg::try_from(degrees)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
                )
                .unwrap(),
        )
    }

    fn get_begin_civil_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBeginCivilTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_begin_nautical_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBeginNauticalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_begin_astronomical_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getBeginAstronomicalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_end_civil_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getEndCivilTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_end_nautical_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getEndNauticalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_end_astronomical_twilight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(
                    &self.instance,
                    "getEndAstronomicalTwilight",
                    InvocationArg::empty(),
                )
                .unwrap(),
        )
    }

    fn get_temporal_hour(&self) -> Option<i64> {
        java_long_to_i64(
            &self.jvm,
            self.jvm
                .invoke(&self.instance, "getTemporalHour", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_sun_transit(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSunTransit", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_solar_midnight(&self) -> Option<i64> {
        java_date_to_i64(
            &self.jvm,
            &self
                .jvm
                .invoke(&self.instance, "getSolarMidnight", InvocationArg::empty())
                .unwrap(),
        )
    }

    fn get_temporal_hour_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getTemporalHour",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_time as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_time as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_long_to_i64(&self.jvm, result)
    }

    fn get_sun_transit_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getSunTransit",
                &[
                    InvocationArg::try_from(create_date(&self.jvm, start_time as i64)).unwrap(),
                    InvocationArg::try_from(create_date(&self.jvm, end_time as i64)).unwrap(),
                ],
            )
            .unwrap();
        java_date_to_i64(&self.jvm, &result)
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

#[allow(dead_code)]
fn java_long_to_i64(jvm: &Jvm, value: Instance) -> Option<i64> {
    let long_value: i64 = jvm.to_rust(value).unwrap();
    if long_value == i64::MIN {
        None
    } else {
        Some(long_value)
    }
}
