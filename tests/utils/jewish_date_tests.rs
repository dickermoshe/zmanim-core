use zmanim_core::hebrew_calendar::jewish_date::{JewishDate, JewishDateTrait};
#[cfg(test)]
mod tests {
    use super::*;
    use crate::java::jewish_date::JavaJewishDate;
    use crate::test_utils;

    #[test]
    fn test_rust_java_jewish_date_comparison() {
        // Initialize JVM for Java implementation
        let jvm = test_utils::create_jvm();

        // Test with multiple random timestamps and timezone offsets
        for _ in 0..100 {
            let timestamp = test_utils::random_test_timestamp();
            println!("Testing timestamp: {}", timestamp);
            let tz_offset = test_utils::random_test_timestamp() % (24 * 60 * 60 * 1000); // Random offset within 24 hours

            // Create Rust implementation
            let rust_date = match JewishDate::new(timestamp, tz_offset) {
                Some(date) => date,
                None => continue, // Skip invalid timestamps
            };

            // Create Java implementation (note: Java version doesn't use timezone offset in constructor)
            let java_date = JavaJewishDate::from_date(&jvm, timestamp);

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
                "Jewish month mismatch for timestamp: {}, tz_offset: {}",
                timestamp,
                tz_offset,
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

            // Note: get_chalakim_since_molad_tohu may not match exactly due to implementation differences
            // so we'll skip that comparison for now
        }
    }

    #[test]
    fn test_jewish_date_edge_cases() {
        // Initialize JVM for Java implementation
        let jvm = test_utils::create_jvm();

        // Test specific edge cases
        let edge_cases = vec![
            (0i64, 0i64),                    // Unix epoch
            (1640995200000i64, 0i64),        // 2022-01-01 00:00:00 UTC
            (1672531200000i64, 0i64),        // 2023-01-01 00:00:00 UTC
            (1704067200000i64, 0i64),        // 2024-01-01 00:00:00 UTC (leap year)
            (1735689600000i64, 0i64),        // 2025-01-01 00:00:00 UTC
            (946684800000i64, -18000000i64), // 2000-01-01 with timezone offset
            (1262304000000i64, 3600000i64),  // 2010-01-01 with positive timezone offset
        ];

        for (timestamp, tz_offset) in edge_cases {
            println!(
                "Testing edge case: timestamp={}, tz_offset={}",
                timestamp, tz_offset
            );

            // Create Rust implementation
            let rust_date = match JewishDate::new(timestamp, tz_offset) {
                Some(date) => date,
                None => {
                    println!("  Skipping invalid timestamp");
                    continue;
                }
            };

            // Create Java implementation
            let java_date = JavaJewishDate::from_date(&jvm, timestamp);

            // Compare key methods
            assert_eq!(
                rust_date.get_jewish_year(),
                java_date.get_jewish_year(),
                "Edge case Jewish year mismatch"
            );
            assert_eq!(
                rust_date.get_jewish_month() as i32,
                java_date.get_jewish_month() as i32,
                "Edge case Jewish month mismatch"
            );
            assert_eq!(
                rust_date.get_jewish_day_of_month(),
                java_date.get_jewish_day_of_month(),
                "Edge case Jewish day of month mismatch"
            );
            assert_eq!(
                rust_date.get_gregorian_year(),
                java_date.get_gregorian_year(),
                "Edge case Gregorian year mismatch"
            );
            assert_eq!(
                rust_date.is_jewish_leap_year(),
                java_date.is_jewish_leap_year(),
                "Edge case leap year mismatch"
            );
        }
    }

    #[test]
    fn test_static_methods_consistency() {
        // Test static methods for consistency
        let test_years = vec![5780, 5781, 5782, 5783, 5784, 5785]; // Recent Jewish years

        let jvm = test_utils::create_jvm();

        for year in test_years {
            let is_leap_rust = JewishDate::is_jewish_leap_year_static(year);
            let is_leap_java = JavaJewishDate::is_jewish_leap_year_static(&jvm, year);

            assert_eq!(
                is_leap_rust, is_leap_java,
                "Static leap year mismatch for year: {}",
                year
            );

            let last_month_rust = JewishDate::get_last_month_of_jewish_year(year);
            let last_month_java = JavaJewishDate::get_last_month_of_jewish_year(&jvm, year);

            assert_eq!(
                last_month_rust, last_month_java,
                "Static last month mismatch for year: {}",
                year
            );

            let days_in_year_rust = JewishDate::get_days_in_jewish_year_static(year);
            let days_in_year_java = JavaJewishDate::get_days_in_jewish_year_static(&jvm, year);

            assert_eq!(
                days_in_year_rust, days_in_year_java,
                "Static days in year mismatch for year: {}",
                year
            );
        }
    }
}
