use zmanim_core::{
    hebrew_calendar::{JewishCalendar, JewishCalendarTrait, JewishDateTrait},
    prelude::jewish_calendar::GetDafYomiBavliTrait,
};

mod java;
mod test_utils;
use java::jewish_calendar::JavaJewishCalendar;

#[allow(dead_code)]
fn compare_calendar_methods(
    rust_cal: &JewishCalendar,
    java_cal: &JavaJewishCalendar,
    timestamp: i64,
    tz_offset: i64,
) {
    let formatted_java_date = format!(
        "{:?} {:?} {:?} {:?}",
        java_cal.get_jewish_year(),
        java_cal.get_jewish_month_int(),
        java_cal.get_jewish_day_of_month(),
        java_cal.get_day_of_week_int(),
    );
    let formatted_rust_date = format!(
        "{:?} {:?} {:?} {:?}",
        rust_cal.jewish_date.get_jewish_year(),
        rust_cal.jewish_date.get_jewish_month(),
        rust_cal.jewish_date.get_jewish_day_of_month(),
        rust_cal.jewish_date.get_day_of_week(),
    );
    let message = format!(
        "Yom Tov index mismatch ts={}, tz={}, rust={}, java={}",
        timestamp, tz_offset, formatted_rust_date, formatted_java_date
    );
    assert_eq!(rust_cal.in_israel, java_cal.get_in_israel());
    assert_eq!(
        rust_cal.use_modern_holidays,
        java_cal.get_use_modern_holidays()
    );

    assert_eq!(
        rust_cal.get_yom_tov_index(),
        java_cal.get_yom_tov_index(),
        "{}",
        message
    );
    let formatted_java_date = format!(
        "{:?} {:?} {:?} {:?}",
        java_cal.get_jewish_year(),
        java_cal.get_jewish_month_int(),
        java_cal.get_jewish_day_of_month(),
        java_cal.get_day_of_week_int(),
    );
    let formatted_rust_date = format!(
        "{:?} {:?} {:?} {:?}",
        rust_cal.jewish_date.get_jewish_year(),
        rust_cal.jewish_date.get_jewish_month(),
        rust_cal.jewish_date.get_jewish_day_of_month(),
        rust_cal.jewish_date.get_day_of_week(),
    );
    let base_message = format!(
        "Method mismatch ts={}, tz={}, rust={}, java={}",
        timestamp, tz_offset, formatted_rust_date, formatted_java_date
    );

    assert_eq!(
        rust_cal.is_yom_tov(),
        java_cal.is_yom_tov(),
        "{} - is_yom_tov",
        base_message
    );

    assert_eq!(
        rust_cal.is_yom_tov_assur_bemelacha(),
        java_cal.is_yom_tov_assur_bemelacha(),
        "{} - is_yom_tov_assur_bemelacha",
        base_message
    );

    assert_eq!(
        rust_cal.is_assur_bemelacha(),
        java_cal.is_assur_bemelacha(),
        "{} - is_assur_bemelacha",
        base_message
    );

    assert_eq!(
        rust_cal.has_candle_lighting(),
        java_cal.has_candle_lighting(),
        "{} - has_candle_lighting",
        base_message
    );

    assert_eq!(
        rust_cal.is_tomorrow_shabbos_or_yom_tov(),
        java_cal.is_tomorrow_shabbos_or_yom_tov(),
        "{} - is_tomorrow_shabbos_or_yom_tov",
        base_message
    );

    assert_eq!(
        rust_cal.is_erev_yom_tov_sheni(),
        java_cal.is_erev_yom_tov_sheni(),
        "{} - is_erev_yom_tov_sheni",
        base_message
    );

    assert_eq!(
        rust_cal.is_aseres_yemei_teshuva(),
        java_cal.is_aseres_yemei_teshuva(),
        "{} - is_aseres_yemei_teshuva",
        base_message
    );

    assert_eq!(
        rust_cal.is_pesach(),
        java_cal.is_pesach(),
        "{} - is_pesach",
        base_message
    );

    assert_eq!(
        rust_cal.is_chol_hamoed_pesach(),
        java_cal.is_chol_hamoed_pesach(),
        "{} - is_chol_hamoed_pesach",
        base_message
    );

    assert_eq!(
        rust_cal.is_shavuos(),
        java_cal.is_shavuos(),
        "{} - is_shavuos",
        base_message
    );

    assert_eq!(
        rust_cal.is_rosh_hashana(),
        java_cal.is_rosh_hashana(),
        "{} - is_rosh_hashana",
        base_message
    );

    assert_eq!(
        rust_cal.is_yom_kippur(),
        java_cal.is_yom_kippur(),
        "{} - is_yom_kippur",
        base_message
    );

    assert_eq!(
        rust_cal.is_succos(),
        java_cal.is_succos(),
        "{} - is_succos",
        base_message
    );

    assert_eq!(
        rust_cal.is_hoshana_rabba(),
        java_cal.is_hoshana_rabba(),
        "{} - is_hoshana_rabba",
        base_message
    );

    assert_eq!(
        rust_cal.is_shemini_atzeres(),
        java_cal.is_shemini_atzeres(),
        "{} - is_shemini_atzeres",
        base_message
    );

    assert_eq!(
        rust_cal.is_simchas_torah(),
        java_cal.is_simchas_torah(),
        "{} - is_simchas_torah",
        base_message
    );

    assert_eq!(
        rust_cal.is_chol_hamoed_succos(),
        java_cal.is_chol_hamoed_succos(),
        "{} - is_chol_hamoed_succos",
        base_message
    );

    assert_eq!(
        rust_cal.is_chol_hamoed(),
        java_cal.is_chol_hamoed(),
        "{} - is_chol_hamoed",
        base_message
    );

    assert_eq!(
        rust_cal.is_erev_yom_tov(),
        java_cal.is_erev_yom_tov(),
        "{} - is_erev_yom_tov",
        base_message
    );

    assert_eq!(
        rust_cal.is_rosh_chodesh(),
        java_cal.is_rosh_chodesh(),
        "{} - is_rosh_chodesh",
        base_message
    );

    assert_eq!(
        rust_cal.is_isru_chag(),
        java_cal.is_isru_chag(),
        "{} - is_isru_chag",
        base_message
    );

    assert_eq!(
        rust_cal.is_taanis(),
        java_cal.is_taanis(),
        "{} - is_taanis",
        base_message
    );

    assert_eq!(
        rust_cal.is_taanis_bechoros(),
        java_cal.is_taanis_bechoros(),
        "{} - is_taanis_bechoros",
        base_message
    );

    assert_eq!(
        rust_cal.is_chanukah(),
        java_cal.is_chanukah(),
        "{} - is_chanukah",
        base_message
    );

    assert_eq!(
        rust_cal.is_purim(),
        java_cal.is_purim(),
        "{} - is_purim",
        base_message
    );

    assert_eq!(
        rust_cal.is_tisha_beav(),
        java_cal.is_tisha_beav(),
        "{} - is_tisha_beav",
        base_message
    );

    assert_eq!(
        rust_cal.get_day_of_chanukah(),
        java_cal.get_day_of_chanukah(),
        "{} - get_day_of_chanukah",
        base_message
    );

    assert_eq!(
        rust_cal.get_day_of_omer(),
        java_cal.get_day_of_omer(),
        "{} - get_day_of_omer",
        base_message
    );

    let rust_daf = rust_cal.get_daf_yomi_bavli();
    let java_daf = java_cal.get_daf_yomi_bavli();
    assert_eq!(
        rust_daf, java_daf,
        "{} - daf_yomi_bavli masechta",
        base_message
    );
}

#[test]
fn test_rust_java_jewish_calendar_comparison() {
    let jvm = test_utils::create_jvm();

    for _ in 0..test_utils::DEFAULT_TEST_ITERATIONS {
        let timestamp = test_utils::random_test_timestamp();
        let tz_offset = test_utils::random_test_timestamp() % (24 * 60 * 60 * 1000);

        let mut java_cal = JavaJewishCalendar::from_date(&jvm, timestamp, tz_offset);

        let rust_cal_1 = match JewishCalendar::new(timestamp, tz_offset, false, false) {
            Some(c) => c,
            None => continue,
        };
        java_cal.set_in_israel(false);
        java_cal.set_use_modern_holidays(false);
        compare_calendar_methods(&rust_cal_1, &java_cal, timestamp, tz_offset);

        let rust_cal_2 = match JewishCalendar::new(timestamp, tz_offset, true, false) {
            Some(c) => c,
            None => continue,
        };
        java_cal.set_in_israel(true);
        java_cal.set_use_modern_holidays(false);
        compare_calendar_methods(&rust_cal_2, &java_cal, timestamp, tz_offset);

        let rust_cal_3 = match JewishCalendar::new(timestamp, tz_offset, false, true) {
            Some(c) => c,
            None => continue,
        };
        java_cal.set_in_israel(false);
        java_cal.set_use_modern_holidays(true);
        compare_calendar_methods(&rust_cal_3, &java_cal, timestamp, tz_offset);

        let rust_cal_4 = match JewishCalendar::new(timestamp, tz_offset, true, true) {
            Some(c) => c,
            None => continue,
        };
        java_cal.set_in_israel(true);
        java_cal.set_use_modern_holidays(true);
        compare_calendar_methods(&rust_cal_4, &java_cal, timestamp, tz_offset);
    }
}
