
impl NOAACalculator {


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

