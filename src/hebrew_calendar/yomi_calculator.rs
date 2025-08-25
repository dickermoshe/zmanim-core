extern crate alloc;

use chrono::{DateTime, Datelike, NaiveDate, Utc};
use libm::floor;

use crate::hebrew_calendar::daf::{BavliTractate, Daf, Mesachta};

pub struct YomiCalculator;

impl YomiCalculator {
    const DAF_YOMI_START_DAY: &'static DateTime<Utc> = &NaiveDate::from_ymd_opt(1923, 9, 11)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

    const SHEKALIM_CHANGE_DAY: &'static DateTime<Utc> = &NaiveDate::from_ymd_opt(1975, 6, 24)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();

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

        let mut blatt_per_masechta = [
            64, 157, 105, 121, 22, 88, 56, 40, 35, 31, 32, 29, 27, 122, 112, 91, 66, 49, 90, 82,
            119, 119, 176, 113, 24, 49, 76, 14, 120, 110, 142, 61, 34, 34, 28, 22, 4, 9, 5, 73,
        ];

        if cycle_no <= 7 {
            blatt_per_masechta[4] = 13;
        }

        let mut total = 0;
        let mut masechta = -1;
        let mut blatt = 0;

        for (i, &blatt_count) in blatt_per_masechta.iter().enumerate() {
            masechta = i as i32;
            total = total + blatt_count - 1;
            if daf_no < total {
                blatt = 1 + blatt_count - (total - daf_no);

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
}

fn get_julian_day(date: &DateTime<Utc>) -> i32 {
    let mut year = date.year();
    let mut month = date.month();
    let day = date.day();
    if month <= 2 {
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
