

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

