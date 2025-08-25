use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration as ChronoDuration;
use icu_calendar::cal::Hebrew;
use icu_calendar::Gregorian;
use icu_calendar::{types::Weekday, Date, DateDuration};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DayOfWeek {
    Sunday = 1,
    Monday = 2,
    Tuesday = 3,
    Wednesday = 4,
    Thursday = 5,
    Friday = 6,
    Saturday = 7,
}
impl From<i32> for DayOfWeek {
    fn from(day_of_week: i32) -> Self {
        match day_of_week {
            1 => DayOfWeek::Sunday,
            2 => DayOfWeek::Monday,
            3 => DayOfWeek::Tuesday,
            4 => DayOfWeek::Wednesday,
            5 => DayOfWeek::Thursday,
            6 => DayOfWeek::Friday,
            7 => DayOfWeek::Saturday,
            _ => panic!("Invalid day of week: {}", day_of_week),
        }
    }
}
impl From<Weekday> for DayOfWeek {
    fn from(weekday: Weekday) -> Self {
        match weekday {
            Weekday::Sunday => DayOfWeek::Sunday,
            Weekday::Monday => DayOfWeek::Monday,
            Weekday::Tuesday => DayOfWeek::Tuesday,
            Weekday::Wednesday => DayOfWeek::Wednesday,
            Weekday::Thursday => DayOfWeek::Thursday,
            Weekday::Friday => DayOfWeek::Friday,
            Weekday::Saturday => DayOfWeek::Saturday,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MoladData {
    pub hours: i64,
    pub minutes: i64,
    pub chalakim: i64,
}


pub trait JewishDateTrait {
    
    fn get_jewish_year(&self) -> i32;

    
    fn get_jewish_month(&self) -> JewishMonth;

    
    fn get_jewish_day_of_month(&self) -> i32;

    
    fn get_gregorian_year(&self) -> i32;

    
    fn get_gregorian_month(&self) -> i32;

    
    fn get_gregorian_day_of_month(&self) -> i32;

    
    fn get_day_of_week(&self) -> DayOfWeek;

    
    fn is_jewish_leap_year(&self) -> bool;

    
    fn get_days_in_jewish_year(&self) -> i32;

    
    fn get_days_in_jewish_month(&self) -> i32;

    
    fn is_cheshvan_long(&self) -> bool;

    
    fn is_kislev_short(&self) -> bool;

    
    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType;

    
    fn get_days_since_start_of_jewish_year(&self) -> i32;

    
    fn get_chalakim_since_molad_tohu(&self) -> i64;

    
    fn get_molad(&self) -> (impl JewishDateTrait, MoladData);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JewishDate {
    pub hebrew_date: Date<Hebrew>,
    pub gregorian_date: Date<Gregorian>,
}

impl JewishDate {
    pub fn new(timestamp: i64, tz_offset: i64) -> Option<Self> {
        let chrono_date =
            DateTime::from_timestamp_millis(timestamp)? + ChronoDuration::milliseconds(tz_offset);
        let year = chrono_date.year();
        let month = chrono_date.month();
        let day = chrono_date.day();
        
        
        
        
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
        let month_code = self.hebrew_date.month().formatting_code.0;
        match month_code.as_str() {
            "M01" => JewishMonth::TISHREI,
            "M02" => JewishMonth::CHESHVAN,
            "M03" => JewishMonth::KISLEV,
            "M04" => JewishMonth::TEVES,
            "M05" => JewishMonth::SHEVAT,
            "M05L" => JewishMonth::ADAR,   
            "M06" => JewishMonth::ADAR,    
            "M06L" => JewishMonth::ADARII, 
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
        self.hebrew_date.day_of_month().0 as i32
    }

    fn get_gregorian_year(&self) -> i32 {
        self.gregorian_date.era_year().year
    }

    fn get_gregorian_month(&self) -> i32 {
        self.gregorian_date.month().ordinal as i32 - 1 
    }

    fn get_gregorian_day_of_month(&self) -> i32 {
        self.gregorian_date.day_of_month().0 as i32
    }

    fn get_day_of_week(&self) -> DayOfWeek {
        self.hebrew_date.day_of_week().into()
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
        Self::is_cheshvan_long_static(self.get_jewish_year())
    }

    fn is_kislev_short(&self) -> bool {
        Self::is_kislev_short_static(self.get_jewish_year())
    }

    fn get_cheshvan_kislev_kviah(&self) -> YearLengthType {
        let year = self.get_jewish_year();
        if Self::is_cheshvan_long_static(year) && !Self::is_kislev_short_static(year) {
            YearLengthType::SHELAIMIM
        } else if !Self::is_cheshvan_long_static(year) && Self::is_kislev_short_static(year) {
            YearLengthType::CHASERIM
        } else {
            YearLengthType::KESIDRAN
        }
    }

    fn get_days_since_start_of_jewish_year(&self) -> i32 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month() as i32;
        let day = self.get_jewish_day_of_month();
        Self::get_days_since_start_of_jewish_year_static(year, month, day)
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let year = self.get_jewish_year();
        let month = self.get_jewish_month() as i32;
        Self::get_chalakim_since_molad_tohu_static(year, month)
    }

    fn get_molad(&self) -> (impl JewishDateTrait, MoladData) {
        let chalakim_since_molad_tohu = self.get_chalakim_since_molad_tohu();
        let abs_date = Self::molad_to_abs_date(chalakim_since_molad_tohu);
        let mut gregorian_date = Self::abs_date_to_date(abs_date);
        let conjunction_day = chalakim_since_molad_tohu / constants::CHALAKIM_PER_DAY;
        let conjunction_parts =
            chalakim_since_molad_tohu - conjunction_day * constants::CHALAKIM_PER_DAY;
        let mut hours = conjunction_parts / constants::CHALAKIM_PER_HOUR;
        let adjusted_conjunction_parts = conjunction_parts - (hours * constants::CHALAKIM_PER_HOUR);
        let minutes = adjusted_conjunction_parts / constants::CHALAKIM_PER_MINUTE;
        let chalakim = adjusted_conjunction_parts - (minutes * constants::CHALAKIM_PER_MINUTE);
        if hours >= 6 {
            gregorian_date.add(DateDuration::new(0, 0, 0, 1));
        }
        hours = (hours + 18) % 24;
        let molad_date = Self::from_gregorian_date(gregorian_date);
        (
            molad_date,
            MoladData {
                hours,
                minutes,
                chalakim,
            },
        )
    }
}


impl JewishDate {
    fn from_gregorian_date(gregorian_date: Date<Gregorian>) -> Self {
        let hebrew_date = gregorian_date.to_calendar(Hebrew);
        Self {
            hebrew_date,
            gregorian_date,
        }
    }
    fn get_chalakim_since_molad_tohu_static(year: i32, month: i32) -> i64 {
        
        let month_of_year = Self::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19))
            + (12 * ((year - 1) % 19))
            + ((7 * ((year - 1) % 19) + 1) / 19)
            + (month_of_year - 1);

        constants::CHALAKIM_MOLAD_TOHU as i64
            + (constants::CHALAKIM_PER_MONTH * months_elapsed as i64)
    }

    fn get_jewish_month_of_year(year: i32, month: i32) -> i32 {
        let is_leap_year = JewishDate::is_jewish_leap_year_static(year);
        (month + if is_leap_year { 6 } else { 5 }) % if is_leap_year { 13 } else { 12 } + 1
    }

    fn add_dechiyos(year: i32, molad_day: i32, molad_parts: i32) -> i32 {
        let mut rosh_hashana_day = molad_day;

        
        if (molad_parts >= 19440)
            || (((molad_day % 7) == 2)
                && (molad_parts >= 9924)
                && !JewishDate::is_jewish_leap_year_static(year))
            || (((molad_day % 7) == 1)
                && (molad_parts >= 16789)
                && (JewishDate::is_jewish_leap_year_static(year - 1)))
        {
            rosh_hashana_day += 1;
        }

        
        if ((rosh_hashana_day % 7) == 0)
            || ((rosh_hashana_day % 7) == 3)
            || ((rosh_hashana_day % 7) == 5)
        {
            rosh_hashana_day += 1;
        }

        rosh_hashana_day
    }

    fn is_cheshvan_long_static(year: i32) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 5
    }

    fn is_kislev_short_static(year: i32) -> bool {
        JewishDate::get_days_in_jewish_year_static(year) % 10 == 3
    }

    fn get_days_since_start_of_jewish_year_static(year: i32, month: i32, day_of_month: i32) -> i32 {
        let mut elapsed_days = day_of_month;

        if month < constants::TISHREI {
            
            
            for m in constants::TISHREI..=JewishDate::get_last_month_of_jewish_year(year) {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
            for m in constants::NISSAN..month {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
        } else {
            
            for m in constants::TISHREI..month {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
        }

        elapsed_days
    }

    fn is_jewish_leap_year_static(year: i32) -> bool {
        
        
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }
    fn get_last_month_of_jewish_year(year: i32) -> i32 {
        if Self::is_jewish_leap_year_static(year) {
            13 
        } else {
            12 
        }
    }
    pub fn get_jewish_calendar_elapsed_days(year: i32) -> i32 {
        let chalakim_since = Self::get_chalakim_since_molad_tohu_static(year, constants::TISHREI);
        let molad_day = (chalakim_since / constants::CHALAKIM_PER_DAY as i64) as i32;
        let molad_parts =
            (chalakim_since - molad_day as i64 * constants::CHALAKIM_PER_DAY as i64) as i32;

        Self::add_dechiyos(year, molad_day, molad_parts)
    }
    fn get_days_in_jewish_year_static(year: i32) -> i32 {
        Self::get_jewish_calendar_elapsed_days(year + 1)
            - Self::get_jewish_calendar_elapsed_days(year)
    }
    fn get_days_in_jewish_month_static(month: i32, year: i32) -> i32 {
        match month {
            constants::IYAR | constants::TAMMUZ | constants::ELUL | constants::TEVES => 29,
            constants::CHESHVAN => {
                if Self::is_cheshvan_long_static(year) {
                    30
                } else {
                    29
                }
            }
            constants::KISLEV => {
                if Self::is_kislev_short_static(year) {
                    29
                } else {
                    30
                }
            }
            constants::ADAR => {
                if Self::is_jewish_leap_year_static(year) {
                    30
                } else {
                    29
                }
            }
            constants::ADAR_II => 29,
            _ => 30, 
        }
    }
    fn molad_to_abs_date(chalakim: i64) -> i64 {
        return constants::JEWISH_EPOCH + (chalakim / constants::CHALAKIM_PER_DAY as i64);
    }
    fn gregorian_date_to_abs_date(year: i64, month: i64, day_of_month: i64) -> i64 {
        let mut abs_date = day_of_month;
        for m in (1..month).rev() {
            abs_date += Self::get_last_day_of_gregorian_month(m, year);
        }
        abs_date + 365 * (year - 1) + (year - 1) / 4 - (year - 1) / 100 + (year - 1) / 400
    }

    fn get_last_day_of_gregorian_month(month: i64, year: i64) -> i64 {
        match month {
            2 => {
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }
    fn abs_date_to_date(abs_date: i64) -> Date<Gregorian> {
        let mut year: i64 = abs_date / 366; 
        while abs_date >= Self::gregorian_date_to_abs_date(year + 1, 1, 1) {
            year += 1;
        }
        let mut month: i64 = 1;
        while abs_date
            > Self::gregorian_date_to_abs_date(
                year,
                month,
                Self::get_last_day_of_gregorian_month(month, year),
            )
        {
            month += 1;
        }
        let day_of_month: i64 = abs_date - Self::gregorian_date_to_abs_date(year, month, 1) + 1;
        Date::try_new_gregorian(year as i32, month as u8, day_of_month as u8).unwrap()
    }
}


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

    pub const CHALAKIM_PER_MINUTE: i64 = 18;
    pub const CHALAKIM_PER_HOUR: i64 = 1080;
    pub const CHALAKIM_PER_DAY: i64 = 25920;
    pub const CHALAKIM_PER_MONTH: i64 = 765433;
    pub const CHALAKIM_MOLAD_TOHU: i64 = 31524;
    pub const JEWISH_EPOCH: i64 = -1373429;

    pub const CHASERIM: i32 = 0;
    pub const KESIDRAN: i32 = 1;
    pub const SHELAIMIM: i32 = 2;
}
