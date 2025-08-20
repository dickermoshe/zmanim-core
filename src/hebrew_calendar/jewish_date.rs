// Core functionality for Jewish date calculations

/// Jewish date constants - month values
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

/// The Jewish epoch using the RD (Rata Die/Fixed Date or Reingold Dershowitz) day used in Calendrical Calculations.
/// Day 1 is January 1, 0001 of the Gregorian calendar
const JEWISH_EPOCH: i32 = -1373429;

/// The number of chalakim (18) in a minute
#[allow(dead_code)]
const CHALAKIM_PER_MINUTE: i32 = 18;
/// The number of chalakim (1080) in an hour
#[allow(dead_code)]
const CHALAKIM_PER_HOUR: i32 = 1080;
/// The number of chalakim (25,920) in a 24-hour day
const CHALAKIM_PER_DAY: i32 = 25920; // 24 * 1080
/// The number of chalakim in an average Jewish month. A month has 29 days, 12 hours and 793
/// chalakim (44 minutes and 3.3 seconds) for a total of 765,433 chalakim
const CHALAKIM_PER_MONTH: i64 = 765433; // (29 * 24 + 12) * 1080 + 793
/// Days from the beginning of Sunday till molad BaHaRaD. Calculated as 1 day, 5 hours and 204 chalakim =
/// (24 + 5) * 1080 + 204 = 31524
const CHALAKIM_MOLAD_TOHU: i32 = 31524;

/// A short year where both CHESHVAN and KISLEV are 29 days
pub const CHASERIM: i32 = 0;
/// An ordered year where CHESHVAN is 29 days and KISLEV is 30 days
pub const KESIDRAN: i32 = 1;
/// A long year where both CHESHVAN and KISLEV are 30 days
pub const SHELAIMIM: i32 = 2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JewishDate {
    timestamp: i64,
    timezone_offset_millis: i32,
    jewish_month: i32,
    jewish_day: i32,
    jewish_year: i32,
    gregorian_month: i32,
    gregorian_day_of_month: i32,
    gregorian_year: i32,
    day_of_week: i32,
    gregorian_abs_date: i32,
    molad_hours: i32,
    molad_minutes: i32,
    molad_chalakim: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JewishMonth {
    Tishrei = 1,
    Cheshvan = 2,
    Kislev = 3,
    Teves = 4,
    Shevat = 5,
    Adar = 6,
    Nissan = 7,
    Iyar = 8,
    Sivan = 9,
    Tammuz = 10,
    Av = 11,
    Elul = 12,
    AdarII = 13,
}

pub enum JewishYearType {
    Chaserim = 0,
    Kesidran = 1,
    Shelamim = 2,
}

impl JewishDate {
    pub fn new(timestamp: i64, timezone_offset_millis: i32) -> Self {
        Self {
            timestamp,
            timezone_offset_millis,
            jewish_month: 0,
            jewish_day: 0,
            jewish_year: 0,
            gregorian_month: 0,
            gregorian_day_of_month: 0,
            gregorian_year: 0,
            day_of_week: 0,
            gregorian_abs_date: 0,
            molad_hours: 0,
            molad_minutes: 0,
            molad_chalakim: 0,
        }
    }

    /// Returns if the year passed in is a Gregorian leap year
    pub fn is_gregorian_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    /// Returns the number of days in a given month in a given month and year
    pub fn get_last_day_of_gregorian_month(month: i32, year: i32) -> i32 {
        match month {
            2 => {
                if Self::is_gregorian_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Returns if the year is a Jewish leap year. Years 3, 6, 8, 11, 14, 17 and 19 in the 19-year cycle are leap years.
    pub fn is_jewish_leap_year(year: i32) -> bool {
        ((7 * year) + 1) % 19 < 7
    }

    /// Returns the last month of a given Jewish year. This will be 12 on a non leap year or 13 on a leap year.
    pub fn get_last_month_of_jewish_year(year: i32) -> i32 {
        if Self::is_jewish_leap_year(year) {
            ADAR_II
        } else {
            ADAR
        }
    }

    /// Computes the absolute date from a Gregorian date. ND+ER
    pub fn gregorian_date_to_abs_date(year: i32, month: i32, day_of_month: i32) -> i32 {
        let mut abs_date = day_of_month;
        for m in (1..month).rev() {
            abs_date += Self::get_last_day_of_gregorian_month(m, year);
        }
        abs_date
            + 365 * (year - 1) // days in previous years ignoring leap days
            + (year - 1) / 4 // Julian leap days before this year
            - (year - 1) / 100 // minus prior century years
            + (year - 1) / 400 // plus prior years divisible by 400
    }

    /// Returns the number of days elapsed from the Sunday prior to the start of the Jewish calendar
    /// to the mean conjunction of Tishri of the Jewish year.
    pub fn get_jewish_calendar_elapsed_days(year: i32) -> i32 {
        let chalakim_since = Self::get_chalakim_since_molad_tohu(year, TISHREI);
        let molad_day = (chalakim_since / CHALAKIM_PER_DAY as i64) as i32;
        let molad_parts = (chalakim_since - molad_day as i64 * CHALAKIM_PER_DAY as i64) as i32;
        // delay Rosh Hashana for the 4 dechiyos
        Self::add_dechiyos(year, molad_day, molad_parts)
    }

    /// Adds the 4 dechiyos for molad Tishrei
    fn add_dechiyos(year: i32, molad_day: i32, molad_parts: i32) -> i32 {
        let mut rosh_hashana_day = molad_day; // if no dechiyos

        // delay Rosh Hashana for the dechiyos of the Molad - new moon
        // 1 - Molad Zaken, 2- GaTRaD 3- BeTuTaKPaT
        if (molad_parts >= 19440) // Dechiya of Molad Zaken - molad is >= midday (18 hours * 1080 chalakim)
            || (((molad_day % 7) == 2) // start Dechiya of GaTRaD - Ga = is a Tuesday
                && (molad_parts >= 9924) // TRaD = 9 hours, 204 parts or later (9 * 1080 + 204)
                && !Self::is_jewish_leap_year(year)) // of a non-leap year - end Dechiya of GaTRaD
            || (((molad_day % 7) == 1) // start Dechiya of BeTuTaKPaT - Be = is on a Monday
                && (molad_parts >= 16789) // TUTaKPaT part of BeTuTaKPaT = 15 hours, 589 parts or later (15 * 1080 + 589)
                && (Self::is_jewish_leap_year(year - 1)))
        // in a year following a leap year - end Dechiya of BeTuTaKPaT
        {
            rosh_hashana_day += 1; // Then postpone Rosh HaShanah one day
        }

        // start 4th Dechiya - Lo ADU Rosh - Rosh Hashana can't occur on A- sunday, D- Wednesday, U - Friday
        if ((rosh_hashana_day % 7) == 0) // If Rosh HaShanah would occur on Sunday,
            || ((rosh_hashana_day % 7) == 3) // or Wednesday,
            || ((rosh_hashana_day % 7) == 5)
        // or Friday - end 4th Dechiya - Lo ADU Rosh
        {
            rosh_hashana_day = rosh_hashana_day + 1; // Then postpone it one (more) day
        }
        rosh_hashana_day
    }

    /// Returns the number of chalakim (parts - 1080 to the hour) from the original hypothetical Molad Tohu
    /// to the year and month passed in.
    pub fn get_chalakim_since_molad_tohu(year: i32, month: i32) -> i64 {
        // Jewish lunar month = 29 days, 12 hours and 793 chalakim
        // chalakim since Molad Tohu BeHaRaD - 1 day, 5 hours and 204 chalakim
        let month_of_year = Self::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19)) // Months in complete 19-year lunar (Metonic) cycles so far
            + (12 * ((year - 1) % 19)) // Regular months in this cycle
            + ((7 * ((year - 1) % 19) + 1) / 19) // Leap months this cycle
            + (month_of_year - 1); // add elapsed months till the start of the molad of the month
        // return chalakim prior to BeHaRaD + number of chalakim since
        CHALAKIM_MOLAD_TOHU as i64 + (CHALAKIM_PER_MONTH * months_elapsed as i64)
    }

    /// Converts the NISSAN based constants used by this class to numeric month starting from TISHREI.
    /// This is required for molad calculations.
    fn get_jewish_month_of_year(year: i32, month: i32) -> i32 {
        let is_leap_year = Self::is_jewish_leap_year(year);
        (month + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
    }

    /// Returns the number of days for a given Jewish year. ND+ER
    pub fn get_days_in_jewish_year(year: i32) -> i32 {
        Self::get_jewish_calendar_elapsed_days(year + 1)
            - Self::get_jewish_calendar_elapsed_days(year)
    }

    /// Returns if Cheshvan is long in a given Jewish year. The method name is_long is done since in a Kesidran (ordered)
    /// year Cheshvan is short. ND+ER
    pub fn is_cheshvan_long(year: i32) -> bool {
        Self::get_days_in_jewish_year(year) % 10 == 5
    }

    /// Returns if Kislev is short (29 days VS 30 days) in a given Jewish year. The method name is_short is done since in
    /// a Kesidran (ordered) year Kislev is long. ND+ER
    pub fn is_kislev_short(year: i32) -> bool {
        Self::get_days_in_jewish_year(year) % 10 == 3
    }

    /// Returns the number of days of a Jewish month for a given month and year.
    pub fn get_days_in_jewish_month(month: i32, year: i32) -> i32 {
        if (month == IYAR)
            || (month == TAMMUZ)
            || (month == ELUL)
            || ((month == CHESHVAN) && !Self::is_cheshvan_long(year))
            || ((month == KISLEV) && Self::is_kislev_short(year))
            || (month == TEVES)
            || ((month == ADAR) && !Self::is_jewish_leap_year(year))
            || (month == ADAR_II)
        {
            29
        } else {
            30
        }
    }

    /// Returns the absolute date of Jewish date. ND+ER
    pub fn jewish_date_to_abs_date(year: i32, month: i32, day_of_month: i32) -> i32 {
        let elapsed = Self::get_days_since_start_of_jewish_year(year, month, day_of_month);
        // add elapsed days this year + Days in prior years + Days elapsed before absolute year 1
        elapsed + Self::get_jewish_calendar_elapsed_days(year) + JEWISH_EPOCH
    }

    /// Returns the number of days from Rosh Hashana of the date passed in, to the full date passed in.
    pub fn get_days_since_start_of_jewish_year(year: i32, month: i32, day_of_month: i32) -> i32 {
        let mut elapsed_days = day_of_month;
        // Before Tishrei (from Nissan to Tishrei), add days in prior months
        if month < TISHREI {
            // this year before and after Nissan.
            for m in TISHREI..=Self::get_last_month_of_jewish_year(year) {
                elapsed_days += Self::get_days_in_jewish_month(m, year);
            }
            for m in NISSAN..month {
                elapsed_days += Self::get_days_in_jewish_month(m, year);
            }
        } else {
            // Add days in prior months this year
            for m in TISHREI..month {
                elapsed_days += Self::get_days_in_jewish_month(m, year);
            }
        }
        elapsed_days
    }

    /// Returns the number of days from the Jewish epoch from the number of chalakim from the epoch passed in.
    pub fn molad_to_abs_date(chalakim: i64) -> i32 {
        (chalakim / CHALAKIM_PER_DAY as i64) as i32 + JEWISH_EPOCH
    }

    /// Validates the components of a Jewish date for validity.
    pub fn validate_jewish_date(
        year: i32,
        month: i32,
        day_of_month: i32,
        hours: i32,
        minutes: i32,
        chalakim: i32,
    ) -> Result<(), &'static str> {
        if month < NISSAN || month > Self::get_last_month_of_jewish_year(year) {
            return Err("The Jewish month has to be between 1 and 12 (or 13 on a leap year)");
        }
        if day_of_month < 1 || day_of_month > 30 {
            return Err("The Jewish day of month can't be < 1 or > 30");
        }
        // reject dates prior to 18 Teves, 3761 (1/1/1 AD)
        if (year < 3761)
            || (year == 3761 && (month >= TISHREI && month < TEVES))
            || (year == 3761 && month == TEVES && day_of_month < 18)
        {
            return Err("A Jewish date earlier than 18 Teves, 3761 (1/1/1 Gregorian) can't be set");
        }
        if hours < 0 || hours > 23 {
            return Err("Hours < 0 or > 23 can't be set");
        }
        if minutes < 0 || minutes > 59 {
            return Err("Minutes < 0 or > 59 can't be set");
        }
        if chalakim < 0 || chalakim > 17 {
            return Err("Chalakim/parts < 0 or > 17 can't be set");
        }
        Ok(())
    }

    /// Validates the components of a Gregorian date for validity.
    pub fn validate_gregorian_date(
        year: i32,
        month: i32,
        day_of_month: i32,
    ) -> Result<(), &'static str> {
        if year < 1 {
            return Err("Years < 1 can't be calculated");
        }
        if month > 11 || month < 0 {
            return Err("The Gregorian month has to be between 0 - 11");
        }
        if day_of_month <= 0 {
            return Err("The day of month can't be less than 1");
        }
        Ok(())
    }
}
