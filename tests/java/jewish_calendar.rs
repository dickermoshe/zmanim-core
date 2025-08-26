#![allow(dead_code, unused)]

#[allow(dead_code, unused)]
pub struct JavaJewishCalendar<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
    pub timestamp: i64,
    pub tz_offset: i64,
}

use j4rs::{Instance, InvocationArg, Jvm};
use zmanim_core::prelude::{jewish_calendar::GetDafYomiBavliTrait, *};

use crate::java::calendar::create_calendar;

impl Clone for JavaJewishCalendar<'_> {
    fn clone(&self) -> Self {
        JavaJewishCalendar::from_date(self.jvm, self.timestamp, self.tz_offset)
    }
}

#[allow(dead_code, unused)]
impl<'a> JewishCalendarTrait for JavaJewishCalendar<'a> {
    fn get_yom_tov_index(&self) -> Option<JewishHoliday> {
        let result = self
            .jvm
            .invoke(&self.instance, "getYomTovIndex", InvocationArg::empty())
            .unwrap();
        let index = self.jvm.to_rust::<i32>(result).unwrap();
        if index == -1 {
            None
        } else {
            Some(JewishHoliday::from_index(index))
        }
    }

    fn is_yom_tov(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isYomTov", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_yom_tov_assur_bemelacha(&self) -> bool {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "isYomTovAssurBemelacha",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_assur_bemelacha(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isAssurBemelacha", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn has_candle_lighting(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "hasCandleLighting", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_tomorrow_shabbos_or_yom_tov(&self) -> bool {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "isTomorrowShabbosOrYomTov",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_erev_yom_tov_sheni(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isErevYomTovSheni", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_aseres_yemei_teshuva(&self) -> bool {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "isAseresYemeiTeshuva",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_pesach(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isPesach", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_chol_hamoed_pesach(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isCholHamoedPesach", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_shavuos(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isShavuos", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_rosh_hashana(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isRoshHashana", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_yom_kippur(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isYomKippur", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_succos(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isSuccos", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_hoshana_rabba(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isHoshanaRabba", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_shemini_atzeres(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isShminiAtzeres", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_simchas_torah(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isSimchasTorah", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_chol_hamoed_succos(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isCholHamoedSuccos", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_chol_hamoed(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isCholHamoed", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_erev_yom_tov(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isErevYomTov", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_rosh_chodesh(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isRoshChodesh", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_isru_chag(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isIsruChag", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_taanis(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isTaanis", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_taanis_bechoros(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isTaanisBechoros", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_day_of_chanukah(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getDayOfChanukah", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_chanukah(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isChanukah", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_purim(&self) -> bool {
        let purim_result = self
            .jvm
            .invoke(&self.instance, "isPurim", InvocationArg::empty())
            .unwrap();
        let is_purim: bool = self.jvm.to_rust(purim_result).unwrap();
        if is_purim {
            return true;
        }

        let yom_tov_idx = self
            .jvm
            .invoke(&self.instance, "getYomTovIndex", InvocationArg::empty())
            .unwrap();
        let idx: i32 = self.jvm.to_rust(yom_tov_idx).unwrap();

        idx == 26
    }

    fn get_day_of_omer(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getDayOfOmer", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_tisha_beav(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isTishaBav", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_parshah(&self) -> Parsha {
        let result: Instance = self
            .jvm
            .invoke(&self.instance, "getParshah", InvocationArg::empty())
            .unwrap();

        let parsha_name: String = self.jvm.to_rust(result).unwrap();

        match parsha_name.as_str() {
            "NONE" => Parsha::NONE,
            "BERESHIS" => Parsha::BERESHIS,
            "NOACH" => Parsha::NOACH,
            "LECH_LECHA" => Parsha::LECH_LECHA,
            "VAYERA" => Parsha::VAYERA,
            "CHAYEI_SARA" => Parsha::CHAYEI_SARA,
            "TOLDOS" => Parsha::TOLDOS,
            "VAYETZEI" => Parsha::VAYETZEI,
            "VAYISHLACH" => Parsha::VAYISHLACH,
            "VAYESHEV" => Parsha::VAYESHEV,
            "MIKETZ" => Parsha::MIKETZ,
            "VAYIGASH" => Parsha::VAYIGASH,
            "VAYECHI" => Parsha::VAYECHI,
            "SHEMOS" => Parsha::SHEMOS,
            "VAERA" => Parsha::VAERA,
            "BO" => Parsha::BO,
            "BESHALACH" => Parsha::BESHALACH,
            "YISRO" => Parsha::YISRO,
            "MISHPATIM" => Parsha::MISHPATIM,
            "TERUMAH" => Parsha::TERUMAH,
            "TETZAVEH" => Parsha::TETZAVEH,
            "KI_SISA" => Parsha::KI_SISA,
            "VAYAKHEL" => Parsha::VAYAKHEL,
            "PEKUDEI" => Parsha::PEKUDEI,
            "VAYIKRA" => Parsha::VAYIKRA,
            "TZAV" => Parsha::TZAV,
            "SHMINI" => Parsha::SHMINI,
            "TAZRIA" => Parsha::TAZRIA,
            "METZORA" => Parsha::METZORA,
            "ACHREI_MOS" => Parsha::ACHREI_MOS,
            "KEDOSHIM" => Parsha::KEDOSHIM,
            "EMOR" => Parsha::EMOR,
            "BEHAR" => Parsha::BEHAR,
            "BECHUKOSAI" => Parsha::BECHUKOSAI,
            "BAMIDBAR" => Parsha::BAMIDBAR,
            "NASSO" => Parsha::NASSO,
            "BEHAALOSCHA" => Parsha::BEHAALOSCHA,
            "SHLACH" => Parsha::SHLACH,
            "KORACH" => Parsha::KORACH,
            "CHUKAS" => Parsha::CHUKAS,
            "BALAK" => Parsha::BALAK,
            "PINCHAS" => Parsha::PINCHAS,
            "MATOS" => Parsha::MATOS,
            "MASEI" => Parsha::MASEI,
            "DEVARIM" => Parsha::DEVARIM,
            "VAESCHANAN" => Parsha::VAESCHANAN,
            "EIKEV" => Parsha::EIKEV,
            "REEH" => Parsha::REEH,
            "SHOFTIM" => Parsha::SHOFTIM,
            "KI_SEITZEI" => Parsha::KI_SEITZEI,
            "KI_SAVO" => Parsha::KI_SAVO,
            "NITZAVIM" => Parsha::NITZAVIM,
            "VAYEILECH" => Parsha::VAYEILECH,
            "HAAZINU" => Parsha::HAAZINU,
            "VZOS_HABERACHA" => Parsha::VZOS_HABERACHA,
            "VAYAKHEL_PEKUDEI" => Parsha::VAYAKHEL_PEKUDEI,
            "TAZRIA_METZORA" => Parsha::TAZRIA_METZORA,
            "ACHREI_MOS_KEDOSHIM" => Parsha::ACHREI_MOS_KEDOSHIM,
            "BEHAR_BECHUKOSAI" => Parsha::BEHAR_BECHUKOSAI,
            "CHUKAS_BALAK" => Parsha::CHUKAS_BALAK,
            "MATOS_MASEI" => Parsha::MATOS_MASEI,
            "NITZAVIM_VAYEILECH" => Parsha::NITZAVIM_VAYEILECH,
            "SHKALIM" => Parsha::SHKALIM,
            "ZACHOR" => Parsha::ZACHOR,
            "PARA" => Parsha::PARA,
            "HACHODESH" => Parsha::HACHODESH,
            "SHUVA" => Parsha::SHUVA,
            "SHIRA" => Parsha::SHIRA,
            "HAGADOL" => Parsha::HAGADOL,
            "CHAZON" => Parsha::CHAZON,
            "NACHAMU" => Parsha::NACHAMU,
            _ => Parsha::NONE,
        }
    }
}

impl<'a> JavaJewishCalendar<'a> {
    pub fn from_date(jvm: &'a Jvm, timestamp: i64, tz_offset: i64) -> Self {
        let date_instance = create_calendar(jvm, timestamp + tz_offset);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishCalendar",
                &[InvocationArg::from(date_instance)],
            )
            .unwrap();

        Self {
            jvm,
            instance,
            timestamp,
            tz_offset,
        }
    }

    pub fn set_in_israel(&mut self, in_israel: bool) {
        let _ = self.jvm.invoke(
            &self.instance,
            "setInIsrael",
            &[InvocationArg::try_from(in_israel)
                .unwrap()
                .into_primitive()
                .unwrap()],
        );
    }

    pub fn get_in_israel(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "getInIsrael", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn set_use_modern_holidays(&mut self, use_modern_holidays: bool) {
        let _ = self.jvm.invoke(
            &self.instance,
            "setUseModernHolidays",
            &[InvocationArg::try_from(use_modern_holidays)
                .unwrap()
                .into_primitive()
                .unwrap()],
        );
    }

    pub fn get_use_modern_holidays(&self) -> bool {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "isUseModernHolidays",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getJewishYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_jewish_month_int(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getJewishMonth", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_jewish_day_of_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getJewishDayOfMonth",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_day_of_week_int(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getDayOfWeek", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }
}

impl GetDafYomiBavliTrait for JavaJewishCalendar<'_> {
    fn get_daf_yomi_bavli(&self) -> Option<BavliDaf> {
        let result = self
            .jvm
            .invoke(&self.instance, "getDafYomiBavli", InvocationArg::empty());

        let result = result.ok()?;

        let masechta_result = self
            .jvm
            .invoke(&result, "getMasechtaNumber", InvocationArg::empty())
            .unwrap();
        let masechta_number: u32 = self.jvm.to_rust(masechta_result).unwrap();

        let daf_result = self
            .jvm
            .invoke(&result, "getDaf", InvocationArg::empty())
            .unwrap();
        let daf_number: i32 = self.jvm.to_rust(daf_result).unwrap();

        Some(BavliDaf::new(
            BavliTractate::from(masechta_number as i32),
            daf_number,
        ))
    }
}
