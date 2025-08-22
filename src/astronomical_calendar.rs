use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

use crate::{GeoLocationTrait, NOAACalculator, NOAACalculatorTrait, SolarEvent};

pub struct AstronomicalCalendar<'a> {
    timestamp: i64,
    geo_location: &'a dyn GeoLocationTrait,
    noaa_calculator: NOAACalculator,
}

pub const GEOMETRIC_ZENITH: f64 = 90.0;

const CIVIL_ZENITH: f64 = 96.0;

const NAUTICAL_ZENITH: f64 = 102.0;

const ASTRONOMICAL_ZENITH: f64 = 108.0;

pub const MINUTE_MILLIS: i64 = 60 * 1000;

pub trait AstronomicalCalendarTrait {
    fn get_utc_sunset(&self, zenith: f64) -> f64;
    fn get_utc_sunrise(&self, zenith: f64) -> f64;
    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> f64;
    fn get_utc_sea_level_sunset(&self, zenith: f64) -> f64;

    fn get_sea_level_sunset(&self) -> Option<i64>;
    fn get_sunset(&self) -> Option<i64>;
    fn get_sunrise(&self) -> Option<i64>;
    fn get_sea_level_sunrise(&self) -> Option<i64>;
    fn get_sunrise_offset_by_degrees(&self, degrees: f64) -> Option<i64>;
    fn get_sunset_offset_by_degrees(&self, degrees: f64) -> Option<i64>;
    fn get_begin_civil_twilight(&self) -> Option<i64>;
    fn get_begin_nautical_twilight(&self) -> Option<i64>;
    fn get_begin_astronomical_twilight(&self) -> Option<i64>;
    fn get_end_civil_twilight(&self) -> Option<i64>;
    fn get_end_nautical_twilight(&self) -> Option<i64>;
    fn get_end_astronomical_twilight(&self) -> Option<i64>;
    fn get_temporal_hour(&self) -> Option<i64>;
    fn get_sun_transit(&self) -> Option<i64>;
    fn get_solar_midnight(&self) -> Option<i64>;
    fn get_temporal_hour_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64>;
    fn get_sun_transit_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64>;
}

impl<'a> AstronomicalCalendar<'a> {
    pub fn new(timestamp: i64, geo_location: &'a dyn GeoLocationTrait) -> Self {
        Self {
            timestamp,
            geo_location,
            noaa_calculator: NOAACalculator::new(),
        }
    }

    fn get_date_from_time(&self, time: f64, solar_event: SolarEvent) -> Option<i64> {
        if time.is_nan() {
            return None;
        }

        let mut calculated_time = time;

        // Get the adjusted calendar (base date from timestamp)
        let adjusted_dt = DateTime::from_timestamp_millis(self.timestamp)?;

        // Create a new UTC datetime with the same date components
        let mut cal = Utc
            .with_ymd_and_hms(
                adjusted_dt.year(),
                adjusted_dt.month(),
                adjusted_dt.day(),
                0,
                0,
                0,
            )
            .unwrap();

        // Extract hours, minutes, seconds from the calculated time
        let hours = calculated_time as i32;
        calculated_time -= hours as f64;

        calculated_time *= 60.0; // This matches Java's calculatedTime *= 60
        let minutes = calculated_time as i32;
        calculated_time -= minutes as f64;

        calculated_time *= 60.0; // This matches Java's calculatedTime *= 60
        let seconds = calculated_time as i32;
        calculated_time -= seconds as f64;

        // Calculate local time offset based on longitude
        let local_time_hours = (self.geo_location.get_longitude() / 15.0) as i32;

        // Adjust date based on solar event and local time
        if solar_event == SolarEvent::Sunrise && local_time_hours + hours > 18 {
            cal = cal - chrono::Duration::days(1);
        } else if solar_event == SolarEvent::Sunset && local_time_hours + hours < 6 {
            cal = cal + chrono::Duration::days(1);
        } else if solar_event == SolarEvent::Midnight && local_time_hours + hours < 12 {
            cal = cal + chrono::Duration::days(1);
        }

        // Set the time components
        cal = cal
            .with_hour(hours as u32)
            .and_then(|dt| dt.with_minute(minutes as u32))
            .and_then(|dt| dt.with_second(seconds as u32))
            .and_then(|dt| dt.with_nanosecond((calculated_time * 1_000_000_000.0) as u32))?;

        // Return timestamp in milliseconds
        Some(cal.timestamp_millis())
    }
}

impl<'a> AstronomicalCalendarTrait for AstronomicalCalendar<'a> {
    fn get_utc_sunset(&self, zenith: f64) -> f64 {
        self.noaa_calculator
            .get_utc_sunset(self.timestamp, self.geo_location, zenith, true)
    }

    fn get_utc_sunrise(&self, zenith: f64) -> f64 {
        self.noaa_calculator
            .get_utc_sunrise(self.timestamp, self.geo_location, zenith, true)
    }

    fn get_utc_sea_level_sunrise(&self, zenith: f64) -> f64 {
        self.noaa_calculator
            .get_utc_sunrise(self.timestamp, self.geo_location, zenith, false)
    }

    fn get_utc_sea_level_sunset(&self, zenith: f64) -> f64 {
        self.noaa_calculator
            .get_utc_sunset(self.timestamp, self.geo_location, zenith, false)
    }

    fn get_sea_level_sunset(&self) -> Option<i64> {
        let result = self.get_utc_sea_level_sunset(GEOMETRIC_ZENITH);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunset);
    }
    fn get_sunset(&self) -> Option<i64> {
        let result = self.get_utc_sunset(GEOMETRIC_ZENITH);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunset);
    }
    fn get_sunrise(&self) -> Option<i64> {
        let result = self.get_utc_sunrise(GEOMETRIC_ZENITH);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunrise);
    }
    fn get_sea_level_sunrise(&self) -> Option<i64> {
        let result = self.get_utc_sea_level_sunrise(GEOMETRIC_ZENITH);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunrise);
    }
    fn get_sunrise_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        let result = self.get_utc_sunrise(degrees);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunrise);
    }
    fn get_sunset_offset_by_degrees(&self, degrees: f64) -> Option<i64> {
        let result = self.get_utc_sunset(degrees);
        if result.is_nan() {
            return None;
        }
        return self.get_date_from_time(result, SolarEvent::Sunset);
    }
    fn get_begin_civil_twilight(&self) -> Option<i64> {
        return self.get_sunrise_offset_by_degrees(CIVIL_ZENITH);
    }
    fn get_begin_nautical_twilight(&self) -> Option<i64> {
        return self.get_sunrise_offset_by_degrees(NAUTICAL_ZENITH);
    }
    fn get_begin_astronomical_twilight(&self) -> Option<i64> {
        return self.get_sunrise_offset_by_degrees(ASTRONOMICAL_ZENITH);
    }
    fn get_end_civil_twilight(&self) -> Option<i64> {
        return self.get_sunset_offset_by_degrees(CIVIL_ZENITH);
    }
    fn get_end_nautical_twilight(&self) -> Option<i64> {
        return self.get_sunset_offset_by_degrees(NAUTICAL_ZENITH);
    }
    fn get_end_astronomical_twilight(&self) -> Option<i64> {
        return self.get_sunset_offset_by_degrees(ASTRONOMICAL_ZENITH);
    }
    fn get_temporal_hour(&self) -> Option<i64> {
        let sea_level_sunrise = self.get_sea_level_sunrise()?;
        let sea_level_sunset = self.get_sea_level_sunset()?;
        return self
            .get_temporal_hour_with_start_and_end_times(sea_level_sunrise, sea_level_sunset);
    }
    fn get_temporal_hour_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        return Some((end_time - start_time) as i64 / 12);
    }

    fn get_sun_transit(&self) -> Option<i64> {
        let noon = self
            .noaa_calculator
            .get_utc_noon(self.timestamp, self.geo_location);
        if noon.is_nan() {
            return None;
        }
        return self.get_date_from_time(noon, SolarEvent::Noon);
    }
    fn get_sun_transit_with_start_and_end_times(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Option<i64> {
        let temporal_hour =
            self.get_temporal_hour_with_start_and_end_times(start_time, end_time)?;
        Some(start_time + (temporal_hour * 6))
    }

    fn get_solar_midnight(&self) -> Option<i64> {
        let midnight = self
            .noaa_calculator
            .get_utc_midnight(self.timestamp, self.geo_location);
        if midnight.is_nan() {
            return None;
        }
        return self.get_date_from_time(midnight, SolarEvent::Midnight);
    }
}
