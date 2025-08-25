use crate::{
    astronomical_calendar::{
        AstronomicalCalendar, AstronomicalCalendarTrait, GEOMETRIC_ZENITH, MINUTE_MILLIS,
    },
    geolocation::GeoLocation,
};

use safer_ffi::prelude::*;

const ZENITH_16_POINT_1: f64 = GEOMETRIC_ZENITH + 16.1;
const ZENITH_8_POINT_5: f64 = GEOMETRIC_ZENITH + 8.5;

#[derive_ReprC]
#[repr(C)]
pub struct ZmanimCalendar {
    astronomical_calendar: AstronomicalCalendar,
    use_astronomical_chatzos: bool,
    use_astronomical_chatzos_for_other_zmanim: bool,
    candle_lighting_offset: i64,
}

pub trait ZmanimCalendarTrait {
    fn get_tzais(&self) -> Option<i64>;
    fn get_alos_hashachar(&self) -> Option<i64>;
    fn get_alos72(&self) -> Option<i64>;
    fn get_chatzos(&self) -> Option<i64>;
    fn get_chatzos_as_half_day(&self) -> Option<i64>;
    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64>;
    fn get_half_day_based_zman(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64>;
    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
    ) -> Option<i64>;
    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: i64,
        end_of_day: i64,
        hours: f64,
    ) -> Option<i64>;
    fn _get_sof_zman_shma(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_sof_zman_shma_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;
    fn get_sof_zman_shma_gra(&self) -> Option<i64>;
    fn get_sof_zman_shma_mga(&self) -> Option<i64>;
    fn get_tzais72(&self) -> Option<i64>;
    fn get_candle_lighting(&self) -> Option<i64>;
    fn _get_sof_zman_tfila(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_sof_zman_tfila_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;
    fn get_sof_zman_tfila_gra(&self) -> Option<i64>;
    fn get_sof_zman_tfila_mga(&self) -> Option<i64>;
    fn _get_mincha_gedola(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_mincha_gedola_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;
    fn get_mincha_gedola_default(&self) -> Option<i64>;
    fn _get_samuch_le_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_samuch_le_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64)
        -> Option<i64>;
    fn _get_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;
    fn get_mincha_ketana_default(&self) -> Option<i64>;
    fn _get_plag_hamincha(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64>;
    fn get_plag_hamincha_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64>;
    fn get_plag_hamincha_default(&self) -> Option<i64>;
    fn get_shaah_zmanis_gra(&self) -> Option<i64>;
    fn get_shaah_zmanis_mga(&self) -> Option<i64>;
}

impl ZmanimCalendar {
    pub fn new(
        timestamp: i64,
        geo_location: GeoLocation,

        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: i64,
    ) -> Self {
        Self {
            astronomical_calendar: AstronomicalCalendar::new(timestamp, geo_location),
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
        }
    }
    pub fn get_astronomical_calendar(&self) -> &AstronomicalCalendar {
        &self.astronomical_calendar
    }
}

impl ZmanimCalendarTrait for ZmanimCalendar {
    fn get_tzais(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_sunset_offset_by_degrees(ZENITH_8_POINT_5)
    }

    fn get_alos_hashachar(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_sunrise_offset_by_degrees(ZENITH_16_POINT_1)
    }
    fn get_alos72(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_sunrise()
            .map(|sunrise| sunrise + -72 * MINUTE_MILLIS)
    }
    fn get_chatzos(&self) -> Option<i64> {
        if self.use_astronomical_chatzos {
            self.astronomical_calendar.get_sun_transit()
        } else {
            self.get_chatzos_as_half_day()
                .or(self.astronomical_calendar.get_sun_transit())
        }
    }
    fn get_chatzos_as_half_day(&self) -> Option<i64> {
        let sunrise = self.astronomical_calendar.get_sea_level_sunrise()?;
        let sunset = self.astronomical_calendar.get_sea_level_sunset()?;
        let chatzos = self
            .astronomical_calendar
            .get_sun_transit_with_start_and_end_times(sunrise, sunset)?;
        Some(chatzos)
    }

    fn get_percent_of_shaah_zmanis_from_degrees(&self, degrees: f64, sunset: bool) -> Option<f64> {
        let sea_level_sunrise = self.astronomical_calendar.get_sea_level_sunrise();
        let sea_level_sunset = self.astronomical_calendar.get_sea_level_sunset();

        let twilight = if sunset {
            self.astronomical_calendar
                .get_sunset_offset_by_degrees(GEOMETRIC_ZENITH + degrees)
        } else {
            self.astronomical_calendar
                .get_sunrise_offset_by_degrees(GEOMETRIC_ZENITH + degrees)
        };

        let result = match (sea_level_sunrise, sea_level_sunset, twilight) {
            (Some(sunrise), Some(sunset_time), Some(twilight_time)) => {
                let shaah_zmanis = (sunset_time - sunrise) as f64 / 12.0;
                let rise_set_to_twilight = if sunset {
                    twilight_time - sunset_time
                } else {
                    sunrise - twilight_time
                };
                Some(rise_set_to_twilight as f64 / shaah_zmanis)
            }
            _ => None,
        };
        result
    }

    fn get_half_day_based_zman(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let shaah_zmanis =
            self.get_half_day_based_shaah_zmanis(start_of_half_day, end_of_half_day)?;
        if hours >= 0.0 {
            Some(start_of_half_day + (shaah_zmanis as f64 * hours) as i64)
        } else {
            Some(end_of_half_day + (shaah_zmanis as f64 * hours) as i64)
        }
    }

    fn get_half_day_based_shaah_zmanis(
        &self,
        start_of_half_day: i64,
        end_of_half_day: i64,
    ) -> Option<i64> {
        Some((end_of_half_day - start_of_half_day) / 6)
    }

    fn get_shaah_zmanis_based_zman(
        &self,
        start_of_day: i64,
        end_of_day: i64,
        hours: f64,
    ) -> Option<i64> {
        let shaah_zmanis = self
            .astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(start_of_day, end_of_day)?;
        Some(start_of_day + (shaah_zmanis as f64 * hours) as i64)
    }

    fn _get_sof_zman_shma(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(start_of_day, self.get_chatzos()?, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day, end_of_day?, 3.0)
        }
    }

    fn get_sof_zman_shma_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        self._get_sof_zman_shma(start_of_day, Some(end_of_day), false)
    }

    fn get_sof_zman_shma_gra(&self) -> Option<i64> {
        self._get_sof_zman_shma(
            self.astronomical_calendar.get_sunrise()?,
            self.astronomical_calendar.get_sunset(),
            true,
        )
    }

    fn get_sof_zman_shma_mga(&self) -> Option<i64> {
        self._get_sof_zman_shma(self.get_alos72()?, self.get_tzais72(), true)
    }

    fn get_tzais72(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_sunset()
            .map(|sunset| sunset + 72 * MINUTE_MILLIS)
    }

    fn get_candle_lighting(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_sea_level_sunset()
            .map(|sunset| sunset - self.candle_lighting_offset)
    }

    fn _get_sof_zman_tfila(
        &self,
        start_of_day: i64,
        end_of_day: Option<i64>,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(start_of_day, self.get_chatzos()?, 4.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day, end_of_day?, 4.0)
        }
    }

    fn get_sof_zman_tfila_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        self._get_sof_zman_tfila(start_of_day, Some(end_of_day), false)
    }

    fn get_sof_zman_tfila_gra(&self) -> Option<i64> {
        self._get_sof_zman_tfila(
            self.astronomical_calendar.get_sunrise()?,
            self.astronomical_calendar.get_sunset(),
            true,
        )
    }

    fn get_sof_zman_tfila_mga(&self) -> Option<i64> {
        self._get_sof_zman_tfila(self.get_alos72()?, self.get_tzais72(), true)
    }

    fn _get_mincha_gedola(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 0.5)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 6.5)
        }
    }

    fn get_mincha_gedola_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        self._get_mincha_gedola(Some(start_of_day), end_of_day, false)
    }

    fn get_mincha_gedola_default(&self) -> Option<i64> {
        self._get_mincha_gedola(
            self.astronomical_calendar.get_sunrise(),
            self.astronomical_calendar.get_sunset()?,
            true,
        )
    }

    fn _get_samuch_le_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 3.0)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 9.0)
        }
    }

    fn get_samuch_le_mincha_ketana_simple(
        &self,
        start_of_day: i64,
        end_of_day: i64,
    ) -> Option<i64> {
        self._get_samuch_le_mincha_ketana(Some(start_of_day), end_of_day, false)
    }

    fn _get_mincha_ketana(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 3.5)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 9.5)
        }
    }

    fn get_mincha_ketana_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        self._get_mincha_ketana(Some(start_of_day), end_of_day, false)
    }

    fn get_mincha_ketana_default(&self) -> Option<i64> {
        self._get_mincha_ketana(
            self.astronomical_calendar.get_sunrise(),
            self.astronomical_calendar.get_sunset()?,
            true,
        )
    }

    fn _get_plag_hamincha(
        &self,
        start_of_day: Option<i64>,
        end_of_day: i64,
        synchronous: bool,
    ) -> Option<i64> {
        if self.use_astronomical_chatzos_for_other_zmanim && synchronous {
            self.get_half_day_based_zman(self.get_chatzos()?, end_of_day, 4.75)
        } else {
            self.get_shaah_zmanis_based_zman(start_of_day?, end_of_day, 10.75)
        }
    }

    fn get_plag_hamincha_simple(&self, start_of_day: i64, end_of_day: i64) -> Option<i64> {
        self._get_plag_hamincha(Some(start_of_day), end_of_day, false)
    }

    fn get_plag_hamincha_default(&self) -> Option<i64> {
        self._get_plag_hamincha(
            self.astronomical_calendar.get_sunrise(),
            self.astronomical_calendar.get_sunset()?,
            true,
        )
    }

    fn get_shaah_zmanis_gra(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(
                self.astronomical_calendar.get_sunrise()?,
                self.astronomical_calendar.get_sunset()?,
            )
    }

    fn get_shaah_zmanis_mga(&self) -> Option<i64> {
        self.astronomical_calendar
            .get_temporal_hour_with_start_and_end_times(self.get_alos72()?, self.get_tzais72()?)
    }
}

// FFI Functions for ZmanimCalendar
#[cfg(feature = "ffi")]
pub mod zmanim_calendar_ffi {
    use super::*;
    use safer_ffi::option::TaggedOption;

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_new(
        timestamp: i64,
        geo_location: GeoLocation,
        use_astronomical_chatzos: bool,
        use_astronomical_chatzos_for_other_zmanim: bool,
        candle_lighting_offset: i64,
    ) -> ZmanimCalendar {
        ZmanimCalendar::new(
            timestamp,
            geo_location,
            use_astronomical_chatzos,
            use_astronomical_chatzos_for_other_zmanim,
            candle_lighting_offset,
        )
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_tzais(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_tzais().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_alos_hashachar(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_alos_hashachar().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_alos72(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_alos72().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_chatzos(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_chatzos().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_chatzos_as_half_day(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_chatzos_as_half_day().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_percent_of_shaah_zmanis_from_degrees(
        calendar: &ZmanimCalendar,
        degrees: f64,
        sunset: bool,
    ) -> TaggedOption<f64> {
        calendar
            .get_percent_of_shaah_zmanis_from_degrees(degrees, sunset)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_half_day_based_zman(
        calendar: &ZmanimCalendar,
        start_of_half_day: i64,
        end_of_half_day: i64,
        hours: f64,
    ) -> TaggedOption<i64> {
        calendar
            .get_half_day_based_zman(start_of_half_day, end_of_half_day, hours)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_half_day_based_shaah_zmanis(
        calendar: &ZmanimCalendar,
        start_of_half_day: i64,
        end_of_half_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_half_day_based_shaah_zmanis(start_of_half_day, end_of_half_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_shaah_zmanis_based_zman(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
        hours: f64,
    ) -> TaggedOption<i64> {
        calendar
            .get_shaah_zmanis_based_zman(start_of_day, end_of_day, hours)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_shma_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_sof_zman_shma_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_shma_gra(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_sof_zman_shma_gra().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_shma_mga(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_sof_zman_shma_mga().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_tzais72(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_tzais72().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_candle_lighting(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_candle_lighting().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_tfila_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_sof_zman_tfila_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_tfila_gra(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_sof_zman_tfila_gra().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_sof_zman_tfila_mga(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_sof_zman_tfila_mga().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_mincha_gedola_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_mincha_gedola_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_mincha_gedola_default(
        calendar: &ZmanimCalendar,
    ) -> TaggedOption<i64> {
        calendar.get_mincha_gedola_default().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_samuch_le_mincha_ketana_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_samuch_le_mincha_ketana_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_mincha_ketana_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_mincha_ketana_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_mincha_ketana_default(
        calendar: &ZmanimCalendar,
    ) -> TaggedOption<i64> {
        calendar.get_mincha_ketana_default().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_plag_hamincha_simple(
        calendar: &ZmanimCalendar,
        start_of_day: i64,
        end_of_day: i64,
    ) -> TaggedOption<i64> {
        calendar
            .get_plag_hamincha_simple(start_of_day, end_of_day)
            .into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_plag_hamincha_default(
        calendar: &ZmanimCalendar,
    ) -> TaggedOption<i64> {
        calendar.get_plag_hamincha_default().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_shaah_zmanis_gra(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_shaah_zmanis_gra().into()
    }

    #[no_mangle]
    #[ffi_export]
    pub fn zmanim_calendar_get_shaah_zmanis_mga(calendar: &ZmanimCalendar) -> TaggedOption<i64> {
        calendar.get_shaah_zmanis_mga().into()
    }
}
