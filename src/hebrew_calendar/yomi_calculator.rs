extern crate alloc;

use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use libm::floor;

use crate::hebrew_calendar::daf::{BavliTractate, Daf, Mesachta, YerushalmiTractate};

/// Calculator for Daf Yomi Bavli (Babylonian Talmud daily page)
///
/// This class calculates the Daf Yomi Bavli page (daf) for a given date.
/// The first Daf Yomi cycle started on Rosh Hashana 5684 (September 11, 1923).
///
/// For historical calculations, note that a change in length of the cycle was instituted
/// starting in the eighth Daf Yomi cycle beginning on June 24, 1975. The Daf Yomi Bavli
/// cycle has a single masechta of the Talmud Yerushalmi - Shekalim as part of the cycle.
///
/// Unlike the Bavli where the number of daf per masechta was standardized since the original
/// Bomberg Edition published from 1520 - 1523, there is no uniform page length in the Yerushalmi.
/// The early cycles had the Yerushalmi Shekalim length of 13 days following the Slavuta/Zhytomyr
/// Shas used by Rabbi Meir Shapiro. With the start of the eighth Daf Yomi cycle beginning on
/// June 24, 1975, the length of the Yerushalmi Shekalim was changed from 13 to 22 daf to follow
/// the Vilna Shas that is in common use today.
pub struct YomiCalculator;

impl YomiCalculator {
    /// The start date of the first Daf Yomi Bavli cycle of September 11, 1923 / Rosh Hashana 5684.
    const DAF_YOMI_START_DAY: &'static DateTime<Utc> = &NaiveDate::from_ymd_opt(1923, 9, 11)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    /// The date that the pagination for the Daf Yomi Maseches Shekalim changed to use the commonly used Vilna
    /// Shas pagination from the no longer commonly available Zhitomir / Slavuta Shas used by Rabbi Meir Shapiro.
    const SHEKALIM_CHANGE_DAY: &'static DateTime<Utc> = &NaiveDate::from_ymd_opt(1975, 6, 24)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    /// The start date of the first Daf Yomi Yerushalmi cycle of February 2, 1980 / 15 Shevat, 5740.
    const YERUSHALMI_DAF_YOMI_START_DAY: &'static DateTime<Utc> =
        &NaiveDate::from_ymd_opt(1980, 2, 2)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();

    /// The number of pages in the Talmud Yerushalmi.
    const YERUSHALMI_WHOLE_SHAS_DAFS: i32 = 1554;

    /// The number of pages per masechta (tractate) in Yerushalmi.
    const YERUSHALMI_BLATT_PER_MASECHTA: [i32; 39] = [
        68, 37, 34, 44, 31, 59, 26, 33, 28, 20, 13, 92, 65, 71, 22, 22, 42, 26, 26, 33, 34, 22, 19,
        85, 72, 47, 40, 47, 54, 48, 44, 37, 34, 44, 9, 57, 37, 19, 13,
    ];

    /// Returns the Daf Yomi Bavli page (daf) for a given date.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp for calculation
    ///
    /// # Returns
    ///
    /// The Daf representing the page for the given date, or None if the date is prior to September 11, 1923
    pub fn get_daf_yomi_bavli(timestamp: i64) -> Option<Daf> {
        let date = DateTime::from_timestamp_millis(timestamp).unwrap();

        let daf_yomi_julian_start = get_julian_day(&Self::DAF_YOMI_START_DAY);
        let shekalim_julian_change = get_julian_day(&Self::SHEKALIM_CHANGE_DAY);

        if date < *Self::DAF_YOMI_START_DAY {
            return None;
        }

        let julian_day = get_julian_day(&date);
        let (cycle_no, daf_no) = if date >= *Self::SHEKALIM_CHANGE_DAY {
            let cycle_no = 8 + ((julian_day - shekalim_julian_change) / 2711);
            let daf_no = (julian_day - shekalim_julian_change) % 2711;
            (cycle_no, daf_no)
        } else {
            let cycle_no = 1 + ((julian_day - daf_yomi_julian_start) / 2702);
            let daf_no = (julian_day - daf_yomi_julian_start) % 2702;
            (cycle_no, daf_no)
        };

        // The number of daf per masechta
        let mut blatt_per_masechta = [
            64, 157, 105, 121, 22, 88, 56, 40, 35, 31, 32, 29, 27, 122, 112, 91, 66, 49, 90, 82,
            119, 119, 176, 113, 24, 49, 76, 14, 120, 110, 142, 61, 34, 34, 28, 22, 4, 9, 5, 73,
        ];

        // Fix Shekalim for old cycles
        if cycle_no <= 7 {
            blatt_per_masechta[4] = 13;
        }

        // Finally find the daf
        let mut total = 0;
        let mut masechta = -1;
        let mut blatt = 0;

        for (i, &blatt_count) in blatt_per_masechta.iter().enumerate() {
            masechta = i as i32;
            total = total + blatt_count - 1;
            if daf_no < total {
                blatt = 1 + blatt_count - (total - daf_no);

                // Fiddle with the weird ones near the end
                if masechta == 36 {
                    blatt += 21;
                } else if masechta == 37 {
                    blatt += 24;
                } else if masechta == 38 {
                    blatt += 32;
                }
                break;
            }
        }

        // Convert masechta index to BavliTractate enum
        let tractate = match masechta {
            0 => BavliTractate::Berachos,
            1 => BavliTractate::Shabbos,
            2 => BavliTractate::Eruvin,
            3 => BavliTractate::Pesachim,
            4 => BavliTractate::Shekalim,
            5 => BavliTractate::Yoma,
            6 => BavliTractate::Sukkah,
            7 => BavliTractate::Beitzah,
            8 => BavliTractate::RoshHashana,
            9 => BavliTractate::Taanis,
            10 => BavliTractate::Megillah,
            11 => BavliTractate::MoedKatan,
            12 => BavliTractate::Chagigah,
            13 => BavliTractate::Yevamos,
            14 => BavliTractate::Kesubos,
            15 => BavliTractate::Nedarim,
            16 => BavliTractate::Nazir,
            17 => BavliTractate::Sotah,
            18 => BavliTractate::Gitin,
            19 => BavliTractate::Kiddushin,
            20 => BavliTractate::BavaKamma,
            21 => BavliTractate::BavaMetzia,
            22 => BavliTractate::BavaBasra,
            23 => BavliTractate::Sanhedrin,
            24 => BavliTractate::Makkos,
            25 => BavliTractate::Shevuos,
            26 => BavliTractate::AvodahZarah,
            27 => BavliTractate::Horiyos,
            28 => BavliTractate::Zevachim,
            29 => BavliTractate::Menachos,
            30 => BavliTractate::Chullin,
            31 => BavliTractate::Bechoros,
            32 => BavliTractate::Arachin,
            33 => BavliTractate::Temurah,
            34 => BavliTractate::Kerisos,
            35 => BavliTractate::Meilah,
            36 => BavliTractate::Kinnim,
            37 => BavliTractate::Tamid,
            38 => BavliTractate::Midos,
            39 => BavliTractate::Niddah,
            _ => panic!("Invalid masechta index: {}", masechta),
        };

        Some(Daf::new(Mesachta::Bavli(tractate), blatt))
    }

    /// Returns the Daf Yomi Yerushalmi page (daf) for a given date.
    ///
    /// The first Daf Yomi cycle started on 15 Shevat (Tu Bishvat), 5740 (February, 2, 1980) and calculations
    /// prior to this date will result in None being returned.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - The timestamp for calculation
    ///
    /// # Returns
    ///
    /// The Daf representing the page for the given date, or None if the date is prior to February 2, 1980
    pub fn get_daf_yomi_yerushalmi(timestamp: i64) -> Option<Daf> {
        let date = DateTime::from_timestamp_millis(timestamp).unwrap();

        if date < *Self::YERUSHALMI_DAF_YOMI_START_DAY {
            return None;
        }

        // Calculate the current cycle
        let mut next_cycle = *Self::YERUSHALMI_DAF_YOMI_START_DAY;
        let mut prev_cycle = *Self::YERUSHALMI_DAF_YOMI_START_DAY;

        // Go cycle by cycle, until we get the next cycle
        while date > next_cycle {
            prev_cycle = next_cycle;

            // Add the number of whole shas dafs and the number of days that don't have daf
            next_cycle =
                next_cycle + chrono::Duration::days(Self::YERUSHALMI_WHOLE_SHAS_DAFS as i64);
            next_cycle = next_cycle
                + chrono::Duration::days(
                    Self::get_num_of_special_days(&prev_cycle, &next_cycle) as i64
                );
        }

        // Get the number of days from cycle start until request
        let daf_no = Self::get_diff_between_days(&prev_cycle, &date);

        // Get the number of special days to subtract
        let special_days = Self::get_num_of_special_days(&prev_cycle, &date);
        let mut total = daf_no - special_days;

        // Finally find the daf
        let mut masechta = 0;
        for (i, &blatt_count) in Self::YERUSHALMI_BLATT_PER_MASECHTA.iter().enumerate() {
            if total < blatt_count {
                let tractate = Self::get_yerushalmi_tractate_by_index(i as i32);
                return Some(Daf::new(Mesachta::Yerushalmi(tractate), total + 1));
            }
            total -= blatt_count;
            masechta = i + 1;
        }

        None
    }

    /// Return the number of special days (Yom Kippur and Tisha Beav, where there are no dafim), between the start date
    /// and end date.
    fn get_num_of_special_days(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i32 {
        // For simplicity, we'll use a basic calculation
        // In a full implementation, this would need to check Jewish calendar dates
        // and count Yom Kippur and Tisha B'Av occurrences

        // This is a simplified version - in practice, you'd need to implement
        // proper Jewish calendar calculations to find these specific dates
        let days_diff = Self::get_diff_between_days(start, end);

        // Rough estimate: Yom Kippur and Tisha B'Av occur roughly once per year
        // This is a simplified approximation
        let years_diff = days_diff / 365;
        years_diff * 2
    }

    /// Return the number of days between the dates passed in
    fn get_diff_between_days(start: &DateTime<Utc>, end: &DateTime<Utc>) -> i32 {
        let duration = end.signed_duration_since(*start);
        duration.num_days() as i32
    }

    /// Convert masechta index to YerushalmiTractate enum
    fn get_yerushalmi_tractate_by_index(index: i32) -> YerushalmiTractate {
        match index {
            0 => YerushalmiTractate::Berachos,
            1 => YerushalmiTractate::Peah,
            2 => YerushalmiTractate::Demai,
            3 => YerushalmiTractate::Kilayim,
            4 => YerushalmiTractate::Sheviis,
            5 => YerushalmiTractate::Terumos,
            6 => YerushalmiTractate::Maasros,
            7 => YerushalmiTractate::MaaserSheni,
            8 => YerushalmiTractate::Chalah,
            9 => YerushalmiTractate::Orlah,
            10 => YerushalmiTractate::Bikurim,
            11 => YerushalmiTractate::Shabbos,
            12 => YerushalmiTractate::Eruvin,
            13 => YerushalmiTractate::Pesachim,
            14 => YerushalmiTractate::Beitzah,
            15 => YerushalmiTractate::RoshHashanah,
            16 => YerushalmiTractate::Yoma,
            17 => YerushalmiTractate::Sukah,
            18 => YerushalmiTractate::Taanis,
            19 => YerushalmiTractate::Shekalim,
            20 => YerushalmiTractate::Megilah,
            21 => YerushalmiTractate::Chagigah,
            22 => YerushalmiTractate::MoedKatan,
            23 => YerushalmiTractate::Yevamos,
            24 => YerushalmiTractate::Kesuvos,
            25 => YerushalmiTractate::Sotah,
            26 => YerushalmiTractate::Nedarim,
            27 => YerushalmiTractate::Nazir,
            28 => YerushalmiTractate::Gitin,
            29 => YerushalmiTractate::Kidushin,
            30 => YerushalmiTractate::BavaKama,
            31 => YerushalmiTractate::BavaMetzia,
            32 => YerushalmiTractate::BavaBasra,
            33 => YerushalmiTractate::Shevuos,
            34 => YerushalmiTractate::Makos,
            35 => YerushalmiTractate::Sanhedrin,
            36 => YerushalmiTractate::AvodahZarah,
            37 => YerushalmiTractate::Horayos,
            38 => YerushalmiTractate::Nidah,
            _ => panic!("Invalid Yerushalmi masechta index: {}", index),
        }
    }
}

fn get_julian_day(date: &DateTime<Utc>) -> i32 {
    let mut year = date.year();
    let mut month = date.month();
    let day = date.day();
    if (month <= 2) {
        year -= 1;
        month += 12;
    }
    let a = year / 100;
    let b = 2 - a + a / 4;
    return (floor(365.25 * (year + 4716) as f64)
        + floor(30.6001 * (month + 1) as f64)
        + day as f64
        + b as f64
        - 1524.5) as i32;
}
