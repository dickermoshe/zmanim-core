#[cfg(feature = "uniffi")]
use crate::prelude::AstronomicalCalendar;
use crate::{
    astronomical_calendar::{AstronomicalCalendarTrait, GEOMETRIC_ZENITH, MINUTE_MILLIS},
    utils::GeoLocation,
    zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait},
};
#[cfg(feature = "uniffi")]
use std::sync::Arc;

const ZENITH_3_POINT_7: f64 = GEOMETRIC_ZENITH + 3.7;
const ZENITH_3_POINT_8: f64 = GEOMETRIC_ZENITH + 3.8;
const ZENITH_5_POINT_95: f64 = GEOMETRIC_ZENITH + 5.95;
const ZENITH_7_POINT_083: f64 = GEOMETRIC_ZENITH + 7.0 + (5.0 / 60.0);
const ZENITH_10_POINT_2: f64 = GEOMETRIC_ZENITH + 10.2;
const ZENITH_11_DEGREES: f64 = GEOMETRIC_ZENITH + 11.0;
const ZENITH_11_POINT_5: f64 = GEOMETRIC_ZENITH + 11.5;
const ZENITH_13_POINT_24: f64 = GEOMETRIC_ZENITH + 13.24;
const ZENITH_19_DEGREES: f64 = GEOMETRIC_ZENITH + 19.0;
const ZENITH_19_POINT_8: f64 = GEOMETRIC_ZENITH + 19.8;
const ZENITH_26_DEGREES: f64 = GEOMETRIC_ZENITH + 26.0;
const ZENITH_4_POINT_37: f64 = GEOMETRIC_ZENITH + 4.37;
const ZENITH_4_POINT_61: f64 = GEOMETRIC_ZENITH + 4.61;
const ZENITH_4_POINT_8: f64 = GEOMETRIC_ZENITH + 4.8;
const ZENITH_3_POINT_65: f64 = GEOMETRIC_ZENITH + 3.65;
const ZENITH_3_POINT_676: f64 = GEOMETRIC_ZENITH + 3.676;
const ZENITH_5_POINT_88: f64 = GEOMETRIC_ZENITH + 5.88;
const ZENITH_1_POINT_583: f64 = GEOMETRIC_ZENITH + 1.583;
const ZENITH_16_POINT_9: f64 = GEOMETRIC_ZENITH + 16.9;
const ZENITH_6_DEGREES: f64 = GEOMETRIC_ZENITH + 6.0;
const ZENITH_6_POINT_45: f64 = GEOMETRIC_ZENITH + 6.45;
const ZENITH_7_POINT_65: f64 = GEOMETRIC_ZENITH + 7.65;
const ZENITH_7_POINT_67: f64 = GEOMETRIC_ZENITH + 7.67;
const ZENITH_9_POINT_3: f64 = GEOMETRIC_ZENITH + 9.3;
const ZENITH_9_POINT_5: f64 = GEOMETRIC_ZENITH + 9.5;
const ZENITH_9_POINT_75: f64 = GEOMETRIC_ZENITH + 9.75;
const ZENITH_MINUS_2_POINT_1: f64 = GEOMETRIC_ZENITH - 2.1;
const ZENITH_MINUS_2_POINT_8: f64 = GEOMETRIC_ZENITH - 2.8;
const ZENITH_MINUS_3_POINT_05: f64 = GEOMETRIC_ZENITH - 3.05;

const ZENITH_8_POINT_5: f64 = GEOMETRIC_ZENITH + 8.5;
const ZENITH_16_POINT_1: f64 = GEOMETRIC_ZENITH + 16.1;
const ASTRONOMICAL_ZENITH: f64 = 108.0;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct ComplexZmanimCalendar {
    pub zmanim_calendar: ZmanimCalendar,
    pub ateret_torah_sunset_offset: i64,
}
impl ComplexZmanimCalendar {
    pub fn new(
        timestamp: i64,
        geo_location: GeoLocation,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: i64,
        ateret_torah_sunset_offset: i64,
    ) -> Self {
        Self {
            zmanim_calendar: ZmanimCalendar::new(
                timestamp,
                geo_location,
                use_astronomical_chatzos,
                use_astronomical_chatzos_for_other_zmanim,
                candle_lighting_offset,
            ),
            ateret_torah_sunset_offset,
        }
    }

    fn get_zmanis_based_offset(&self, hours: f64) -> Option<i64> {
        let shaah_zmanis = self.zmanim_calendar.get_shaah_zmanis_gra()? as f64;
        if hours == 0.0 {
            return None;
        }

        if hours > 0.0 {
            let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
            Some(sunset + (shaah_zmanis * hours) as i64)
        } else {
            let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
            Some(sunrise + (shaah_zmanis * hours) as i64)
        }
    }

    fn get_sunrise_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_1_POINT_583)
    }

    fn get_sunset_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_1_POINT_583)
    }
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
pub fn new_complex_zmanim_calendar(
    timestamp: i64,
    geo_location: Arc<GeoLocation>,
    use_astronomical_chatzos: bool,
    use_astronomical_chatzos_for_other_zmanim: bool,
    candle_lighting_offset: i64,
    ateret_torah_sunset_offset: i64,
) -> Arc<ComplexZmanimCalendar> {
    Arc::new(ComplexZmanimCalendar::new(
        timestamp,
        (*geo_location).clone(),
        use_astronomical_chatzos,
        use_astronomical_chatzos_for_other_zmanim,
        candle_lighting_offset,
        ateret_torah_sunset_offset,
    ))
}
pub trait ComplexZmanimCalendarTrait {
    fn get_shaah_zmanis_19_point_8_degrees(&self) -> Option<i64>;
    fn get_shaah_zmanis_18_degrees(&self) -> Option<i64>;
    fn get_shaah_zmanis_26_degrees(&self) -> Option<i64>;
    fn get_shaah_zmanis_16_point_1_degrees(&self) -> Option<i64>;
    fn get_shaah_zmanis_60_minutes(&self) -> Option<i64>;
    fn get_shaah_zmanis_72_minutes(&self) -> Option<i64>;
    fn get_shaah_zmanis_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_shaah_zmanis_90_minutes(&self) -> Option<i64>;
    fn get_shaah_zmanis_90_minutes_zmanis(&self) -> Option<i64>;
    fn get_shaah_zmanis_96_minutes_zmanis(&self) -> Option<i64>;
    fn get_shaah_zmanis_ateret_torah(&self) -> Option<i64>;
    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(&self) -> Option<i64>;
    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(&self) -> Option<i64>;
    fn get_shaah_zmanis_96_minutes(&self) -> Option<i64>;
    fn get_shaah_zmanis_120_minutes(&self) -> Option<i64>;
    fn get_shaah_zmanis_120_minutes_zmanis(&self) -> Option<i64>;
    fn get_shaah_zmanis_baal_hatanya(&self) -> Option<i64>;

    fn get_alos_60(&self) -> Option<i64>;
    fn get_alos_72_zmanis(&self) -> Option<i64>;
    fn get_alos_96(&self) -> Option<i64>;
    fn get_alos_90_zmanis(&self) -> Option<i64>;
    fn get_alos_96_zmanis(&self) -> Option<i64>;
    fn get_alos_90(&self) -> Option<i64>;
    fn get_alos_120(&self) -> Option<i64>;
    fn get_alos_120_zmanis(&self) -> Option<i64>;
    fn get_alos_26_degrees(&self) -> Option<i64>;
    fn get_alos_18_degrees(&self) -> Option<i64>;
    fn get_alos_19_degrees(&self) -> Option<i64>;
    fn get_alos_19_point_8_degrees(&self) -> Option<i64>;
    fn get_alos_16_point_1_degrees(&self) -> Option<i64>;
    fn get_alos_baal_hatanya(&self) -> Option<i64>;

    fn get_misheyakir_11_point_5_degrees(&self) -> Option<i64>;
    fn get_misheyakir_11_degrees(&self) -> Option<i64>;
    fn get_misheyakir_10_point_2_degrees(&self) -> Option<i64>;
    fn get_misheyakir_7_point_65_degrees(&self) -> Option<i64>;
    fn get_misheyakir_9_point_5_degrees(&self) -> Option<i64>;

    fn get_sof_zman_shma_mga_19_point_8_degrees(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_16_point_1_degrees(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_18_degrees(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_72_minutes(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_90_minutes(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_90_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_96_minutes(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_96_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_shma_3_hours_before_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_120_minutes(&self) -> Option<i64>;
    fn get_sof_zman_shma_alos_16_point_1_to_sunset(&self) -> Option<i64>;
    fn get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;
    fn get_sof_zman_shma_kol_eliyahu(&self) -> Option<i64>;
    fn get_sof_zman_shma_ateret_torah(&self) -> Option<i64>;
    fn get_sof_zman_shma_baal_hatanya(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64>;

    fn get_sof_zman_tfila_mga_19_point_8_degrees(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_16_point_1_degrees(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_18_degrees(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_72_minutes(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_90_minutes(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_90_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_96_minutes(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_96_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga_120_minutes(&self) -> Option<i64>;
    fn get_sof_zman_tfila_2_hours_before_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_tfila_ateret_torah(&self) -> Option<i64>;
    fn get_sof_zman_tfila_baal_hatanya(&self) -> Option<i64>;
    fn get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64>;

    fn get_mincha_gedola_30_minutes(&self) -> Option<i64>;
    fn get_mincha_gedola_72_minutes(&self) -> Option<i64>;
    fn get_mincha_gedola_16_point_1_degrees(&self) -> Option<i64>;
    fn get_mincha_gedola_ahavat_shalom(&self) -> Option<i64>;
    fn get_mincha_gedola_greater_than_30(&self) -> Option<i64>;
    fn get_mincha_gedola_ateret_torah(&self) -> Option<i64>;
    fn get_mincha_gedola_baal_hatanya(&self) -> Option<i64>;
    fn get_mincha_gedola_baal_hatanya_greater_than_30(&self) -> Option<i64>;
    fn get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(&self) -> Option<i64>;

    fn get_mincha_ketana_16_point_1_degrees(&self) -> Option<i64>;
    fn get_mincha_ketana_ahavat_shalom(&self) -> Option<i64>;
    fn get_mincha_ketana_72_minutes(&self) -> Option<i64>;
    fn get_mincha_ketana_ateret_torah(&self) -> Option<i64>;
    fn get_mincha_ketana_baal_hatanya(&self) -> Option<i64>;
    fn get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64>;

    fn get_plag_hamincha_60_minutes(&self) -> Option<i64>;
    fn get_plag_hamincha_72_minutes(&self) -> Option<i64>;
    fn get_plag_hamincha_90_minutes(&self) -> Option<i64>;
    fn get_plag_hamincha_96_minutes(&self) -> Option<i64>;
    fn get_plag_hamincha_96_minutes_zmanis(&self) -> Option<i64>;
    fn get_plag_hamincha_90_minutes_zmanis(&self) -> Option<i64>;
    fn get_plag_hamincha_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_plag_hamincha_16_point_1_degrees(&self) -> Option<i64>;
    fn get_plag_hamincha_19_point_8_degrees(&self) -> Option<i64>;
    fn get_plag_hamincha_26_degrees(&self) -> Option<i64>;
    fn get_plag_hamincha_18_degrees(&self) -> Option<i64>;
    fn get_plag_alos_to_sunset(&self) -> Option<i64>;
    fn get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;
    fn get_plag_ahavat_shalom(&self) -> Option<i64>;
    fn get_plag_hamincha_ateret_torah(&self) -> Option<i64>;
    fn get_plag_hamincha_baal_hatanya(&self) -> Option<i64>;
    fn get_plag_hamincha_120_minutes_zmanis(&self) -> Option<i64>;
    fn get_plag_hamincha_120_minutes(&self) -> Option<i64>;
    fn get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64>;

    fn get_bain_hashmashos_rt_13_point_24_degrees(&self) -> Option<i64>;
    fn get_bain_hashmashos_rt_58_point_5_minutes(&self) -> Option<i64>;
    fn get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64>;
    fn get_bain_hashmashos_rt_2_stars(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_18_minutes(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_3_point_05_degrees(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_16_point_875_minutes(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_2_point_8_degrees(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_13_point_5_minutes(&self) -> Option<i64>;
    fn get_bain_hashmashos_yereim_2_point_1_degrees(&self) -> Option<i64>;

    fn get_tzais_geonim_3_point_7_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_3_point_8_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_5_point_95_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_3_point_65_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_3_point_676_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_4_point_61_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_4_point_37_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_5_point_88_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_4_point_8_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_6_point_45_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_7_point_083_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_7_point_67_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_8_point_5_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_9_point_3_degrees(&self) -> Option<i64>;
    fn get_tzais_geonim_9_point_75_degrees(&self) -> Option<i64>;
    fn get_tzais_60(&self) -> Option<i64>;
    fn get_tzais_ateret_torah(&self) -> Option<i64>;
    fn get_tzais_72_zmanis(&self) -> Option<i64>;
    fn get_tzais_90_zmanis(&self) -> Option<i64>;
    fn get_tzais_96_zmanis(&self) -> Option<i64>;
    fn get_tzais_90(&self) -> Option<i64>;
    fn get_tzais_120(&self) -> Option<i64>;
    fn get_tzais_120_zmanis(&self) -> Option<i64>;
    fn get_tzais_16_point_1_degrees(&self) -> Option<i64>;
    fn get_tzais_26_degrees(&self) -> Option<i64>;
    fn get_tzais_18_degrees(&self) -> Option<i64>;
    fn get_tzais_19_point_8_degrees(&self) -> Option<i64>;
    fn get_tzais_96(&self) -> Option<i64>;
    fn get_tzais_50(&self) -> Option<i64>;
    fn get_tzais_baal_hatanya(&self) -> Option<i64>;

    fn get_fixed_local_chatzos(&self) -> Option<i64>;
    fn get_sof_zman_shma_fixed_local(&self) -> Option<i64>;
    fn get_sof_zman_tfila_fixed_local(&self) -> Option<i64>;

    fn get_sof_zman_kidush_levana_between_moldos(
        &self,
        alos: Option<i64>,
        tzais: Option<i64>,
    ) -> Option<i64>;
    fn get_sof_zman_kidush_levana_between_moldos_default(&self) -> Option<i64>;
    fn get_sof_zman_kidush_levana_15_days(
        &self,
        alos: Option<i64>,
        tzais: Option<i64>,
    ) -> Option<i64>;
    fn get_sof_zman_kidush_levana_15_days_default(&self) -> Option<i64>;
    fn get_tchilas_zman_kidush_levana_3_days(&self) -> Option<i64>;
    fn get_tchilas_zman_kidush_levana_3_days_with_times(
        &self,
        alos: Option<i64>,
        tzais: Option<i64>,
    ) -> Option<i64>;
    fn get_zman_molad(&self) -> Option<i64>;
    fn get_tchilas_zman_kidush_levana_7_days(
        &self,
        alos: Option<i64>,
        tzais: Option<i64>,
    ) -> Option<i64>;
    fn get_tchilas_zman_kidush_levana_7_days_default(&self) -> Option<i64>;

    fn get_sof_zman_achilas_chametz_gra(&self) -> Option<i64>;
    fn get_sof_zman_achilas_chametz_mga_72_minutes(&self) -> Option<i64>;
    fn get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_achilas_chametz_mga_16_point_1_degrees(&self) -> Option<i64>;
    fn get_sof_zman_biur_chametz_gra(&self) -> Option<i64>;
    fn get_sof_zman_biur_chametz_mga_72_minutes(&self) -> Option<i64>;
    fn get_sof_zman_biur_chametz_mga_72_minutes_zmanis(&self) -> Option<i64>;
    fn get_sof_zman_biur_chametz_mga_16_point_1_degrees(&self) -> Option<i64>;
    fn get_sof_zman_achilas_chametz_baal_hatanya(&self) -> Option<i64>;
    fn get_sof_zman_biur_chametz_baal_hatanya(&self) -> Option<i64>;

    fn get_samuch_le_mincha_ketana_gra(&self) -> Option<i64>;
    fn get_samuch_le_mincha_ketana_16_point_1_degrees(&self) -> Option<i64>;
    fn get_samuch_le_mincha_ketana_72_minutes(&self) -> Option<i64>;

    fn get_bain_hasmashosrt_13_point_24_degrees(&self) -> Option<i64>;
    fn get_bain_hasmashosrt_58_point_5_minutes(&self) -> Option<i64>;
    fn get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64>;
    fn get_bain_hasmashosrt_2_stars(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_18_minutes(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_3_point_05_degrees(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_16_point_875_minutes(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_2_point_8_degrees(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_13_point_5_minutes(&self) -> Option<i64>;
    fn get_bain_hasmashosyereim_2_point_1_degrees(&self) -> Option<i64>;
    fn get_sof_zman_tfilah_ateret_torah(&self) -> Option<i64>;
    fn get_fixed_local_chatzos_based_zmanim(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64>;
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl ComplexZmanimCalendarTrait for ComplexZmanimCalendar {
    fn get_shaah_zmanis_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_19_point_8_degrees()?,
                self.get_tzais_19_point_8_degrees()?,
            )
    }

    fn get_shaah_zmanis_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_18_degrees()?,
                self.get_tzais_18_degrees()?,
            )
    }

    fn get_shaah_zmanis_26_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_26_degrees()?,
                self.get_tzais_26_degrees()?,
            )
    }

    fn get_shaah_zmanis_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_16_point_1_degrees()?,
                self.get_tzais_16_point_1_degrees()?,
            )
    }

    fn get_shaah_zmanis_60_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(self.get_alos_60()?, self.get_tzais_60()?)
    }

    fn get_shaah_zmanis_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar.get_shaah_zmanis_mga()
    }

    fn get_shaah_zmanis_72_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_72_zmanis()?,
                self.get_tzais_72_zmanis()?,
            )
    }

    fn get_shaah_zmanis_90_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(self.get_alos_90()?, self.get_tzais_90()?)
    }

    fn get_shaah_zmanis_90_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_90_zmanis()?,
                self.get_tzais_90_zmanis()?,
            )
    }

    fn get_shaah_zmanis_96_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_96_zmanis()?,
                self.get_tzais_96_zmanis()?,
            )
    }

    fn get_shaah_zmanis_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_72_zmanis()?,
                self.get_tzais_ateret_torah()?,
            )
    }

    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_16_point_1_degrees()?,
                self.get_tzais_geonim_3_point_8_degrees()?,
            )
    }

    fn get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_16_point_1_degrees()?,
                self.get_tzais_geonim_3_point_7_degrees()?,
            )
    }

    fn get_shaah_zmanis_96_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(self.get_alos_96()?, self.get_tzais_96()?)
    }

    fn get_shaah_zmanis_120_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(self.get_alos_120()?, self.get_tzais_120()?)
    }

    fn get_shaah_zmanis_120_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_alos_120_zmanis()?,
                self.get_tzais_120_zmanis()?,
            )
    }

    fn get_shaah_zmanis_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.get_sunrise_baal_hatanya()?,
                self.get_sunset_baal_hatanya()?,
            )
    }

    fn get_alos_60(&self) -> Option<i64> {
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        Some(sunrise - 60 * MINUTE_MILLIS)
    }

    fn get_alos_72_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(-1.2)
    }

    fn get_alos_96(&self) -> Option<i64> {
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        Some(sunrise - 96 * MINUTE_MILLIS)
    }

    fn get_alos_90_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(-1.5)
    }

    fn get_alos_96_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(-1.6)
    }

    fn get_alos_90(&self) -> Option<i64> {
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        Some(sunrise - 90 * MINUTE_MILLIS)
    }

    fn get_alos_120(&self) -> Option<i64> {
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        Some(sunrise - 120 * MINUTE_MILLIS)
    }

    fn get_alos_120_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(-2.0)
    }

    fn get_alos_26_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_26_DEGREES)
    }

    fn get_alos_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ASTRONOMICAL_ZENITH)
    }

    fn get_alos_19_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_19_DEGREES)
    }

    fn get_alos_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_19_POINT_8)
    }

    fn get_alos_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_16_POINT_1)
    }

    fn get_alos_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_16_POINT_9)
    }

    fn get_misheyakir_11_point_5_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_11_POINT_5)
    }

    fn get_misheyakir_11_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_11_DEGREES)
    }

    fn get_misheyakir_10_point_2_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_10_POINT_2)
    }

    fn get_misheyakir_7_point_65_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_7_POINT_65)
    }

    fn get_misheyakir_9_point_5_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_9_POINT_5)
    }

    fn get_sof_zman_shma_mga_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_19_point_8_degrees()?,
            self.get_tzais_19_point_8_degrees(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_16_point_1_degrees()?,
            self.get_tzais_16_point_1_degrees(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_18_degrees()?,
            self.get_tzais_18_degrees(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar.get_sof_zman_shma_mga()
    }

    fn get_sof_zman_shma_mga_72_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_72_zmanis()?,
            self.get_tzais_72_zmanis(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_90_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_shma(self.get_alos_90()?, self.get_tzais_90(), true)
    }

    fn get_sof_zman_shma_mga_90_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_90_zmanis()?,
            self.get_tzais_90_zmanis(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_96_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_shma(self.get_alos_96()?, self.get_tzais_96(), true)
    }

    fn get_sof_zman_shma_mga_96_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_96_zmanis()?,
            self.get_tzais_96_zmanis(),
            true,
        )
    }

    fn get_sof_zman_shma_3_hours_before_chatzos(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        Some(chatzos - 180 * MINUTE_MILLIS)
    }

    fn get_sof_zman_shma_mga_120_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_shma(self.get_alos_120()?, self.get_tzais_120(), true)
    }

    fn get_sof_zman_shma_alos_16_point_1_to_sunset(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_16_point_1_degrees()?,
            self.zmanim_calendar.astronomical_calendar.get_sunset(),
            false,
        )
    }

    fn get_sof_zman_shma_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_16_point_1_degrees()?,
            self.get_tzais_geonim_7_point_083_degrees(),
            false,
        )
    }

    fn get_sof_zman_shma_kol_eliyahu(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        let diff = (chatzos - sunrise) / 2;
        Some(chatzos - diff)
    }

    fn get_sof_zman_shma_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_alos_72_zmanis()?,
            self.get_tzais_ateret_torah(),
            false,
        )
    }

    fn get_sof_zman_shma_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_shma(
            self.get_sunrise_baal_hatanya()?,
            self.get_sunset_baal_hatanya(),
            true,
        )
    }

    fn get_sof_zman_shma_mga_18_degrees_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.get_alos_18_degrees()?,
            self.zmanim_calendar.get_chatzos()?,
            3.0,
        )
    }

    fn get_sof_zman_shma_mga_16_point_1_degrees_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.get_alos_16_point_1_degrees()?,
            self.zmanim_calendar.get_chatzos()?,
            3.0,
        )
    }

    fn get_sof_zman_shma_mga_90_minutes_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.get_alos_90()?,
            self.zmanim_calendar.get_chatzos()?,
            3.0,
        )
    }

    fn get_sof_zman_shma_mga_72_minutes_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.zmanim_calendar.get_alos72()?,
            self.zmanim_calendar.get_chatzos()?,
            3.0,
        )
    }

    fn get_sof_zman_shma_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.zmanim_calendar.astronomical_calendar.get_sunrise()?,
            self.zmanim_calendar.get_chatzos()?,
            3.0,
        )
    }

    fn get_sof_zman_tfila_mga_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_19_point_8_degrees()?,
            self.get_tzais_19_point_8_degrees(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_16_point_1_degrees()?,
            self.get_tzais_16_point_1_degrees(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_18_degrees()?,
            self.get_tzais_18_degrees(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar.get_sof_zman_tfila_mga()
    }

    fn get_sof_zman_tfila_mga_72_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_72_zmanis()?,
            self.get_tzais_72_zmanis(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_90_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_tfila(self.get_alos_90()?, self.get_tzais_90(), true)
    }

    fn get_sof_zman_tfila_mga_90_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_90_zmanis()?,
            self.get_tzais_90_zmanis(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_96_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_tfila(self.get_alos_96()?, self.get_tzais_96(), true)
    }

    fn get_sof_zman_tfila_mga_96_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_96_zmanis()?,
            self.get_tzais_96_zmanis(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga_120_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_sof_zman_tfila(self.get_alos_120()?, self.get_tzais_120(), true)
    }

    fn get_sof_zman_tfila_2_hours_before_chatzos(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        Some(chatzos - 120 * MINUTE_MILLIS)
    }

    fn get_sof_zman_tfila_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_alos_72_zmanis()?,
            self.get_tzais_ateret_torah(),
            false,
        )
    }

    fn get_sof_zman_tfila_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar._get_sof_zman_tfila(
            self.get_sunrise_baal_hatanya()?,
            self.get_sunset_baal_hatanya(),
            true,
        )
    }

    fn get_sof_zman_tfila_gra_sunrise_to_fixed_local_chatzos(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.zmanim_calendar.astronomical_calendar.get_sunrise()?,
            self.zmanim_calendar.get_chatzos()?,
            4.0,
        )
    }

    fn get_mincha_gedola_30_minutes(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        Some(chatzos + 30 * MINUTE_MILLIS)
    }

    fn get_mincha_gedola_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_gedola(
            self.zmanim_calendar.get_alos72(),
            self.zmanim_calendar.get_tzais72()?,
            true,
        )
    }

    fn get_mincha_gedola_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_gedola(
            self.get_alos_16_point_1_degrees(),
            self.get_tzais_16_point_1_degrees()?,
            true,
        )
    }

    fn get_mincha_gedola_ahavat_shalom(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        let shaah_zmanis = self.get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_7()? as f64;
        let half_shaah = chatzos + (shaah_zmanis / 2.0) as i64;

        if self.get_mincha_gedola_30_minutes()? > half_shaah {
            self.get_mincha_gedola_30_minutes()
        } else {
            Some(half_shaah)
        }
    }

    fn get_mincha_gedola_greater_than_30(&self) -> Option<i64> {
        let mincha_gedola_30 = self.get_mincha_gedola_30_minutes()?;
        let mincha_gedola = self.zmanim_calendar.get_mincha_gedola_default()?;

        if mincha_gedola_30 > mincha_gedola {
            Some(mincha_gedola_30)
        } else {
            Some(mincha_gedola)
        }
    }

    fn get_mincha_gedola_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_gedola(
            self.get_alos_72_zmanis(),
            self.get_tzais_ateret_torah()?,
            false,
        )
    }

    fn get_mincha_gedola_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_gedola(
            self.get_sunrise_baal_hatanya(),
            self.get_sunset_baal_hatanya()?,
            true,
        )
    }

    fn get_mincha_gedola_baal_hatanya_greater_than_30(&self) -> Option<i64> {
        let mincha_gedola_30 = self.get_mincha_gedola_30_minutes()?;
        let mincha_gedola_baal_hatanya = self.get_mincha_gedola_baal_hatanya()?;

        if mincha_gedola_30 > mincha_gedola_baal_hatanya {
            Some(mincha_gedola_30)
        } else {
            Some(mincha_gedola_baal_hatanya)
        }
    }

    fn get_mincha_gedola_gra_fixed_local_chatzos_30_minutes(&self) -> Option<i64> {
        let fixed_local_chatzos = self.get_fixed_local_chatzos()?;
        Some(fixed_local_chatzos + 30 * MINUTE_MILLIS)
    }

    fn get_mincha_ketana_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_ketana(
            self.get_alos_16_point_1_degrees(),
            self.get_tzais_16_point_1_degrees()?,
            true,
        )
    }

    fn get_mincha_ketana_ahavat_shalom(&self) -> Option<i64> {
        let tzais = self.get_tzais_geonim_3_point_8_degrees()?;
        let shaah_zmanis = self.get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8()? as f64;
        Some(tzais - (shaah_zmanis * 2.5) as i64)
    }

    fn get_mincha_ketana_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_ketana(
            self.zmanim_calendar.get_alos72(),
            self.zmanim_calendar.get_tzais72()?,
            true,
        )
    }

    fn get_mincha_ketana_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_ketana(
            self.get_alos_72_zmanis(),
            self.get_tzais_ateret_torah()?,
            false,
        )
    }

    fn get_mincha_ketana_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar._get_mincha_ketana(
            self.get_sunrise_baal_hatanya(),
            self.get_sunset_baal_hatanya()?,
            true,
        )
    }

    fn get_mincha_ketana_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.zmanim_calendar.get_chatzos()?,
            self.zmanim_calendar.astronomical_calendar.get_sunset()?,
            3.5,
        )
    }

    fn get_plag_hamincha_60_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_plag_hamincha(self.get_alos_60(), self.get_tzais_60()?, true)
    }

    fn get_plag_hamincha_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.zmanim_calendar.get_alos72(),
            self.zmanim_calendar.get_tzais72()?,
            true,
        )
    }

    fn get_plag_hamincha_90_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_plag_hamincha(self.get_alos_90(), self.get_tzais_90()?, true)
    }

    fn get_plag_hamincha_96_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_plag_hamincha(self.get_alos_96(), self.get_tzais_96()?, true)
    }

    fn get_plag_hamincha_96_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_96_zmanis(),
            self.get_tzais_96_zmanis()?,
            true,
        )
    }

    fn get_plag_hamincha_90_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_90_zmanis(),
            self.get_tzais_90_zmanis()?,
            true,
        )
    }

    fn get_plag_hamincha_72_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_72_zmanis(),
            self.get_tzais_72_zmanis()?,
            true,
        )
    }

    fn get_plag_hamincha_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_16_point_1_degrees(),
            self.get_tzais_16_point_1_degrees()?,
            true,
        )
    }

    fn get_plag_hamincha_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_19_point_8_degrees(),
            self.get_tzais_19_point_8_degrees()?,
            true,
        )
    }

    fn get_plag_hamincha_26_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_26_degrees(),
            self.get_tzais_26_degrees()?,
            true,
        )
    }

    fn get_plag_hamincha_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_18_degrees(),
            self.get_tzais_18_degrees()?,
            true,
        )
    }

    fn get_plag_alos_to_sunset(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_16_point_1_degrees(),
            self.zmanim_calendar.astronomical_calendar.get_sunset()?,
            false,
        )
    }

    fn get_plag_alos_16_point_1_to_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_16_point_1_degrees(),
            self.get_tzais_geonim_7_point_083_degrees()?,
            false,
        )
    }

    fn get_plag_ahavat_shalom(&self) -> Option<i64> {
        let tzais = self.get_tzais_geonim_3_point_8_degrees()?;
        let shaah_zmanis = self.get_shaah_zmanis_alos_16_point_1_to_tzais_3_point_8()? as f64;
        Some(tzais - (shaah_zmanis * 1.25) as i64)
    }

    fn get_plag_hamincha_ateret_torah(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_72_zmanis(),
            self.get_tzais_ateret_torah()?,
            false,
        )
    }

    fn get_plag_hamincha_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_sunrise_baal_hatanya(),
            self.get_sunset_baal_hatanya()?,
            true,
        )
    }

    fn get_plag_hamincha_120_minutes_zmanis(&self) -> Option<i64> {
        self.zmanim_calendar._get_plag_hamincha(
            self.get_alos_120_zmanis(),
            self.get_tzais_120_zmanis()?,
            true,
        )
    }

    fn get_plag_hamincha_120_minutes(&self) -> Option<i64> {
        self.zmanim_calendar
            ._get_plag_hamincha(self.get_alos_120(), self.get_tzais_120()?, true)
    }

    fn get_plag_hamincha_gra_fixed_local_chatzos_to_sunset(&self) -> Option<i64> {
        self.zmanim_calendar.get_half_day_based_zman(
            self.zmanim_calendar.get_chatzos()?,
            self.zmanim_calendar.astronomical_calendar.get_sunset()?,
            4.75,
        )
    }

    fn get_bain_hashmashos_rt_13_point_24_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_13_POINT_24)
    }

    fn get_bain_hashmashos_rt_58_point_5_minutes(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + (58.5 * MINUTE_MILLIS as f64) as i64)
    }

    fn get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64> {
        let sunset_offset = self
            .zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_7_POINT_083)?;
        Some(sunset_offset - (13.5 * MINUTE_MILLIS as f64) as i64)
    }

    fn get_bain_hashmashos_rt_2_stars(&self) -> Option<i64> {
        let alos_19_8 = self.get_alos_19_point_8_degrees()?;
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + ((sunrise - alos_19_8) as f64 * (5.0 / 18.0)) as i64)
    }

    fn get_bain_hashmashos_yereim_18_minutes(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset - 18 * MINUTE_MILLIS)
    }

    fn get_bain_hashmashos_yereim_3_point_05_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_MINUS_3_POINT_05)
    }

    fn get_bain_hashmashos_yereim_16_point_875_minutes(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset - (16.875 * MINUTE_MILLIS as f64) as i64)
    }

    fn get_bain_hashmashos_yereim_2_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_MINUS_2_POINT_8)
    }

    fn get_bain_hashmashos_yereim_13_point_5_minutes(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset - (13.5 * MINUTE_MILLIS as f64) as i64)
    }

    fn get_bain_hashmashos_yereim_2_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_MINUS_2_POINT_1)
    }

    fn get_tzais_geonim_3_point_7_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_3_POINT_7)
    }

    fn get_tzais_geonim_3_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_3_POINT_8)
    }

    fn get_tzais_geonim_5_point_95_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_5_POINT_95)
    }

    fn get_tzais_geonim_3_point_65_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_3_POINT_65)
    }

    fn get_tzais_geonim_3_point_676_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_3_POINT_676)
    }

    fn get_tzais_geonim_4_point_61_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_4_POINT_61)
    }

    fn get_tzais_geonim_4_point_37_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_4_POINT_37)
    }

    fn get_tzais_geonim_5_point_88_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_5_POINT_88)
    }

    fn get_tzais_geonim_4_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_4_POINT_8)
    }

    fn get_tzais_geonim_6_point_45_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_6_POINT_45)
    }

    fn get_tzais_geonim_7_point_083_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_7_POINT_083)
    }

    fn get_tzais_geonim_7_point_67_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_7_POINT_67)
    }

    fn get_tzais_geonim_8_point_5_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_8_POINT_5)
    }

    fn get_tzais_geonim_9_point_3_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_9_POINT_3)
    }

    fn get_tzais_geonim_9_point_75_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_9_POINT_75)
    }

    fn get_tzais_60(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + 60 * MINUTE_MILLIS)
    }

    fn get_tzais_ateret_torah(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + self.ateret_torah_sunset_offset)
    }

    fn get_tzais_72_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(1.2)
    }

    fn get_tzais_90_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(1.5)
    }

    fn get_tzais_96_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(1.6)
    }

    fn get_tzais_90(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + 90 * MINUTE_MILLIS)
    }

    fn get_tzais_120(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + 120 * MINUTE_MILLIS)
    }

    fn get_tzais_120_zmanis(&self) -> Option<i64> {
        self.get_zmanis_based_offset(2.0)
    }

    fn get_tzais_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_16_POINT_1)
    }

    fn get_tzais_26_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_26_DEGREES)
    }

    fn get_tzais_18_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ASTRONOMICAL_ZENITH)
    }

    fn get_tzais_19_point_8_degrees(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_19_POINT_8)
    }

    fn get_tzais_96(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + 96 * MINUTE_MILLIS)
    }

    fn get_tzais_50(&self) -> Option<i64> {
        let sunset = self.zmanim_calendar.astronomical_calendar.get_sunset()?;
        Some(sunset + 50 * MINUTE_MILLIS)
    }

    fn get_tzais_baal_hatanya(&self) -> Option<i64> {
        self.zmanim_calendar
            .astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_6_DEGREES)
    }

    fn get_fixed_local_chatzos(&self) -> Option<i64> {
        None
    }

    fn get_sof_zman_shma_fixed_local(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        Some(chatzos - 180 * MINUTE_MILLIS)
    }

    fn get_sof_zman_tfila_fixed_local(&self) -> Option<i64> {
        let chatzos = self.zmanim_calendar.get_chatzos()?;
        Some(chatzos - 120 * MINUTE_MILLIS)
    }

    fn get_sof_zman_kidush_levana_between_moldos(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }

    fn get_sof_zman_kidush_levana_between_moldos_default(&self) -> Option<i64> {
        self.get_sof_zman_kidush_levana_between_moldos(None, None)
    }

    fn get_sof_zman_kidush_levana_15_days(
        &self,
        _alos: Option<i64>,
        _tzais: Option<i64>,
    ) -> Option<i64> {
        None
    }

    fn get_sof_zman_kidush_levana_15_days_default(&self) -> Option<i64> {
        self.get_sof_zman_kidush_levana_15_days(None, None)
    }

    fn get_tchilas_zman_kidush_levana_3_days(&self) -> Option<i64> {
        self.get_tchilas_zman_kidush_levana_3_days_with_times(None, None)
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
        self.get_tchilas_zman_kidush_levana_7_days(None, None)
    }

    fn get_sof_zman_achilas_chametz_gra(&self) -> Option<i64> {
        self.zmanim_calendar.get_sof_zman_tfila_gra()
    }

    fn get_sof_zman_achilas_chametz_mga_72_minutes(&self) -> Option<i64> {
        self.get_sof_zman_tfila_mga_72_minutes()
    }

    fn get_sof_zman_achilas_chametz_mga_72_minutes_zmanis(&self) -> Option<i64> {
        self.get_sof_zman_tfila_mga_72_minutes_zmanis()
    }

    fn get_sof_zman_achilas_chametz_mga_16_point_1_degrees(&self) -> Option<i64> {
        self.get_sof_zman_tfila_mga_16_point_1_degrees()
    }

    fn get_sof_zman_biur_chametz_gra(&self) -> Option<i64> {
        let sunrise = self.zmanim_calendar.astronomical_calendar.get_sunrise()?;
        let shaah_zmanis = self.zmanim_calendar.get_shaah_zmanis_gra()? as f64;
        Some(sunrise + (shaah_zmanis * 5.0) as i64)
    }

    fn get_sof_zman_biur_chametz_mga_72_minutes(&self) -> Option<i64> {
        let alos = self.zmanim_calendar.get_alos72()?;
        let shaah_zmanis = self.zmanim_calendar.get_shaah_zmanis_mga()? as f64;
        Some(alos + (shaah_zmanis * 5.0) as i64)
    }

    fn get_sof_zman_biur_chametz_mga_72_minutes_zmanis(&self) -> Option<i64> {
        let alos = self.get_alos_72_zmanis()?;
        let shaah_zmanis = self.get_shaah_zmanis_72_minutes_zmanis()? as f64;
        Some(alos + (shaah_zmanis * 5.0) as i64)
    }

    fn get_sof_zman_biur_chametz_mga_16_point_1_degrees(&self) -> Option<i64> {
        let alos = self.get_alos_16_point_1_degrees()?;
        let shaah_zmanis = self.get_shaah_zmanis_16_point_1_degrees()? as f64;
        Some(alos + (shaah_zmanis * 5.0) as i64)
    }

    fn get_sof_zman_achilas_chametz_baal_hatanya(&self) -> Option<i64> {
        self.get_sof_zman_tfila_baal_hatanya()
    }

    fn get_sof_zman_biur_chametz_baal_hatanya(&self) -> Option<i64> {
        let sunrise = self
            .zmanim_calendar
            .astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_6_DEGREES)?;
        let shaah_zmanis = self.get_shaah_zmanis_baal_hatanya()? as f64;
        Some(sunrise + (shaah_zmanis * 5.0) as i64)
    }

    fn get_samuch_le_mincha_ketana_gra(&self) -> Option<i64> {
        self.zmanim_calendar._get_samuch_le_mincha_ketana(
            Some(self.zmanim_calendar.astronomical_calendar.get_sunrise()?),
            self.zmanim_calendar.astronomical_calendar.get_sunset()?,
            true,
        )
    }

    fn get_samuch_le_mincha_ketana_16_point_1_degrees(&self) -> Option<i64> {
        self.zmanim_calendar._get_samuch_le_mincha_ketana(
            self.get_alos_16_point_1_degrees(),
            self.get_tzais_16_point_1_degrees()?,
            true,
        )
    }

    fn get_samuch_le_mincha_ketana_72_minutes(&self) -> Option<i64> {
        self.zmanim_calendar._get_samuch_le_mincha_ketana(
            self.zmanim_calendar.get_alos72(),
            self.zmanim_calendar.get_tzais72()?,
            true,
        )
    }

    fn get_bain_hasmashosrt_13_point_24_degrees(&self) -> Option<i64> {
        self.get_bain_hashmashos_rt_13_point_24_degrees()
    }

    fn get_bain_hasmashosrt_58_point_5_minutes(&self) -> Option<i64> {
        self.get_bain_hashmashos_rt_58_point_5_minutes()
    }

    fn get_bain_hasmashosrt_13_point_5_minutes_before_7_point_083_degrees(&self) -> Option<i64> {
        self.get_bain_hashmashos_rt_13_point_5_minutes_before_7_point_083_degrees()
    }

    fn get_bain_hasmashosrt_2_stars(&self) -> Option<i64> {
        self.get_bain_hashmashos_rt_2_stars()
    }

    fn get_bain_hasmashosyereim_18_minutes(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_18_minutes()
    }

    fn get_bain_hasmashosyereim_3_point_05_degrees(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_3_point_05_degrees()
    }

    fn get_bain_hasmashosyereim_16_point_875_minutes(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_16_point_875_minutes()
    }

    fn get_bain_hasmashosyereim_2_point_8_degrees(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_2_point_8_degrees()
    }

    fn get_bain_hasmashosyereim_13_point_5_minutes(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_13_point_5_minutes()
    }

    fn get_bain_hasmashosyereim_2_point_1_degrees(&self) -> Option<i64> {
        self.get_bain_hashmashos_yereim_2_point_1_degrees()
    }

    fn get_sof_zman_tfilah_ateret_torah(&self) -> Option<i64> {
        self.get_sof_zman_tfila_ateret_torah()
    }

    fn get_fixed_local_chatzos_based_zmanim(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64> {
        self.zmanim_calendar
            .get_half_day_based_zman(start_of_half_day, end_of_half_day, hours)
    }
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl ComplexZmanimCalendar {
    pub fn get_astronomical_calendar(&self) -> AstronomicalCalendar {
        self.zmanim_calendar.astronomical_calendar
    }
    pub fn get_use_astronomical_chatzos(&self) -> bool {
        self.zmanim_calendar.use_astronomical_chatzos
    }
    pub fn get_use_astronomical_chatzos_for_other_zmanim(&self) -> bool {
        self.zmanim_calendar
            .use_astronomical_chatzos_for_other_zmanim
    }
    pub fn get_candle_lighting_offset(&self) -> i64 {
        self.zmanim_calendar.candle_lighting_offset
    }
    pub fn get_ateret_torah_sunset_offset(&self) -> i64 {
        self.ateret_torah_sunset_offset
    }
}
