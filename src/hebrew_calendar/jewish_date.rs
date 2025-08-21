use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration;
use icu_calendar::Gregorian;
use icu_calendar::cal::Hebrew;
use icu_calendar::{Date, types::Weekday};

pub enum JewishMonth {
    NISSAN = 1,
    IYAR = 2,
    SIVAN = 3,
    TAMMUZ = 4,
    AV = 5,
    ELUL = 6,
    TISHREI = 7,
    CHESHVAN = 8,
    KISLEV = 9,
    TEVES = 10,
    SHEVAT = 11,
    ADAR = 12,
    ADARII = 13,
}
impl From<JewishMonth> for i32 {
    fn from(month: JewishMonth) -> Self {
        month as i32
    }
}
impl From<i32> for JewishMonth {
    fn from(month: i32) -> Self {
        match month {
            1 => JewishMonth::NISSAN,
            2 => JewishMonth::IYAR,
            3 => JewishMonth::SIVAN,
            4 => JewishMonth::TAMMUZ,
            5 => JewishMonth::AV,
            6 => JewishMonth::ELUL,
            7 => JewishMonth::TISHREI,
            8 => JewishMonth::CHESHVAN,
            9 => JewishMonth::KISLEV,
            10 => JewishMonth::TEVES,
            11 => JewishMonth::SHEVAT,
            12 => JewishMonth::ADAR,
            13 => JewishMonth::ADARII,
            _ => panic!("Invalid Jewish month: {}", month),
        }
    }
}
pub enum YearLengthType {
    CHASERIM = 0,
    KESIDRAN = 1,
    SHELAIMIM = 2,
}
impl From<i32> for YearLengthType {
    fn from(year_length: i32) -> Self {
        match year_length {
            0 => YearLengthType::CHASERIM,
            1 => YearLengthType::KESIDRAN,
            2 => YearLengthType::SHELAIMIM,
            _ => panic!("Invalid year length: {}", year_length),
        }
    }
}
impl From<YearLengthType> for i32 {
    fn from(year_length: YearLengthType) -> Self {
        year_length as i32
    }
}

/// Trait defining the interface for JewishDate operations
pub trait JewishDateTrait {
    /// Get the Jewish year
    fn get_jewish_year(&self) -> i32;

    /// Get the Jewish month (1-based)
    fn get_jewish_month(&self) -> JewishMonth;

    /// Get the Jewish day of month
    fn get_jewish_day_of_month(&self) -> i32;

    /// Get the Gregorian year
    fn get_gregorian_year(&self) -> i32;

    /// Get the Gregorian month (0-based)
    fn get_gregorian_month(&self) -> i32;

    /// Get the Gregorian day of month
    fn get_gregorian_day_of_month(&self) -> i32;

    /// Get the day of week (1 = Sunday, 7 = Saturday)
    fn get_day_of_week(&self) -> Weekday;

    /// Check if it's a Jewish leap year
    fn is_jewish_leap_year(&self) -> bool;

    /// Get the days in the Jewish year
    fn get_days_in_jewish_year(&self) -> i32;

    /// Get the days in the Jewish month
    fn get_days_in_jewish_month(&self) -> i32;

    /// Check if Cheshvan is long
    fn is_cheshvan_long(&self) -> bool;

    /// Check if Kislev is short
    fn is_kislev_short(&self) -> bool;

    /// Get Cheshvan/Kislev kviah
    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;

    /// Get the days since start of Jewish year
    fn get_days_since_start_of_jewish_year(&self) -> i32;

    /// Get the chalakim since molad tohu
    fn get_chalakim_since_molad_tohu(&self) -> i64;

    /// Check if a Jewish year is a leap year (static method)
    fn is_jewish_leap_year_static(year: i32) -> bool;

    /// Get the last month of a Jewish year (static method)
    fn get_last_month_of_jewish_year(year: i32) -> i32;

    /// Get the Jewish calendar elapsed days (static method)
    fn get_jewish_calendar_elapsed_days(year: i32) -> i32;

    /// Get the days in a Jewish year (static method)
    fn get_days_in_jewish_year_static(year: i32) -> i32;

    /// Get the days in a Jewish month (static method)
    fn get_days_in_jewish_month_static(month: i32, year: i32) -> i32;
}

pub struct JewishDate {
    pub hebrew_date: Date<Hebrew>,
    pub gregorian_date: Date<Gregorian>,
}

impl JewishDate {
    pub fn new(timestamp: i64, tz_offset: i64) -> Option<Self> {
        let chrono_date =
            DateTime::from_timestamp_millis(timestamp)? + Duration::milliseconds(tz_offset);
        let year = chrono_date.year();
        let month = chrono_date.month();
        let day = chrono_date.day();
        // very gregorian day spans two jewish dates. This API will consider the 1st date to be the correct one.
        // meaning that a timestamp at 8:00 PM will be considered the previous day's hebrew date.
        // for instance, Thursday, August 21, 2025 11:06:53 PM/1755817613000 is after shkia, so it's technically the the next day,
        // however we don't care. It's we still return 27 Av even though it's technically 28 Av.
        let gregorian_date = Date::try_new_gregorian(year as i32, month as u8, day as u8).ok()?;
        let hebrew_date = gregorian_date.to_calendar(Hebrew);

        Some(Self {
            hebrew_date,
            gregorian_date,
        })
    }
}

impl JewishDateTrait for JewishDate {
    fn get_jewish_year(&self) -> i32 {
        self.hebrew_date.era_year().year
    }

    fn get_jewish_month(&self) -> JewishMonth {
        let month_code = self.hebrew_date.month().standard_code.0;
        match month_code.as_str() {
            "M01" => JewishMonth::TISHREI,
            "M02" => JewishMonth::CHESHVAN,
            "M03" => JewishMonth::KISLEV,
            "M04" => JewishMonth::TEVES,
            "M05" => JewishMonth::SHEVAT,
            "M05L" => JewishMonth::ADAR,   // Adar I in leap years
            "M06" => JewishMonth::ADAR,    // Adar in non-leap years, or Adar II in leap years?
            "M06L" => JewishMonth::ADARII, // Adar II in leap years
            "M07" => JewishMonth::NISSAN,
            "M08" => JewishMonth::IYAR,
            "M09" => JewishMonth::SIVAN,
            "M10" => JewishMonth::TAMMUZ,
            "M11" => JewishMonth::AV,
            "M12" => JewishMonth::ELUL,
            _ => {
                panic!("Unknown Hebrew month code: {}", month_code);
            }
        }
    }

    fn get_jewish_day_of_month(&self) -> i32 {
        self.hebrew_date.day_of_month().0 as i32 - 2
    }

    fn get_gregorian_year(&self) -> i32 {
        self.gregorian_date.era_year().year
    }

    fn get_gregorian_month(&self) -> i32 {
        self.gregorian_date.month().ordinal as i32 - 1 // Convert to 0-based
    }

    fn get_gregorian_day_of_month(&self) -> i32 {
        self.gregorian_date.day_of_month().0 as i32
    }

    fn get_day_of_week(&self) -> Weekday {
        self.hebrew_date.day_of_week()
    }

    fn is_jewish_leap_year(&self) -> bool {
        Self::is_jewish_leap_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_year(&self) -> i32 {
        Self::get_days_in_jewish_year_static(self.get_jewish_year())
    }

    fn get_days_in_jewish_month(&self) -> i32 {
        Self::get_days_in_jewish_month_static(
            self.get_jewish_month().into(),
            self.get_jewish_year(),
        )
    }

    fn is_cheshvan_long(&self) -> bool {
        let year = self.get_jewish_year();
        Self::get_days_in_jewish_year_static(year) == 355
    }

    fn is_kislev_short(&self) -> bool {
        let year = self.get_jewish_year();
        Self::get_days_in_jewish_year_static(year) == 353
    }

    /// Get the kviah of Cheshvan/Kislev
    ///
    /// Returns:
    /// - CHASERIM if both months have 29 days
    /// - KESIDRAN if Cheshvan has 30, Kislev has 29
    /// - SHELAIMIM if both months have 30 days
    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType {
        let year = self.get_jewish_year();
        match Self::get_days_in_jewish_year_static(year) {
            353 => YearLengthType::CHASERIM, // CHASERIM - both months have 29 days
            354 => YearLengthType::KESIDRAN, // KESIDRAN - Cheshvan has 30, Kislev has 29
            355 => YearLengthType::SHELAIMIM, // SHELAIMIM - both months have 30 days
            _ => YearLengthType::KESIDRAN,   // Default to KESIDRAN
        }
    }

    fn get_days_since_start_of_jewish_year(&self) -> i32 {
        // Calculate days from start of Jewish year (Rosh Hashanah)
        let year = self.get_jewish_year();
        let mut days = 0;

        // Add days for each month before current month
        for month in 1..self.get_jewish_month().into() {
            days += Self::get_days_in_jewish_month_static(month, year);
        }

        // Add days in current month
        days += self.get_jewish_day_of_month() - 1;

        days
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        // This is a simplified implementation. In a full implementation,
        // this would calculate the actual chalakim since molad tohu.
        // For now, return a placeholder value.
        0
    }

    fn is_jewish_leap_year_static(year: i32) -> bool {
        // Jewish leap year calculation: every 19 years, add 7 leap years
        // Years 3, 6, 8, 11, 14, 17, 19 in the cycle are leap years
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }

    fn get_last_month_of_jewish_year(year: i32) -> i32 {
        if Self::is_jewish_leap_year_static(year) {
            13 // Adar II
        } else {
            12 // Adar
        }
    }

    fn get_jewish_calendar_elapsed_days(year: i32) -> i32 {
        // This is a simplified implementation. In a full implementation,
        // this would calculate the actual elapsed days in the Jewish calendar.
        // For now, return a placeholder value.
        (year - 1) * 365 + (year - 1) / 19 * 7 // Approximate calculation
    }

    fn get_days_in_jewish_year_static(year: i32) -> i32 {
        // Regular year has 354 days, leap year has 383 days
        // Plus variations for Cheshvan/Kislev
        let base_days = if Self::is_jewish_leap_year_static(year) {
            383
        } else {
            354
        };

        // For simplicity, we'll use a basic calculation
        // In a full implementation, this would account for Cheshvan/Kislev variations
        base_days
    }

    fn get_days_in_jewish_month_static(month: i32, _year: i32) -> i32 {
        match month {
            1 | 2 | 4 | 5 | 7 | 9 | 10 | 11 => 30, // 30-day months
            3 | 6 | 8 | 12 => 29,                  // 29-day months in non-leap years
            13 => 29,                              // Adar II in leap years
            _ => 29,                               // Default
        }
    }
}

// Constants for the Jewish calendar
pub mod constants {
    pub const NISSAN: i32 = 1;
    pub const IYAR: i32 = 2;
    pub const SIVAN: i32 = 3;
    pub const TAMMUZ: i32 = 4;
    pub const AV: i32 = 5;
    pub const ELUL: i32 = 6;
    pub const TISHREI: i32 = 7;
    pub const CHESHVAN: i32 = 8;
    pub const KISLEV: i32 = 9;
    pub const TEVES: i32 = 10;
    pub const SHEVAT: i32 = 11;
    pub const ADAR: i32 = 12;
    pub const ADAR_II: i32 = 13;

    pub const CHALAKIM_PER_MINUTE: i32 = 18;
    pub const CHALAKIM_PER_HOUR: i32 = 1080;
    pub const CHALAKIM_PER_DAY: i32 = 25920;

    pub const CHASERIM: i32 = 0;
    pub const KESIDRAN: i32 = 1;
    pub const SHELAIMIM: i32 = 2;
}
