use zmanim_core::hebrew_calendar::MoladData;
use zmanim_core::hebrew_calendar::jewish_date::{JewishDate, JewishDateTrait};
#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use super::*;
    use crate::java::jewish_date::JavaJewishDate;
    use crate::test_utils;

    #[test]
    fn test_rust_java_jewish_date_comparison() {
        // Initialize JVM for Java implementation
        let jvm = test_utils::create_jvm();

        // Test with multiple random timestamps and timezone offsets
        for _ in 0..10000 {
            let timestamp = test_utils::random_test_timestamp();

            let tz_offset = test_utils::random_test_timestamp() % (24 * 60 * 60 * 1000); // Random offset within 24 hours

            // Create Rust implementation
            let rust_date = match JewishDate::new(timestamp, tz_offset) {
                Some(date) => date,
                None => continue, // Skip invalid timestamps
            };
            let chrono_date = DateTime::from_timestamp_millis(timestamp + tz_offset).unwrap();

            // Create Java implementation (note: Java version doesn't use timezone offset in constructor)
            let java_date = JavaJewishDate::from_date(&jvm, timestamp, tz_offset);

            // Compare all trait methods
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
        }
    }

    #[test]
    fn test_get_molad_comparison() {
        // Initialize JVM for Java implementation
        let jvm = test_utils::create_jvm();

        // Test with multiple random timestamps and timezone offsets
        for _ in 0..1000 {
            let timestamp = test_utils::random_test_timestamp();
            let tz_offset = test_utils::random_test_timestamp() % (24 * 60 * 60 * 1000);

            // Create Rust implementation
            let rust_date = match JewishDate::new(timestamp, tz_offset) {
                Some(date) => date,
                None => continue, // Skip invalid timestamps
            };

            // Create Java implementation
            let java_date = JavaJewishDate::from_date(&jvm, timestamp, tz_offset);

            // Get molad from both implementations
            let (rust_molad, rust_molad_data) = rust_date.get_molad();
            let (java_molad, java_molad_data) = JewishDateTrait::get_molad(&java_date);

            // // Compare molad dates
            // assert_eq!(
            //     rust_molad.get_jewish_year(),
            //     java_molad.get_jewish_year(),
            //     "Molad Jewish year mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // assert_eq!(
            //     rust_molad.get_jewish_month() as i32,
            //     java_molad.get_jewish_month() as i32,
            //     "Molad Jewish month mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // assert_eq!(
            //     rust_molad.get_jewish_day_of_month(),
            //     java_molad.get_jewish_day_of_month(),
            //     "Molad Jewish day of month mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // assert_eq!(
            //     rust_molad.get_gregorian_year(),
            //     java_molad.get_gregorian_year(),
            //     "Molad Gregorian year mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // assert_eq!(
            //     rust_molad.get_gregorian_month(),
            //     java_molad.get_gregorian_month(),
            //     "Molad Gregorian month mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // assert_eq!(
            //     rust_molad.get_gregorian_day_of_month(),
            //     java_molad.get_gregorian_day_of_month(),
            //     "Molad Gregorian day of month mismatch for timestamp: {}, tz_offset: {}",
            //     timestamp,
            //     tz_offset
            // );

            // Compare molad time data
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
}
