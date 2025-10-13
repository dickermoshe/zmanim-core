use chrono::DateTime;
use chrono::Datelike;
use chrono::Duration as ChronoDuration;
use icu_calendar::cal::Hebrew;
use icu_calendar::Gregorian;
use icu_calendar::{types::Weekday, Date, DateDuration};
#[cfg(feature = "uniffi")]
use std::sync::Arc;

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
}

pub trait GetMoladTrait {
    fn get_molad(&self) -> Option<(impl JewishDateTrait, MoladData)>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
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

        let gregorian_date = Date::try_new_gregorian(year, month as u8, day as u8).ok()?;
        let hebrew_date = gregorian_date.to_calendar(Hebrew);

        Some(Self {
            hebrew_date,
            gregorian_date,
        })
    }
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
pub fn new_jewish_date(timestamp: i64, tz_offset: i64) -> Option<Arc<JewishDate>> {
    JewishDate::new(timestamp, tz_offset).map(Arc::new)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
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
}

impl GetMoladTrait for JewishDate {
    fn get_molad(&self) -> Option<(impl JewishDateTrait, MoladData)> {
        self._get_molad()
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
    fn _get_molad(&self) -> Option<(JewishDate, MoladData)> {
        let chalakim_since_molad_tohu = self.get_chalakim_since_molad_tohu();
        let abs_date = Self::molad_to_abs_date(chalakim_since_molad_tohu);
        let mut gregorian_date = Self::abs_date_to_date(abs_date)?;
        let conjunction_day = chalakim_since_molad_tohu / date_constants::CHALAKIM_PER_DAY;
        let conjunction_parts =
            chalakim_since_molad_tohu - conjunction_day * date_constants::CHALAKIM_PER_DAY;
        let mut hours = conjunction_parts / date_constants::CHALAKIM_PER_HOUR;
        let adjusted_conjunction_parts =
            conjunction_parts - (hours * date_constants::CHALAKIM_PER_HOUR);
        let minutes = adjusted_conjunction_parts / date_constants::CHALAKIM_PER_MINUTE;
        let chalakim = adjusted_conjunction_parts - (minutes * date_constants::CHALAKIM_PER_MINUTE);
        if hours >= 6 {
            gregorian_date.add(DateDuration::new(0, 0, 0, 1));
        }
        hours = (hours + 18) % 24;
        let molad_date = Self::from_gregorian_date(gregorian_date);
        Some((
            molad_date,
            MoladData {
                hours,
                minutes,
                chalakim,
            },
        ))
    }
    fn get_chalakim_since_molad_tohu_static(year: i32, month: i32) -> i64 {
        let month_of_year = Self::get_jewish_month_of_year(year, month);
        let months_elapsed = (235 * ((year - 1) / 19))
            + (12 * ((year - 1) % 19))
            + ((7 * ((year - 1) % 19) + 1) / 19)
            + (month_of_year - 1);

        date_constants::CHALAKIM_MOLAD_TOHU
            + (date_constants::CHALAKIM_PER_MONTH * months_elapsed as i64)
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

        if month < date_constants::TISHREI.into() {
            for m in
                date_constants::TISHREI.into()..=JewishDate::get_last_month_of_jewish_year(year)
            {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
            for m in date_constants::NISSAN.into()..month {
                elapsed_days += JewishDate::get_days_in_jewish_month_static(m, year);
            }
        } else {
            for m in date_constants::TISHREI.into()..month {
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
        let chalakim_since =
            Self::get_chalakim_since_molad_tohu_static(year, date_constants::TISHREI.into());
        let molad_day = (chalakim_since / date_constants::CHALAKIM_PER_DAY) as i32;
        let molad_parts =
            (chalakim_since - molad_day as i64 * date_constants::CHALAKIM_PER_DAY) as i32;

        Self::add_dechiyos(year, molad_day, molad_parts)
    }
    fn get_days_in_jewish_year_static(year: i32) -> i32 {
        Self::get_jewish_calendar_elapsed_days(year + 1)
            - Self::get_jewish_calendar_elapsed_days(year)
    }
    fn get_days_in_jewish_month_static(month: i32, year: i32) -> i32 {
        match month.try_into().unwrap() {
            date_constants::IYAR
            | date_constants::TAMMUZ
            | date_constants::ELUL
            | date_constants::TEVES => 29,
            date_constants::CHESHVAN => {
                if Self::is_cheshvan_long_static(year) {
                    30
                } else {
                    29
                }
            }
            date_constants::KISLEV => {
                if Self::is_kislev_short_static(year) {
                    29
                } else {
                    30
                }
            }
            date_constants::ADAR => {
                if Self::is_jewish_leap_year_static(year) {
                    30
                } else {
                    29
                }
            }
            date_constants::ADAR_II => 29,
            _ => 30,
        }
    }
    fn molad_to_abs_date(chalakim: i64) -> i64 {
        date_constants::JEWISH_EPOCH + (chalakim / date_constants::CHALAKIM_PER_DAY)
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
    fn abs_date_to_date(abs_date: i64) -> Option<Date<Gregorian>> {
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
        Date::try_new_gregorian(year as i32, month as u8, day_of_month as u8).ok()
    }
}

pub(crate) mod date_constants {
    pub const NISSAN: u8 = 1;
    pub const IYAR: u8 = 2;
    pub const SIVAN: u8 = 3;
    pub const TAMMUZ: u8 = 4;
    #[allow(dead_code)]
    pub const AV: u8 = 5;
    pub const ELUL: u8 = 6;
    pub const TISHREI: u8 = 7;
    pub const CHESHVAN: u8 = 8;
    pub const KISLEV: u8 = 9;
    pub const TEVES: u8 = 10;
    #[allow(dead_code)]
    pub const SHEVAT: u8 = 11;
    pub const ADAR: u8 = 12;
    pub const ADAR_II: u8 = 13;

    pub const CHALAKIM_PER_MINUTE: i64 = 18;
    pub const CHALAKIM_PER_HOUR: i64 = 1080;
    pub const CHALAKIM_PER_DAY: i64 = 25920;
    pub const CHALAKIM_PER_MONTH: i64 = 765433;
    pub const CHALAKIM_MOLAD_TOHU: i64 = 31524;
    pub const JEWISH_EPOCH: i64 = -1373429;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
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
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[repr(u8)]
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
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct MoladData {
    pub hours: i64,
    pub minutes: i64,
    pub chalakim: i64,
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl JewishDate {
    pub fn get_molad_date(&self) -> Option<Arc<JewishDate>> {
        let molad_data = self._get_molad().map(|(molad_date, _)| molad_date);
        molad_data.map(|molad_date| Arc::new(molad_date))
    }
    pub fn get_molad_data(&self) -> Option<Arc<MoladData>> {
        self._get_molad()
            .map(|(_, molad_data)| Arc::new(molad_data))
    }
}

#[cfg(feature = "uniffi")]
#[uniffi::export]
impl MoladData {
    pub fn get_hours(&self) -> i64 {
        self.hours
    }
    pub fn get_minutes(&self) -> i64 {
        self.minutes
    }
    pub fn get_chalakim(&self) -> i64 {
        self.chalakim
    }
}
