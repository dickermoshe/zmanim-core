use chrono::{DateTime, NaiveDate, NaiveDateTime};
use heapless::Vec;

use crate::hebrew_calendar::{parsha::*, YomiCalculator};

use super::daf::Daf;
use super::jewish_date::{constants, DayOfWeek, JewishDate, JewishDateTrait, JewishMonth};
// removed unused calendar imports

/// Trait defining the interface for JewishCalendar operations
pub trait JewishCalendarTrait {
    /// Get the Yom Tov index for the current date
    fn get_yom_tov_index(&self) -> Option<Holiday>;

    /// Check if the current date is a Yom Tov (major holiday)
    fn is_yom_tov(&self) -> bool;

    /// Check if the current date is Yom Tov where work is prohibited
    fn is_yom_tov_assur_bemelacha(&self) -> bool;

    /// Check if the current date is a day when work is prohibited
    fn is_assur_bemelacha(&self) -> bool;

    /// Check if candle lighting is required (Friday or Erev Yom Tov)
    fn has_candle_lighting(&self) -> bool;

    /// Check if tomorrow is Shabbos or Yom Tov
    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool;

    /// Check if it's Erev Yom Tov Sheni (second day of Yom Tov outside Israel)
    fn is_erev_yom_tov_sheni(&self) -> bool;

    /// Check if it's Aseres Yemei Teshuva (Ten Days of Repentance)
    fn is_aseres_yemei_teshuva(&self) -> bool;

    /// Check if it's Pesach
    fn is_pesach(&self) -> bool;

    /// Check if it's Chol Hamoed Pesach
    fn is_chol_hamoed_pesach(&self) -> bool;

    /// Check if it's Shavuos
    fn is_shavuos(&self) -> bool;

    /// Check if it's Rosh Hashana
    fn is_rosh_hashana(&self) -> bool;

    /// Check if it's Yom Kippur
    fn is_yom_kippur(&self) -> bool;

    /// Check if it's Succos
    fn is_succos(&self) -> bool;

    /// Check if it's Hoshana Rabba
    fn is_hoshana_rabba(&self) -> bool;

    /// Check if it's Shemini Atzeres
    fn is_shemini_atzeres(&self) -> bool;

    /// Check if it's Simchas Torah
    fn is_simchas_torah(&self) -> bool;

    /// Check if it's Chol Hamoed Succos
    fn is_chol_hamoed_succos(&self) -> bool;

    /// Check if it's Chol Hamoed (any)
    fn is_chol_hamoed(&self) -> bool;

    /// Check if it's Erev Yom Tov
    fn is_erev_yom_tov(&self) -> bool;

    /// Check if it's Rosh Chodesh
    fn is_rosh_chodesh(&self) -> bool;

    /// Check if it's Isru Chag
    fn is_isru_chag(&self) -> bool;

    /// Check if it's a Taanis (fast day)
    fn is_taanis(&self) -> bool;

    /// Check if it's Taanis Bechoros
    fn is_taanis_bechoros(&self) -> bool;

    /// Get the day of Chanukah (0 if not Chanukah)
    fn get_day_of_chanukah(&self) -> i32;

    /// Check if it's Chanukah
    fn is_chanukah(&self) -> bool;

    /// Check if it's Purim
    fn is_purim(&self) -> bool;

    /// Get the day of Omer (0 if not during Sefiras HaOmer)
    fn get_day_of_omer(&self) -> i32;

    /// Check if it's Tisha B'Av
    fn is_tisha_beav(&self) -> bool;

    /// Get the Daf Yomi Bavli
    fn get_daf_yomi_bavli(&self) -> Option<Daf>;

    // /// Get the Daf Yomi Yerushalmi
    // fn get_daf_yomi_yerushalmi(&self) -> Option<Daf>;

    /// Get the Parsha for this Shabbos
    fn get_parshah(&self) -> Parsha;
}

/// Trait for configuration methods
pub trait JewishCalendarConfig {
    /// Get whether this calendar is for Israel
    fn get_in_israel(&self) -> bool;

    /// Get whether modern holidays are being used
    fn get_use_modern_holidays(&self) -> bool;
}

/// JewishCalendar extends JewishDate with holiday and calendar-specific functionality
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JewishCalendar {
    pub jewish_date: JewishDate,
    pub in_israel: bool,
    pub use_modern_holidays: bool,
}

/// Enum of holiday/fast indices corresponding to Java constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Holiday {
    EREV_PESACH = 0,
    PESACH = 1,
    CHOL_HAMOED_PESACH = 2,
    PESACH_SHENI = 3,
    EREV_SHAVUOS = 4,
    SHAVUOS = 5,
    SEVENTEEN_OF_TAMMUZ = 6,
    TISHA_BEAV = 7,
    TU_BEAV = 8,
    EREV_ROSH_HASHANA = 9,
    ROSH_HASHANA = 10,
    FAST_OF_GEDALYAH = 11,
    EREV_YOM_KIPPUR = 12,
    YOM_KIPPUR = 13,
    EREV_SUCCOS = 14,
    SUCCOS = 15,
    CHOL_HAMOED_SUCCOS = 16,
    HOSHANA_RABBA = 17,
    SHEMINI_ATZERES = 18,
    SIMCHAS_TORAH = 19,
    CHANUKAH = 21,
    TENTH_OF_TEVES = 22,
    TU_BESHVAT = 23,
    FAST_OF_ESTHER = 24,
    PURIM = 25,
    SHUSHAN_PURIM = 26,
    PURIM_KATAN = 27,
    ROSH_CHODESH = 28,
    YOM_HASHOAH = 29,
    YOM_HAZIKARON = 30,
    YOM_HAATZMAUT = 31,
    YOM_YERUSHALAYIM = 32,
    LAG_BAOMER = 33,
    SHUSHAN_PURIM_KATAN = 34,
    ISRU_CHAG = 35,
    YOM_KIPPUR_KATAN = 36,
    BEHAB = 37,
}

impl Holiday {
    pub fn from_index(value: i32) -> Self {
        match value {
            0 => Holiday::EREV_PESACH,
            1 => Holiday::PESACH,
            2 => Holiday::CHOL_HAMOED_PESACH,
            3 => Holiday::PESACH_SHENI,
            4 => Holiday::EREV_SHAVUOS,
            5 => Holiday::SHAVUOS,
            6 => Holiday::SEVENTEEN_OF_TAMMUZ,
            7 => Holiday::TISHA_BEAV,
            8 => Holiday::TU_BEAV,
            9 => Holiday::EREV_ROSH_HASHANA,
            10 => Holiday::ROSH_HASHANA,
            11 => Holiday::FAST_OF_GEDALYAH,
            12 => Holiday::EREV_YOM_KIPPUR,
            13 => Holiday::YOM_KIPPUR,
            14 => Holiday::EREV_SUCCOS,
            15 => Holiday::SUCCOS,
            16 => Holiday::CHOL_HAMOED_SUCCOS,
            17 => Holiday::HOSHANA_RABBA,
            18 => Holiday::SHEMINI_ATZERES,
            19 => Holiday::SIMCHAS_TORAH,
            21 => Holiday::CHANUKAH,
            22 => Holiday::TENTH_OF_TEVES,
            23 => Holiday::TU_BESHVAT,
            24 => Holiday::FAST_OF_ESTHER,
            25 => Holiday::PURIM,
            26 => Holiday::SHUSHAN_PURIM,
            27 => Holiday::PURIM_KATAN,
            28 => Holiday::ROSH_CHODESH,
            29 => Holiday::YOM_HASHOAH,
            30 => Holiday::YOM_HAZIKARON,
            31 => Holiday::YOM_HAATZMAUT,
            32 => Holiday::YOM_YERUSHALAYIM,
            33 => Holiday::LAG_BAOMER,
            34 => Holiday::SHUSHAN_PURIM_KATAN,
            35 => Holiday::ISRU_CHAG,
            36 => Holiday::YOM_KIPPUR_KATAN,
            37 => Holiday::BEHAB,
            _ => panic!("Invalid holiday index: {}", value),
        }
    }
}

impl JewishCalendar {
    /// Create a new JewishCalendar with a specific timestamp and timezone offset
    pub fn new_with_timestamp(
        timestamp: i64,
        tz_offset: i64,
        in_israel: bool,
        use_modern_holidays: bool,
    ) -> Option<Self> {
        Some(Self {
            jewish_date: JewishDate::new(timestamp, tz_offset)?,
            in_israel: in_israel,
            use_modern_holidays: use_modern_holidays,
        })
    }

    /// Get the underlying JewishDate
    pub fn get_jewish_date(&self) -> &JewishDate {
        &self.jewish_date
    }
}

impl JewishCalendar {
    /// Check if the current date is a Yom Tov (major holiday)
    pub fn is_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        if ((self.is_erev_yom_tov()
            && !(holiday_index == Some(Holiday::HOSHANA_RABBA)
                || holiday_index == Some(Holiday::CHOL_HAMOED_PESACH)))
            || (self.is_taanis() && holiday_index != Some(Holiday::YOM_KIPPUR))
            || holiday_index == Some(Holiday::ISRU_CHAG))
        {
            return false;
        }
        holiday_index.is_some()
    }

    /// Check if the current date is Yom Tov where work is prohibited
    pub fn is_yom_tov_assur_bemelacha(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::PESACH)
                | Some(Holiday::SHAVUOS)
                | Some(Holiday::SUCCOS)
                | Some(Holiday::SHEMINI_ATZERES)
                | Some(Holiday::SIMCHAS_TORAH)
                | Some(Holiday::ROSH_HASHANA)
                | Some(Holiday::YOM_KIPPUR)
        )
    }

    /// Check if the current date is a day when work is prohibited
    pub fn is_assur_bemelacha(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Saturday
            || self.is_yom_tov_assur_bemelacha()
    }

    /// Check if candle lighting is required (Friday or Erev Yom Tov)
    pub fn has_candle_lighting(&self) -> bool {
        self.is_tomorrow_shabbos_or_yom_tov()
    }

    /// Check if tomorrow is Shabbos or Yom Tov
    pub fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Friday
            || self.is_erev_yom_tov()
            || self.is_erev_yom_tov_sheni()
    }

    /// Check if it's Erev Yom Tov Sheni (second day of Yom Tov outside Israel)
    pub fn is_erev_yom_tov_sheni(&self) -> bool {
        let month = self.jewish_date.get_jewish_month();
        let day = self.jewish_date.get_jewish_day_of_month();
        if month == JewishMonth::TISHREI && (day == 1) {
            return true;
        }
        if !self.in_israel {
            if month == JewishMonth::NISSAN && (day == 15 || day == 21) {
                return true;
            }
            if month == JewishMonth::TISHREI && (day == 15 || day == 22) {
                return true;
            }
            if month == JewishMonth::SIVAN && day == 6 {
                return true;
            }
        }
        return false;
    }

    /// Get the Parsha year type (0-16)
    fn get_parsha_year_type(&self) -> Option<i32> {
        let rosh_hashana_day_of_week =
            (JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year()) + 1)
                % 7;
        let rosh_hashana_day_of_week = if rosh_hashana_day_of_week == 0 {
            7
        } else {
            rosh_hashana_day_of_week
        };

        if self.jewish_date.is_jewish_leap_year() {
            match rosh_hashana_day_of_week {
                2 => {
                    // Monday
                    if self.jewish_date.is_kislev_short() {
                        if self.in_israel {
                            Some(14)
                        } else {
                            Some(6)
                        }
                    } else if self.jewish_date.is_cheshvan_long() {
                        if self.in_israel {
                            Some(15)
                        } else {
                            Some(7)
                        }
                    } else {
                        None
                    }
                }
                3 => {
                    // Tuesday
                    if self.in_israel {
                        Some(15)
                    } else {
                        Some(7)
                    }
                }
                5 => {
                    // Thursday
                    if self.jewish_date.is_kislev_short() {
                        Some(8)
                    } else if self.jewish_date.is_cheshvan_long() {
                        Some(9)
                    } else {
                        None
                    }
                }
                7 => {
                    // Saturday
                    if self.jewish_date.is_kislev_short() {
                        Some(10)
                    } else if self.jewish_date.is_cheshvan_long() {
                        if self.in_israel {
                            Some(16)
                        } else {
                            Some(11)
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            match rosh_hashana_day_of_week {
                2 => {
                    // Monday
                    if self.jewish_date.is_kislev_short() {
                        Some(0)
                    } else if self.jewish_date.is_cheshvan_long() {
                        if self.in_israel {
                            Some(12)
                        } else {
                            Some(1)
                        }
                    } else {
                        None
                    }
                }
                3 => {
                    // Tuesday
                    if self.in_israel {
                        Some(12)
                    } else {
                        Some(1)
                    }
                }
                5 => {
                    // Thursday
                    if self.jewish_date.is_cheshvan_long() {
                        Some(3)
                    } else if !self.jewish_date.is_kislev_short() {
                        if self.in_israel {
                            Some(13)
                        } else {
                            Some(2)
                        }
                    } else {
                        None
                    }
                }
                7 => {
                    // Saturday
                    if self.jewish_date.is_kislev_short() {
                        Some(4)
                    } else if self.jewish_date.is_cheshvan_long() {
                        Some(5)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

// Trait implementations for JewishCalendar
impl JewishCalendarTrait for JewishCalendar {
    fn get_yom_tov_index(&self) -> Option<Holiday> {
        let day = self.jewish_date.get_jewish_day_of_month();
        let day_of_week = self.jewish_date.get_day_of_week();
        let month = self.jewish_date.get_jewish_month();

        match month {
            // Nissan
            JewishMonth::NISSAN => {
                if day == 14 {
                    return Some(Holiday::EREV_PESACH);
                }
                if day == 15 || day == 21 || (!self.in_israel && (day == 16 || day == 22)) {
                    return Some(Holiday::PESACH);
                }
                if day >= 17 && day <= 20 || day == 16 {
                    return Some(Holiday::CHOL_HAMOED_PESACH);
                }
                if day == 22 || day == 23 && !self.in_israel {
                    return Some(Holiday::ISRU_CHAG);
                }
                if self.use_modern_holidays {
                    if (day == 26 && day_of_week == DayOfWeek::Thursday)
                        || (day == 28 && day_of_week == DayOfWeek::Monday)
                        || (day == 27
                            && day_of_week != DayOfWeek::Sunday
                            && day_of_week != DayOfWeek::Friday)
                    {
                        // Wednesday
                        return Some(Holiday::YOM_HASHOAH);
                    }
                }
            }
            // Iyar
            JewishMonth::IYAR => {
                if self.use_modern_holidays {
                    if (day == 4 && day_of_week == DayOfWeek::Tuesday)
                        || ((day == 3 || day == 2) && day_of_week == DayOfWeek::Wednesday)
                        || (day == 5 && day_of_week == DayOfWeek::Monday)
                    {
                        // Monday
                        return Some(Holiday::YOM_HAZIKARON);
                    }
                    if (day == 5 && day_of_week == DayOfWeek::Wednesday)
                        || ((day == 4 || day == 3) && day_of_week == DayOfWeek::Thursday)
                        || (day == 6 && day_of_week == DayOfWeek::Tuesday)
                    {
                        // Tuesday
                        return Some(Holiday::YOM_HAATZMAUT);
                    }
                }
                if day == 14 {
                    return Some(Holiday::PESACH_SHENI);
                }
                if day == 18 {
                    return Some(Holiday::LAG_BAOMER);
                }
                if self.use_modern_holidays && day == 28 {
                    return Some(Holiday::YOM_YERUSHALAYIM);
                }
            }
            // Sivan
            JewishMonth::SIVAN => {
                if day == 5 {
                    return Some(Holiday::EREV_SHAVUOS);
                }
                if day == 6 || (day == 7 && !self.in_israel) {
                    return Some(Holiday::SHAVUOS);
                }
                if day == 7 || day == 8 && !self.in_israel {
                    return Some(Holiday::ISRU_CHAG);
                }
            }
            // Tamuz
            JewishMonth::TAMMUZ => {
                if (day == 17 && day_of_week != DayOfWeek::Saturday)
                    || (day == 18 && day_of_week == DayOfWeek::Sunday)
                {
                    // Sunday
                    return Some(Holiday::SEVENTEEN_OF_TAMMUZ);
                }
            }
            // Av
            JewishMonth::AV => {
                if (day_of_week == DayOfWeek::Sunday && day == 10)
                    || (day_of_week != DayOfWeek::Saturday && day == 9)
                {
                    // Not Shabbos
                    return Some(Holiday::TISHA_BEAV);
                }
                if day == 15 {
                    return Some(Holiday::TU_BEAV);
                }
            }
            // Elul
            JewishMonth::ELUL => {
                if day == 29 {
                    return Some(Holiday::EREV_ROSH_HASHANA);
                }
            }
            // Tishrei
            JewishMonth::TISHREI => {
                if day == 1 || day == 2 {
                    return Some(Holiday::ROSH_HASHANA);
                }
                if (day == 3 && day_of_week != DayOfWeek::Saturday)
                    || (day == 4 && day_of_week == DayOfWeek::Sunday)
                {
                    return Some(Holiday::FAST_OF_GEDALYAH);
                }
                if day == 9 {
                    return Some(Holiday::EREV_YOM_KIPPUR);
                }
                if day == 10 {
                    return Some(Holiday::YOM_KIPPUR);
                }
                if day == 14 {
                    return Some(Holiday::EREV_SUCCOS);
                }
                if day == 15 || (day == 16 && !self.in_israel) {
                    return Some(Holiday::SUCCOS);
                }
                if day >= 16 && day <= 20 {
                    return Some(Holiday::CHOL_HAMOED_SUCCOS);
                }
                if day == 21 {
                    return Some(Holiday::HOSHANA_RABBA);
                }
                if day == 22 {
                    return Some(Holiday::SHEMINI_ATZERES);
                }
                if day == 23 && !self.in_israel {
                    return Some(Holiday::SIMCHAS_TORAH);
                }
                if day == 23 || day == 24 && !self.in_israel {
                    return Some(Holiday::ISRU_CHAG);
                }
            }
            // Kislev
            JewishMonth::KISLEV => {
                if day >= 25 {
                    return Some(Holiday::CHANUKAH);
                }
            }
            // Teves
            JewishMonth::TEVES => {
                if day == 1 || day == 2 || (day == 3 && self.jewish_date.is_kislev_short()) {
                    return Some(Holiday::CHANUKAH);
                }
                if day == 10 {
                    return Some(Holiday::TENTH_OF_TEVES);
                }
            }
            // Shevat
            JewishMonth::SHEVAT => {
                if day == 15 {
                    return Some(Holiday::TU_BESHVAT);
                }
            }
            // Adar (non-leap year)
            JewishMonth::ADAR => {
                if !self.jewish_date.is_jewish_leap_year() {
                    if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                        || (day == 13
                            && !(day_of_week == DayOfWeek::Friday
                                || day_of_week == DayOfWeek::Saturday))
                    {
                        // Not Fri/Sat
                        return Some(Holiday::FAST_OF_ESTHER);
                    }
                    if day == 14 {
                        return Some(Holiday::PURIM);
                    }
                    if day == 15 {
                        return Some(Holiday::SHUSHAN_PURIM);
                    }
                } else {
                    if day == 14 {
                        return Some(Holiday::PURIM_KATAN);
                    }
                    if day == 15 {
                        return Some(Holiday::SHUSHAN_PURIM_KATAN);
                    }
                }
            }
            // Adar II (leap year)
            JewishMonth::ADARII => {
                if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                    || (day == 13
                        && !(day_of_week == DayOfWeek::Friday
                            || day_of_week == DayOfWeek::Saturday))
                {
                    // Not Fri/Sat
                    return Some(Holiday::FAST_OF_ESTHER);
                }
                if day == 14 {
                    return Some(Holiday::PURIM);
                }
                if day == 15 {
                    return Some(Holiday::SHUSHAN_PURIM);
                }
            }
            _ => {}
        }

        None
    }

    fn is_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        if self.is_erev_yom_tov()
            && !matches!(
                holiday_index,
                Some(Holiday::HOSHANA_RABBA) | Some(Holiday::CHOL_HAMOED_PESACH)
            )
        {
            return false;
        }
        if self.is_taanis() && holiday_index != Some(Holiday::YOM_KIPPUR) {
            return false;
        }
        if holiday_index == Some(Holiday::ISRU_CHAG) {
            return false;
        }
        holiday_index != None
    }

    fn is_yom_tov_assur_bemelacha(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::PESACH)
                | Some(Holiday::SHAVUOS)
                | Some(Holiday::SUCCOS)
                | Some(Holiday::SHEMINI_ATZERES)
                | Some(Holiday::SIMCHAS_TORAH)
                | Some(Holiday::ROSH_HASHANA)
                | Some(Holiday::YOM_KIPPUR)
        )
    }

    fn is_assur_bemelacha(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Saturday
            || self.is_yom_tov_assur_bemelacha()
    }

    fn has_candle_lighting(&self) -> bool {
        self.is_tomorrow_shabbos_or_yom_tov()
    }

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Friday
            || self.is_erev_yom_tov()
            || self.is_erev_yom_tov_sheni()
    }

    fn is_erev_yom_tov_sheni(&self) -> bool {
        if self.in_israel {
            return false;
        }

        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

        (month == constants::TISHREI && (day == 1))
            || (!self.in_israel
                && ((month == constants::NISSAN && (day == 15 || day == 21))
                    || (month == constants::TISHREI && (day == 15 || day == 22))
                    || (month == constants::SIVAN && day == 6)))
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();
        month == constants::TISHREI && day <= 10
    }

    fn is_pesach(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::PESACH) | Some(Holiday::CHOL_HAMOED_PESACH)
        )
    }

    fn is_chol_hamoed_pesach(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::CHOL_HAMOED_PESACH)
    }

    fn is_shavuos(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::SHAVUOS)
    }

    fn is_rosh_hashana(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::ROSH_HASHANA)
    }

    fn is_yom_kippur(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::YOM_KIPPUR)
    }

    fn is_succos(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::SUCCOS)
                | Some(Holiday::CHOL_HAMOED_SUCCOS)
                | Some(Holiday::HOSHANA_RABBA)
        )
    }

    fn is_hoshana_rabba(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::HOSHANA_RABBA)
    }

    fn is_shemini_atzeres(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::SHEMINI_ATZERES)
    }

    fn is_simchas_torah(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::SIMCHAS_TORAH)
    }

    fn is_chol_hamoed_succos(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::CHOL_HAMOED_SUCCOS) | Some(Holiday::HOSHANA_RABBA)
        )
    }

    fn is_chol_hamoed(&self) -> bool {
        self.is_chol_hamoed_pesach() || self.is_chol_hamoed_succos()
    }

    fn is_erev_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        holiday_index == Some(Holiday::EREV_PESACH)
            || holiday_index == Some(Holiday::EREV_SHAVUOS)
            || holiday_index == Some(Holiday::EREV_ROSH_HASHANA)
            || holiday_index == Some(Holiday::EREV_YOM_KIPPUR)
            || holiday_index == Some(Holiday::EREV_SUCCOS)
            || holiday_index == Some(Holiday::HOSHANA_RABBA)
            || (holiday_index == Some(Holiday::CHOL_HAMOED_PESACH)
                && self.jewish_date.get_jewish_day_of_month() == 20)
    }

    fn is_rosh_chodesh(&self) -> bool {
        let day = self.jewish_date.get_jewish_day_of_month();
        let month = self.jewish_date.get_jewish_month() as i32;
        (day == 1 && month != constants::TISHREI) || day == 30
    }

    fn is_isru_chag(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::ISRU_CHAG)
    }

    fn is_taanis(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::SEVENTEEN_OF_TAMMUZ)
                | Some(Holiday::TISHA_BEAV)
                | Some(Holiday::YOM_KIPPUR)
                | Some(Holiday::FAST_OF_GEDALYAH)
                | Some(Holiday::TENTH_OF_TEVES)
                | Some(Holiday::FAST_OF_ESTHER)
        )
    }

    fn is_taanis_bechoros(&self) -> bool {
        let day = self.jewish_date.get_jewish_day_of_month();
        let day_of_week = self.jewish_date.get_day_of_week() as i32;
        let month = self.jewish_date.get_jewish_month() as i32;

        month == constants::NISSAN
            && (
                (day == 14 && day_of_week != 7) || // Not Shabbos
            (day == 12 && day_of_week == 5)
                // Thursday
            )
    }

    fn get_day_of_chanukah(&self) -> i32 {
        if !self.is_chanukah() {
            return -1;
        }

        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

        if month == constants::KISLEV {
            day - 24
        } else {
            // Teves
            if self.jewish_date.is_kislev_short() {
                day + 5
            } else {
                day + 6
            }
        }
    }

    fn is_chanukah(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::CHANUKAH)
    }

    fn is_purim(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        matches!(
            holiday_index,
            Some(Holiday::PURIM) | Some(Holiday::SHUSHAN_PURIM)
        )
    }

    fn get_day_of_omer(&self) -> i32 {
        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

        if month == constants::NISSAN && day >= 16 {
            day - 15
        } else if month == constants::IYAR {
            day + 15
        } else if month == constants::SIVAN && day < 6 {
            day + 44
        } else {
            -1
        }
    }

    fn is_tisha_beav(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::TISHA_BEAV)
    }

    fn get_daf_yomi_bavli(&self) -> Option<Daf> {
        let day = self.jewish_date.gregorian_date.day_of_month().0;
        let month = self.jewish_date.gregorian_date.month().ordinal;
        let year = self
            .jewish_date
            .gregorian_date
            .year()
            .era_year_or_related_iso();
        let timestamp = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
            .timestamp_millis();
        YomiCalculator::get_daf_yomi_bavli(timestamp)
    }

    fn get_parshah(&self) -> Parsha {
        if self.jewish_date.get_day_of_week() != DayOfWeek::Saturday {
            return Parsha::NONE;
        }

        let year_type = self.get_parsha_year_type();
        if year_type.is_none() {
            return Parsha::NONE;
        }

        let rosh_hashana_day_of_week =
            JewishDate::get_jewish_calendar_elapsed_days(self.jewish_date.get_jewish_year()) % 7;
        let day = rosh_hashana_day_of_week + self.jewish_date.get_days_since_start_of_jewish_year();
        match year_type {
            Some(0) => parsha_list_0[(day / 7) as usize],
            Some(1) => parsha_list_1[(day / 7) as usize],
            Some(2) => parsha_list_2[(day / 7) as usize],
            Some(3) => parsha_list_3[(day / 7) as usize],
            Some(4) => parsha_list_4[(day / 7) as usize],
            Some(5) => parsha_list_5[(day / 7) as usize],
            Some(6) => parsha_list_6[(day / 7) as usize],
            Some(7) => parsha_list_7[(day / 7) as usize],
            Some(8) => parsha_list_8[(day / 7) as usize],
            Some(9) => parsha_list_9[(day / 7) as usize],
            Some(10) => parsha_list_10[(day / 7) as usize],
            Some(11) => parsha_list_11[(day / 7) as usize],
            Some(12) => parsha_list_12[(day / 7) as usize],
            Some(13) => parsha_list_13[(day / 7) as usize],
            Some(14) => parsha_list_14[(day / 7) as usize],
            Some(15) => parsha_list_15[(day / 7) as usize],
            Some(16) => parsha_list_16[(day / 7) as usize],
            _ => Parsha::NONE,
        }
    }

    // fn get_daf_yomi_yerushalmi(&self) -> Option<Daf> {
    //     let day = self.jewish_date.gregorian_date.day_of_month().0;
    //     let month = self.jewish_date.gregorian_date.month().ordinal;
    //     let year = self
    //         .jewish_date
    //         .gregorian_date
    //         .year()
    //         .era_year_or_related_iso();
    //     let timestamp = NaiveDate::from_ymd_opt(year, month as u32, day as u32)
    //         .unwrap()
    //         .and_hms_opt(0, 0, 0)
    //         .unwrap()
    //         .and_utc()
    //         .timestamp_millis();
    //     YomiCalculator::get_daf_yomi_yerushalmi(timestamp)
    // }
}
