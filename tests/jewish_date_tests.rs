mod test_utils;
use test_utils::*;

mod java;
use java::jewish_date::JavaJewishDate;
use zmanim_core::prelude::{jewish_date::GetMoladTrait, *};

#[test]
fn test_rust_java_jewish_date_comparison() {
    let jvm = test_utils::create_jvm();

    for _ in 0..DEFAULT_TEST_ITERATIONS {
        let timestamp = test_utils::random_test_timestamp();

        let tz_offset = test_utils::random_test_timestamp() % (24 * 60 * 60 * 1000);

        let rust_date = match JewishDate::new(timestamp, tz_offset) {
            Some(date) => date,
            None => continue,
        };

        let java_date = JavaJewishDate::from_date(&jvm, timestamp, tz_offset);

        assert_eq!(
            rust_date.get_jewish_year(),
            java_date.get_jewish_year(),
            "Jewish year mismatch for timestamp: {}, tz_offset: {}, rust: {}, java: {}",
            timestamp,
            tz_offset,
            rust_date.get_jewish_year(),
            java_date.get_jewish_year()
        );

        assert_eq!(
            rust_date.get_jewish_month() as i32,
            java_date.get_jewish_month() as i32,
            "Jewish month mismatch for timestamp: {}, tz_offset: {}, rust: {:?}",
            timestamp,
            tz_offset,
            rust_date.hebrew_date,
        );

        assert_eq!(
            rust_date.get_jewish_day_of_month(),
            java_date.get_jewish_day_of_month(),
            "Jewish day of month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_gregorian_year(),
            java_date.get_gregorian_year(),
            "Gregorian year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_gregorian_month(),
            java_date.get_gregorian_month(),
            "Gregorian month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_gregorian_day_of_month(),
            java_date.get_gregorian_day_of_month(),
            "Gregorian day of month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.is_jewish_leap_year(),
            java_date.is_jewish_leap_year(),
            "Leap year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );
        assert_eq!(
            rust_date.get_days_in_jewish_year(),
            java_date.get_days_in_jewish_year(),
            "Days in Jewish year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_days_in_jewish_month(),
            java_date.get_days_in_jewish_month(),
            "Days in Jewish month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.is_cheshvan_long(),
            java_date.is_cheshvan_long(),
            "Cheshvan long mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.is_kislev_short(),
            java_date.is_kislev_short(),
            "Kislev short mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_cheshvan_kislev_kviah() as i32,
            java_date.get_cheshvan_kislev_kviah() as i32,
            "Cheshvan/Kislev kviah mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_days_since_start_of_jewish_year(),
            java_date.get_days_since_start_of_jewish_year(),
            "Days since start of Jewish year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_chalakim_since_molad_tohu(),
            java_date.get_chalakim_since_molad_tohu(),
            "Chalakim since molad tohu mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_date.get_day_of_week() as i32,
            java_date.get_day_of_week() as i32,
            "Day of week mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        let rust_molad_result = rust_date.get_molad();
        let java_molad_result = GetMoladTrait::get_molad(&java_date);

        assert!(
            rust_molad_result.is_some(),
            "Rust get_molad returned None for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );
        assert!(
            java_molad_result.is_some(),
            "Java get_molad returned None for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        let message = format!("timestamp: {}, tz_offset: {}", timestamp, tz_offset);
        let (rust_molad, rust_molad_data) = rust_molad_result.expect(&message);
        let (java_molad, java_molad_data) = java_molad_result.expect(&message);

        assert_eq!(
            rust_molad.get_jewish_year(),
            java_molad.get_jewish_year(),
            "Molad Jewish year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad.get_jewish_month() as i32,
            java_molad.get_jewish_month() as i32,
            "Molad Jewish month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad.get_jewish_day_of_month(),
            java_molad.get_jewish_day_of_month(),
            "Molad Jewish day of month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad.get_gregorian_year(),
            java_molad.get_gregorian_year(),
            "Molad Gregorian year mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad.get_gregorian_month(),
            java_molad.get_gregorian_month(),
            "Molad Gregorian month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad.get_gregorian_day_of_month(),
            java_molad.get_gregorian_day_of_month(),
            "Molad Gregorian day of month mismatch for timestamp: {}, tz_offset: {}",
            timestamp,
            tz_offset
        );

        assert_eq!(
            rust_molad_data.hours, java_molad_data.hours,
            "Molad hours mismatch for timestamp: {}, tz_offset: {}",
            timestamp, tz_offset
        );

        assert_eq!(
            rust_molad_data.minutes, java_molad_data.minutes,
            "Molad minutes mismatch for timestamp: {}, tz_offset: {}",
            timestamp, tz_offset
        );

        assert_eq!(
            rust_molad_data.chalakim, java_molad_data.chalakim,
            "Molad chalakim mismatch for timestamp: {}, tz_offset: {}",
            timestamp, tz_offset
        );
    }
}
