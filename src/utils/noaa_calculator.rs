use chrono::{DateTime, Datelike, Timelike};
use core::f64;
use core::f64::consts::PI;
use libm::{acos, asin, cos, floor, fmod, sin, tan};

use crate::utils::{GeoLocation, GeoLocationTrait};

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct AstronomicalCalculator {}
#[derive(Copy, Clone)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct NOAACalculator {}

const JULIAN_DAY_JAN_1_2000: f64 = 2451545.0;
const JULIAN_DAYS_PER_CENTURY: f64 = 36525.0;
const EARTH_RADIUS: f64 = 6356.9;
const GEOMETRIC_ZENITH: f64 = 90.0;
const SOLAR_RADIUS: f64 = 16.0 / 60.0;
const REFRACTION: f64 = 34.0 / 60.0;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
pub enum SolarEvent {
    Sunrise = 1,
    Sunset = 2,
    Noon = 3,
    Midnight = 4,
}

impl Default for NOAACalculator {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    fn default() -> Self {
        Self::new()
    }
}

impl NOAACalculator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_elevation_adjustment(&self, elevation: f64) -> f64 {
        acos(EARTH_RADIUS / (EARTH_RADIUS + (elevation / 1000.0))).to_degrees()
    }
    fn get_sun_rise_set_utc(
        &self,
        timestamp: i64,
        latitude: f64,
        longitude: f64,
        zenith: f64,
        solar_event: SolarEvent,
    ) -> Option<f64> {
        let julian_day = self.get_julian_day(timestamp)?;

        let noonmin = self.get_solar_noon_midnight_utc(julian_day, longitude, SolarEvent::Noon);

        let tnoon = self.get_julian_centuries_from_julian_day(julian_day + noonmin / 1440.0);

        let mut equation_of_time = self.get_equation_of_time(tnoon);
        let mut solar_declination = self.get_sun_declination(tnoon);
        let mut hour_angle =
            self.get_sun_hour_angle(latitude, solar_declination, zenith, solar_event);
        let mut delta = longitude - hour_angle.to_degrees();
        let mut time_diff = 4.0 * delta;
        let mut time_utc = 720.0 + time_diff - equation_of_time;

        let newt = self.get_julian_centuries_from_julian_day(julian_day + time_utc / 1440.0);
        equation_of_time = self.get_equation_of_time(newt);
        solar_declination = self.get_sun_declination(newt);
        hour_angle = self.get_sun_hour_angle(latitude, solar_declination, zenith, solar_event);
        delta = longitude - hour_angle.to_degrees();
        time_diff = 4.0 * delta;
        time_utc = 720.0 + time_diff - equation_of_time;

        Some(time_utc)
    }

    fn get_solar_elevation_azimuth(
        &self,
        timestamp: i64,
        geo_location: &dyn GeoLocationTrait,
        is_azimuth: bool,
    ) -> Option<f64> {
        let latitude = geo_location.get_latitude();
        let longitude = geo_location.get_longitude();

        let dt = DateTime::from_timestamp_millis(timestamp)?;
        let minute: f64 = dt.minute() as f64;
        let second: f64 = dt.second() as f64;
        let hour: f64 = dt.hour() as f64;
        let milli: f64 = dt.nanosecond() as f64 / 1000000.0;

        let time: f64 = (hour + (minute + (second + (milli / 1000.0)) / 60.0) / 60.0) / 24.0;

        let julian_day = self.get_julian_day(timestamp)? + time;
        let julian_centuries = self.get_julian_centuries_from_julian_day(julian_day);

        let eot = self.get_equation_of_time(julian_centuries);
        let theta = self.get_sun_declination(julian_centuries);

        let adjustment = time + eot / 1440.0;
        let true_solar_time = ((adjustment + longitude / 360.0) + 2.0) % 1.0;
        let hour_angle_rad = true_solar_time * PI * 2.0 - PI;

        let cos_zenith = sin(latitude.to_radians()) * sin(theta.to_radians())
            + cos(latitude.to_radians()) * cos(theta.to_radians()) * cos(hour_angle_rad);

        let cos_zenith_clamped = cos_zenith.clamp(-1.0, 1.0);
        let zenith = acos(cos_zenith_clamped).to_degrees();

        let az_denom = cos(latitude.to_radians()) * sin(zenith.to_radians());
        let refraction_adjustment = 0.0;
        let elevation = 90.0 - (zenith - refraction_adjustment);
        if is_azimuth {
            let azimuth = if az_denom.abs() > 0.001 {
                let az_rad = (sin(latitude.to_radians()) * cos(zenith.to_radians())
                    - sin(theta.to_radians()))
                    / az_denom;

                let az_rad_clamped = az_rad.clamp(-1.0, 1.0);
                180.0
                    - acos(az_rad_clamped).to_degrees()
                        * if hour_angle_rad > 0.0 { -1.0 } else { 1.0 }
            } else if latitude > 0.0 {
                180.0
            } else {
                0.0
            };
            Some(azimuth % 360.0)
        } else {
            Some(elevation)
        }
    }
    fn adjust_zenith(&self, zenith: f64, elevation: f64) -> f64 {
        let mut adjusted_zenith = zenith;
        if zenith == GEOMETRIC_ZENITH {
            adjusted_zenith =
                zenith + (SOLAR_RADIUS + REFRACTION + self.get_elevation_adjustment(elevation));
        }
        adjusted_zenith
    }
    fn get_julian_day(&self, timestamp: i64) -> Option<f64> {
        let dt = DateTime::from_timestamp_millis(timestamp)?;
        let mut year: i32 = dt.year();
        let mut month: i32 = dt.month() as i32;
        let day: i32 = dt.day() as i32;
        if month <= 2 {
            year -= 1;
            month += 12;
        }
        let a: i32 = year / 100;
        let b: i32 = 2 - a + a / 4;
        Some(
            floor(365.25 * (year + 4716) as f64)
                + floor(30.6001 * (month + 1) as f64)
                + day as f64
                + b as f64
                - 1524.5,
        )
    }
    fn get_julian_centuries_from_julian_day(&self, julian_day: f64) -> f64 {
        (julian_day - JULIAN_DAY_JAN_1_2000) / JULIAN_DAYS_PER_CENTURY
    }
    fn get_sun_geometric_mean_longitude(&self, julian_centuries: f64) -> f64 {
        let longitude = 280.46646 + julian_centuries * (36000.76983 + 0.0003032 * julian_centuries);

        let mut r = fmod(longitude, 360.0);
        if r < 0.0 {
            r += 360.0;
        }
        r
    }

    fn get_sun_geometric_mean_anomaly(&self, julian_centuries: f64) -> f64 {
        357.52911 + julian_centuries * (35999.05029 - 0.0001537 * julian_centuries)
    }

    fn get_earth_orbit_eccentricity(&self, julian_centuries: f64) -> f64 {
        0.016708634 - julian_centuries * (0.000042037 + 0.0000001267 * julian_centuries)
    }

    fn get_sun_equation_of_center(&self, julian_centuries: f64) -> f64 {
        let m = self.get_sun_geometric_mean_anomaly(julian_centuries);
        let m_rad = m.to_radians();
        let sin_m = sin(m_rad);
        let sin_2m = sin(m_rad * 2.0);
        let sin_3m = sin(m_rad * 3.0);

        sin_m * (1.914602 - julian_centuries * (0.004817 + 0.000014 * julian_centuries))
            + sin_2m * (0.019993 - 0.000101 * julian_centuries)
            + sin_3m * 0.000289
    }

    fn get_sun_true_longitude(&self, julian_centuries: f64) -> f64 {
        let sun_longitude = self.get_sun_geometric_mean_longitude(julian_centuries);
        let center = self.get_sun_equation_of_center(julian_centuries);
        sun_longitude + center
    }

    fn get_sun_apparent_longitude(&self, julian_centuries: f64) -> f64 {
        let sun_true_longitude = self.get_sun_true_longitude(julian_centuries);
        let omega = 125.04 - 1934.136 * julian_centuries;
        sun_true_longitude - 0.00569 - 0.00478 * sin(omega.to_radians())
    }

    fn get_mean_obliquity_of_ecliptic(&self, julian_centuries: f64) -> f64 {
        let seconds = 21.448
            - julian_centuries
                * (46.8150 + julian_centuries * (0.00059 - julian_centuries * (0.001813)));
        23.0 + (26.0 + (seconds / 60.0)) / 60.0
    }

    fn get_obliquity_correction(&self, julian_centuries: f64) -> f64 {
        let obliquity_of_ecliptic = self.get_mean_obliquity_of_ecliptic(julian_centuries);
        let omega = 125.04 - 1934.136 * julian_centuries;
        obliquity_of_ecliptic + 0.00256 * cos(omega.to_radians())
    }

    fn get_sun_declination(&self, julian_centuries: f64) -> f64 {
        let obliquity_correction = self.get_obliquity_correction(julian_centuries);
        let lambda = self.get_sun_apparent_longitude(julian_centuries);
        let sin_t = sin(obliquity_correction.to_radians()) * sin(lambda.to_radians());
        asin(sin_t).to_degrees()
    }

    fn get_equation_of_time(&self, julian_centuries: f64) -> f64 {
        let epsilon = self.get_obliquity_correction(julian_centuries);
        let geom_mean_long_sun = self.get_sun_geometric_mean_longitude(julian_centuries);
        let eccentricity_earth_orbit = self.get_earth_orbit_eccentricity(julian_centuries);
        let geom_mean_anomaly_sun = self.get_sun_geometric_mean_anomaly(julian_centuries);

        let mut y = tan(epsilon.to_radians() / 2.0);
        y *= y;

        let sin_2l0 = sin(2.0 * geom_mean_long_sun.to_radians());
        let sin_m = sin(geom_mean_anomaly_sun.to_radians());
        let cos_2l0 = cos(2.0 * geom_mean_long_sun.to_radians());
        let sin_4l0 = sin(4.0 * geom_mean_long_sun.to_radians());
        let sin_2m = sin(2.0 * geom_mean_anomaly_sun.to_radians());

        let equation_of_time = y * sin_2l0 - 2.0 * eccentricity_earth_orbit * sin_m
            + 4.0 * eccentricity_earth_orbit * y * sin_m * cos_2l0
            - 0.5 * y * y * sin_4l0
            - 1.25 * eccentricity_earth_orbit * eccentricity_earth_orbit * sin_2m;

        equation_of_time.to_degrees() * 4.0
    }

    fn get_sun_hour_angle(
        &self,
        latitude: f64,
        solar_declination: f64,
        zenith: f64,
        solar_event: SolarEvent,
    ) -> f64 {
        let lat_rad = latitude.to_radians();
        let sd_rad = solar_declination.to_radians();

        let cos_h = (cos(zenith.to_radians()) / (cos(lat_rad) * cos(sd_rad)))
            - (tan(lat_rad) * tan(sd_rad));

        let hour_angle = acos(cos_h);

        if solar_event == SolarEvent::Sunset {
            -hour_angle
        } else {
            hour_angle
        }
    }

    fn get_solar_noon_midnight_utc(
        &self,
        julian_day: f64,
        longitude: f64,
        solar_event: SolarEvent,
    ) -> f64 {
        let julian_day = if solar_event == SolarEvent::Noon {
            julian_day
        } else {
            julian_day + 0.5
        };

        let t_noon = self.get_julian_centuries_from_julian_day(julian_day + longitude / 360.0);
        let mut equation_of_time = self.get_equation_of_time(t_noon);
        let sol_noon_utc = (longitude * 4.0) - equation_of_time;

        let new_t = self.get_julian_centuries_from_julian_day(julian_day + sol_noon_utc / 1440.0);
        equation_of_time = self.get_equation_of_time(new_t);

        let base_minutes = if solar_event == SolarEvent::Noon {
            720.0
        } else {
            1440.0
        };
        base_minutes + (longitude * 4.0) - equation_of_time
    }
}

pub trait NOAACalculatorTrait {
    fn get_utc_sunrise(
        &self,
        timestamp: i64,
        geo_location: &GeoLocation,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64>;
    fn get_utc_sunset(
        &self,
        timestamp: i64,
        geo_location: &GeoLocation,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64>;
    fn get_solar_elevation(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64>;
    fn get_solar_azimuth(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64>;
    fn get_utc_noon(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64>;
    fn get_utc_midnight(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64>;
}
#[cfg_attr(feature = "uniffi", uniffi::export)]
impl NOAACalculatorTrait for NOAACalculator {
    fn get_utc_noon(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64> {
        let julian_day = self.get_julian_day(timestamp)?;
        let noon = self.get_solar_noon_midnight_utc(
            julian_day,
            -geo_location.get_longitude(),
            SolarEvent::Noon,
        );
        let noon_hours = noon / 60.0;
        Some(if noon_hours > 0.0 {
            noon_hours % 24.0
        } else {
            noon_hours % 24.0 + 24.0
        })
    }

    fn get_utc_midnight(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64> {
        let julian_day = self.get_julian_day(timestamp)?;
        let midnight = self.get_solar_noon_midnight_utc(
            julian_day,
            -geo_location.get_longitude(),
            SolarEvent::Midnight,
        );
        let midnight_hours = midnight / 60.0;
        if midnight_hours > 0.0 {
            Some(midnight_hours % 24.0)
        } else {
            Some(midnight_hours % 24.0 + 24.0)
        }
    }

    fn get_utc_sunrise(
        &self,
        timestamp: i64,
        geo_location: &GeoLocation,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let elevation = if adjust_for_elevation {
            geo_location.get_elevation()
        } else {
            0.0
        };
        let adjusted_zenith = self.adjust_zenith(zenith, elevation);
        let sunrise = self.get_sun_rise_set_utc(
            timestamp,
            geo_location.get_latitude(),
            -geo_location.get_longitude(),
            adjusted_zenith,
            SolarEvent::Sunrise,
        )?;
        let sunrise_hours = sunrise / 60.0;
        Some(if sunrise_hours > 0.0 {
            sunrise_hours % 24.0
        } else {
            sunrise_hours % 24.0 + 24.0
        })
    }

    fn get_utc_sunset(
        &self,
        timestamp: i64,
        geo_location: &GeoLocation,
        zenith: f64,
        adjust_for_elevation: bool,
    ) -> Option<f64> {
        let elevation = if adjust_for_elevation {
            geo_location.get_elevation()
        } else {
            0.0
        };
        let adjusted_zenith = self.adjust_zenith(zenith, elevation);
        let sunset = self.get_sun_rise_set_utc(
            timestamp,
            geo_location.get_latitude(),
            -geo_location.get_longitude(),
            adjusted_zenith,
            SolarEvent::Sunset,
        )?;
        let sunset_hours = sunset / 60.0;
        Some(if sunset_hours > 0.0 {
            sunset_hours % 24.0
        } else {
            sunset_hours % 24.0 + 24.0
        })
    }

    fn get_solar_elevation(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64> {
        self.get_solar_elevation_azimuth(timestamp, geo_location, false)
    }

    fn get_solar_azimuth(&self, timestamp: i64, geo_location: &GeoLocation) -> Option<f64> {
        self.get_solar_elevation_azimuth(timestamp, geo_location, true)
    }
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
pub fn new_noaa_calculator() -> NOAACalculator {
    NOAACalculator::new()
}
