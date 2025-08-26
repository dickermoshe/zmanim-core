use j4rs::{Instance, Jvm};
use zmanim_core::prelude::*;

#[allow(dead_code)]
pub fn create_solar_event(jvm: &Jvm, solar_event: SolarEvent) -> Instance {
    let string = match solar_event {
        SolarEvent::Sunrise => "SUNRISE",
        SolarEvent::Sunset => "SUNSET",
        SolarEvent::Noon => "NOON",
        SolarEvent::Midnight => "MIDNIGHT",
    };
    let enum_class = jvm
        .static_class("com.kosherjava.zmanim.util.NOAACalculator$SolarEvent")
        .unwrap();
    let enum_field = jvm.field(&enum_class, string).unwrap();
    enum_field
}
