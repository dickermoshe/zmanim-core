use zmanim_core::{
    GeoLocation, astronomical_calendar::AstronomicalCalendarTrait,
    complex_zmanim_calendar::ComplexZmanimCalendar,
};

fn main() {
    let now = chrono::Utc::now();
    let geo_location = GeoLocation::new(40.7128, -74.0060, 0.0).unwrap();
    let complex_zmanim_calendar =
        ComplexZmanimCalendar::new(now.timestamp_millis(), geo_location, true, true, 0, 0);
    println!(
        "Shkia is at {:?}",
        complex_zmanim_calendar
            .get_zmanim_calendar()
            .get_astronomical_calendar()
            .get_sunset()
    );
}
