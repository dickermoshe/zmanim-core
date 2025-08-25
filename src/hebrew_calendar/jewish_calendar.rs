use chrono::NaiveDate;
use safer_ffi::derive_ReprC;

use crate::hebrew_calendar::{parsha::*, YomiCalculator};

use super::daf::BavliDaf;
use super::jewish_date::{date_constants, DayOfWeek, JewishDate, JewishDateTrait, JewishMonth};

pub trait JewishCalendarTrait {
    fn get_yom_tov_index(&self) -> Option<Holiday>;

    fn is_yom_tov(&self) -> bool;

    fn is_yom_tov_assur_bemelacha(&self) -> bool;

    fn is_assur_bemelacha(&self) -> bool;

    fn has_candle_lighting(&self) -> bool;

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool;

    fn is_erev_yom_tov_sheni(&self) -> bool;

    fn is_aseres_yemei_teshuva(&self) -> bool;

    fn is_pesach(&self) -> bool;

    fn is_chol_hamoed_pesach(&self) -> bool;

    fn is_shavuos(&self) -> bool;

    fn is_rosh_hashana(&self) -> bool;

    fn is_yom_kippur(&self) -> bool;

    fn is_succos(&self) -> bool;

    fn is_hoshana_rabba(&self) -> bool;

    fn is_shemini_atzeres(&self) -> bool;

    fn is_simchas_torah(&self) -> bool;

    fn is_chol_hamoed_succos(&self) -> bool;

    fn is_chol_hamoed(&self) -> bool;

    fn is_erev_yom_tov(&self) -> bool;

    fn is_rosh_chodesh(&self) -> bool;

    fn is_isru_chag(&self) -> bool;

    fn is_taanis(&self) -> bool;

    fn is_taanis_bechoros(&self) -> bool;

    fn get_day_of_chanukah(&self) -> i32;

    fn is_chanukah(&self) -> bool;

    fn is_purim(&self) -> bool;

    fn get_day_of_omer(&self) -> i32;

    fn is_tisha_beav(&self) -> bool;

    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf>;

    fn get_parshah(&self) -> Parsha;
}

pub trait JewishCalendarConfig {
    fn get_in_israel(&self) -> bool;

    fn get_use_modern_holidays(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JewishCalendar {
    pub jewish_date: JewishDate,
    pub in_israel: bool,
    pub use_modern_holidays: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive_ReprC]
#[repr(u8)]
pub enum Holiday {
    #[allow(non_camel_case_types)]
    EREV_PESACH = 0,
    PESACH = 1,
    #[allow(non_camel_case_types)]
    CHOL_HAMOED_PESACH = 2,
    #[allow(non_camel_case_types)]
    PESACH_SHENI = 3,
    #[allow(non_camel_case_types)]
    EREV_SHAVUOS = 4,
    SHAVUOS = 5,
    #[allow(non_camel_case_types)]
    SEVENTEEN_OF_TAMMUZ = 6,
    #[allow(non_camel_case_types)]
    TISHA_BEAV = 7,
    #[allow(non_camel_case_types)]
    TU_BEAV = 8,
    #[allow(non_camel_case_types)]
    EREV_ROSH_HASHANA = 9,
    #[allow(non_camel_case_types)]
    ROSH_HASHANA = 10,
    #[allow(non_camel_case_types)]
    FAST_OF_GEDALYAH = 11,
    #[allow(non_camel_case_types)]
    EREV_YOM_KIPPUR = 12,
    #[allow(non_camel_case_types)]
    YOM_KIPPUR = 13,
    #[allow(non_camel_case_types)]
    EREV_SUCCOS = 14,
    SUCCOS = 15,
    #[allow(non_camel_case_types)]
    CHOL_HAMOED_SUCCOS = 16,
    #[allow(non_camel_case_types)]
    HOSHANA_RABBA = 17,
    #[allow(non_camel_case_types)]
    SHEMINI_ATZERES = 18,
    #[allow(non_camel_case_types)]
    SIMCHAS_TORAH = 19,
    #[allow(non_camel_case_types)]
    CHANUKAH = 21,
    #[allow(non_camel_case_types)]
    TENTH_OF_TEVES = 22,
    #[allow(non_camel_case_types)]
    TU_BESHVAT = 23,
    #[allow(non_camel_case_types)]
    FAST_OF_ESTHER = 24,
    #[allow(non_camel_case_types)]
    PURIM = 25,
    #[allow(non_camel_case_types)]
    SHUSHAN_PURIM = 26,
    #[allow(non_camel_case_types)]
    PURIM_KATAN = 27,
    #[allow(non_camel_case_types)]
    ROSH_CHODESH = 28,
    #[allow(non_camel_case_types)]
    YOM_HASHOAH = 29,
    #[allow(non_camel_case_types)]
    YOM_HAZIKARON = 30,
    #[allow(non_camel_case_types)]
    YOM_HAATZMAUT = 31,
    #[allow(non_camel_case_types)]
    YOM_YERUSHALAYIM = 32,
    #[allow(non_camel_case_types)]
    LAG_BAOMER = 33,
    #[allow(non_camel_case_types)]
    SHUSHAN_PURIM_KATAN = 34,
    #[allow(non_camel_case_types)]
    ISRU_CHAG = 35,
    #[allow(non_camel_case_types)]
    YOM_KIPPUR_KATAN = 36,
    #[allow(non_camel_case_types)]
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
    pub fn new(
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

    pub fn get_jewish_date(&self) -> &JewishDate {
        &self.jewish_date
    }
}

impl JewishCalendar {
    pub fn is_yom_tov(&self) -> bool {
        let holiday_index = self.get_yom_tov_index();
        if (self.is_erev_yom_tov()
            && !(holiday_index == Some(Holiday::HOSHANA_RABBA)
                || holiday_index == Some(Holiday::CHOL_HAMOED_PESACH)))
            || (self.is_taanis() && holiday_index != Some(Holiday::YOM_KIPPUR))
            || holiday_index == Some(Holiday::ISRU_CHAG)
        {
            return false;
        }
        holiday_index.is_some()
    }

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

    pub fn is_assur_bemelacha(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Saturday
            || self.is_yom_tov_assur_bemelacha()
    }

    pub fn has_candle_lighting(&self) -> bool {
        self.is_tomorrow_shabbos_or_yom_tov()
    }

    pub fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        self.jewish_date.get_day_of_week() == DayOfWeek::Friday
            || self.is_erev_yom_tov()
            || self.is_erev_yom_tov_sheni()
    }

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
                    if self.in_israel {
                        Some(15)
                    } else {
                        Some(7)
                    }
                }
                5 => {
                    if self.jewish_date.is_kislev_short() {
                        Some(8)
                    } else if self.jewish_date.is_cheshvan_long() {
                        Some(9)
                    } else {
                        None
                    }
                }
                7 => {
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
                    if self.in_israel {
                        Some(12)
                    } else {
                        Some(1)
                    }
                }
                5 => {
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

impl JewishCalendarTrait for JewishCalendar {
    fn get_yom_tov_index(&self) -> Option<Holiday> {
        let day = self.jewish_date.get_jewish_day_of_month();
        let day_of_week = self.jewish_date.get_day_of_week();
        let month = self.jewish_date.get_jewish_month();

        match month {
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
                        return Some(Holiday::YOM_HASHOAH);
                    }
                }
            }

            JewishMonth::IYAR => {
                if self.use_modern_holidays {
                    if (day == 4 && day_of_week == DayOfWeek::Tuesday)
                        || ((day == 3 || day == 2) && day_of_week == DayOfWeek::Wednesday)
                        || (day == 5 && day_of_week == DayOfWeek::Monday)
                    {
                        return Some(Holiday::YOM_HAZIKARON);
                    }
                    if (day == 5 && day_of_week == DayOfWeek::Wednesday)
                        || ((day == 4 || day == 3) && day_of_week == DayOfWeek::Thursday)
                        || (day == 6 && day_of_week == DayOfWeek::Tuesday)
                    {
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

            JewishMonth::TAMMUZ => {
                if (day == 17 && day_of_week != DayOfWeek::Saturday)
                    || (day == 18 && day_of_week == DayOfWeek::Sunday)
                {
                    return Some(Holiday::SEVENTEEN_OF_TAMMUZ);
                }
            }

            JewishMonth::AV => {
                if (day_of_week == DayOfWeek::Sunday && day == 10)
                    || (day_of_week != DayOfWeek::Saturday && day == 9)
                {
                    return Some(Holiday::TISHA_BEAV);
                }
                if day == 15 {
                    return Some(Holiday::TU_BEAV);
                }
            }

            JewishMonth::ELUL => {
                if day == 29 {
                    return Some(Holiday::EREV_ROSH_HASHANA);
                }
            }

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

            JewishMonth::KISLEV => {
                if day >= 25 {
                    return Some(Holiday::CHANUKAH);
                }
            }

            JewishMonth::TEVES => {
                if day == 1 || day == 2 || (day == 3 && self.jewish_date.is_kislev_short()) {
                    return Some(Holiday::CHANUKAH);
                }
                if day == 10 {
                    return Some(Holiday::TENTH_OF_TEVES);
                }
            }

            JewishMonth::SHEVAT => {
                if day == 15 {
                    return Some(Holiday::TU_BESHVAT);
                }
            }

            JewishMonth::ADAR => {
                if !self.jewish_date.is_jewish_leap_year() {
                    if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                        || (day == 13
                            && !(day_of_week == DayOfWeek::Friday
                                || day_of_week == DayOfWeek::Saturday))
                    {
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

            JewishMonth::ADARII => {
                if ((day == 11 || day == 12) && day_of_week == DayOfWeek::Thursday)
                    || (day == 13
                        && !(day_of_week == DayOfWeek::Friday
                            || day_of_week == DayOfWeek::Saturday))
                {
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

        (month == date_constants::TISHREI.into() && (day == 1))
            || (!self.in_israel
                && ((month == date_constants::NISSAN.into() && (day == 15 || day == 21))
                    || (month == date_constants::TISHREI.into() && (day == 15 || day == 22))
                    || (month == date_constants::SIVAN.into() && day == 6)))
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();
        month == date_constants::TISHREI.into() && day <= 10
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
        (day == 1 && month != date_constants::TISHREI.into()) || day == 30
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

        month == date_constants::NISSAN.into()
            && ((day == 14 && day_of_week != 7) || (day == 12 && day_of_week == 5))
    }

    fn get_day_of_chanukah(&self) -> i32 {
        if !self.is_chanukah() {
            return -1;
        }

        let month = self.jewish_date.get_jewish_month() as i32;
        let day = self.jewish_date.get_jewish_day_of_month();

        if month == date_constants::KISLEV.into() {
            day - 24
        } else {
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

        if month == date_constants::NISSAN.into() && day >= 16 {
            day - 15
        } else if month == date_constants::IYAR.into() {
            day + 15
        } else if month == date_constants::SIVAN.into() && day < 6 {
            day + 44
        } else {
            -1
        }
    }

    fn is_tisha_beav(&self) -> bool {
        self.get_yom_tov_index() == Some(Holiday::TISHA_BEAV)
    }

    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf> {
        let day = self.jewish_date.gregorian_date.day_of_month().0;
        let month = self.jewish_date.gregorian_date.month().ordinal;
        let year = self
            .jewish_date
            .gregorian_date
            .year()
            .era_year_or_related_iso();
        let timestamp = NaiveDate::from_ymd_opt(year, month as u32, day as u32)?
            .and_hms_opt(0, 0, 0)?
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
            Some(0) => PARSHA_LIST_0[(day / 7) as usize],
            Some(1) => PARSHA_LIST_1[(day / 7) as usize],
            Some(2) => PARSHA_LIST_2[(day / 7) as usize],
            Some(3) => PARSHA_LIST_3[(day / 7) as usize],
            Some(4) => PARSHA_LIST_4[(day / 7) as usize],
            Some(5) => PARSHA_LIST_5[(day / 7) as usize],
            Some(6) => PARSHA_LIST_6[(day / 7) as usize],
            Some(7) => PARSHA_LIST_7[(day / 7) as usize],
            Some(8) => PARSHA_LIST_8[(day / 7) as usize],
            Some(9) => PARSHA_LIST_9[(day / 7) as usize],
            Some(10) => PARSHA_LIST_10[(day / 7) as usize],
            Some(11) => PARSHA_LIST_11[(day / 7) as usize],
            Some(12) => PARSHA_LIST_12[(day / 7) as usize],
            Some(13) => PARSHA_LIST_13[(day / 7) as usize],
            Some(14) => PARSHA_LIST_14[(day / 7) as usize],
            Some(15) => PARSHA_LIST_15[(day / 7) as usize],
            Some(16) => PARSHA_LIST_16[(day / 7) as usize],
            _ => Parsha::NONE,
        }
    }
}

// FFI module - only compiled when FFI is needed
#[cfg(feature = "ffi")]
pub mod ffi {
    use super::*;
    use safer_ffi::{ffi_export, option::TaggedOption};

    #[derive(Debug, Clone, PartialEq, Eq)]
    #[derive_ReprC]
    #[repr(C)]
    pub struct JewishCalendarData {
        // Jewish date information
        pub jewish_year: i32,
        pub jewish_month: JewishMonth,
        pub jewish_day_of_month: i32,
        pub gregorian_year: i32,
        pub gregorian_month: i32,
        pub gregorian_day_of_month: i32,
        pub day_of_week: DayOfWeek,
        pub is_jewish_leap_year: bool,
        pub days_in_jewish_year: i32,
        pub days_in_jewish_month: i32,
        pub is_cheshvan_long: bool,
        pub is_kislev_short: bool,
        pub cheshvan_kislev_kviah: i32, // Using i32 instead of YearLengthType for FFI compatibility
        pub days_since_start_of_jewish_year: i32,
        pub chalakim_since_molad_tohu: i64,

        // Calendar configuration
        pub in_israel: bool,
        pub use_modern_holidays: bool,

        // Holiday and special day information
        pub yom_tov_index: TaggedOption<i32>, // Holiday index or None
        pub is_yom_tov: bool,
        pub is_yom_tov_assur_bemelacha: bool,
        pub is_assur_bemelacha: bool,
        pub has_candle_lighting: bool,
        pub is_tomorrow_shabbos_or_yom_tov: bool,
        pub is_erev_yom_tov_sheni: bool,
        pub is_aseres_yemei_teshuva: bool,
        pub is_pesach: bool,
        pub is_chol_hamoed_pesach: bool,
        pub is_shavuos: bool,
        pub is_rosh_hashana: bool,
        pub is_yom_kippur: bool,
        pub is_succos: bool,
        pub is_hoshana_rabba: bool,
        pub is_shemini_atzeres: bool,
        pub is_simchas_torah: bool,
        pub is_chol_hamoed_succos: bool,
        pub is_chol_hamoed: bool,
        pub is_erev_yom_tov: bool,
        pub is_rosh_chodesh: bool,
        pub is_isru_chag: bool,
        pub is_taanis: bool,
        pub is_taanis_bechoros: bool,
        pub day_of_chanukah: i32,
        pub is_chanukah: bool,
        pub is_purim: bool,
        pub day_of_omer: i32,
        pub is_tisha_beav: bool,

        // Additional information
        pub daf_yomi_bavli: TaggedOption<i32>, // Daf index or None
        pub parshah: i32,                      // Parsha index
    }

    impl JewishCalendarData {
        pub fn from_jewish_calendar(calendar: JewishCalendar) -> Self {
            let jewish_date = calendar.get_jewish_date();

            Self {
                // Jewish date information
                jewish_year: jewish_date.get_jewish_year(),
                jewish_month: jewish_date.get_jewish_month(),
                jewish_day_of_month: jewish_date.get_jewish_day_of_month(),
                gregorian_year: jewish_date.get_gregorian_year(),
                gregorian_month: jewish_date.get_gregorian_month(),
                gregorian_day_of_month: jewish_date.get_gregorian_day_of_month(),
                day_of_week: jewish_date.get_day_of_week(),
                is_jewish_leap_year: jewish_date.is_jewish_leap_year(),
                days_in_jewish_year: jewish_date.get_days_in_jewish_year(),
                days_in_jewish_month: jewish_date.get_days_in_jewish_month(),
                is_cheshvan_long: jewish_date.is_cheshvan_long(),
                is_kislev_short: jewish_date.is_kislev_short(),
                cheshvan_kislev_kviah: jewish_date.get_cheshvan_kislev_kviah() as i32,
                days_since_start_of_jewish_year: jewish_date.get_days_since_start_of_jewish_year(),
                chalakim_since_molad_tohu: jewish_date.get_chalakim_since_molad_tohu(),

                // Calendar configuration
                in_israel: calendar.in_israel,
                use_modern_holidays: calendar.use_modern_holidays,

                // Holiday and special day information
                yom_tov_index: calendar.get_yom_tov_index().map(|h| h as i32).into(),
                is_yom_tov: calendar.is_yom_tov(),
                is_yom_tov_assur_bemelacha: calendar.is_yom_tov_assur_bemelacha(),
                is_assur_bemelacha: calendar.is_assur_bemelacha(),
                has_candle_lighting: calendar.has_candle_lighting(),
                is_tomorrow_shabbos_or_yom_tov: calendar.is_tomorrow_shabbos_or_yom_tov(),
                is_erev_yom_tov_sheni: calendar.is_erev_yom_tov_sheni(),
                is_aseres_yemei_teshuva: calendar.is_aseres_yemei_teshuva(),
                is_pesach: calendar.is_pesach(),
                is_chol_hamoed_pesach: calendar.is_chol_hamoed_pesach(),
                is_shavuos: calendar.is_shavuos(),
                is_rosh_hashana: calendar.is_rosh_hashana(),
                is_yom_kippur: calendar.is_yom_kippur(),
                is_succos: calendar.is_succos(),
                is_hoshana_rabba: calendar.is_hoshana_rabba(),
                is_shemini_atzeres: calendar.is_shemini_atzeres(),
                is_simchas_torah: calendar.is_simchas_torah(),
                is_chol_hamoed_succos: calendar.is_chol_hamoed_succos(),
                is_chol_hamoed: calendar.is_chol_hamoed(),
                is_erev_yom_tov: calendar.is_erev_yom_tov(),
                is_rosh_chodesh: calendar.is_rosh_chodesh(),
                is_isru_chag: calendar.is_isru_chag(),
                is_taanis: calendar.is_taanis(),
                is_taanis_bechoros: calendar.is_taanis_bechoros(),
                day_of_chanukah: calendar.get_day_of_chanukah(),
                is_chanukah: calendar.is_chanukah(),
                is_purim: calendar.is_purim(),
                day_of_omer: calendar.get_day_of_omer(),
                is_tisha_beav: calendar.is_tisha_beav(),

                // Additional information
                daf_yomi_bavli: calendar
                    .get_daf_yomi_bavli()
                    .map(|d| (d.masechta as i32) * 1000 + d.daf)
                    .into(),
                parshah: calendar.get_parshah() as i32,
            }
        }
    }

    #[ffi_export]
    pub fn jewish_calendar_data_from_timestamp(
        timestamp: i64,
        tz_offset: i64,
        in_israel: bool,
        use_modern_holidays: bool,
    ) -> TaggedOption<JewishCalendarData> {
        let calendar = JewishCalendar::new(timestamp, tz_offset, in_israel, use_modern_holidays);
        calendar
            .map(|c| JewishCalendarData::from_jewish_calendar(c))
            .into()
    }
}
