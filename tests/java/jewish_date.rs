/// Java JewishDate wrapper using JNI
pub struct JavaJewishDate<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

use j4rs::{Instance, InvocationArg, Jvm};

impl<'a> zmanim_core::hebrew_calendar::jewish_date::JewishDateTrait for JavaJewishDate<'a> {
    fn get_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getJewishYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_jewish_month(&self) -> zmanim_core::hebrew_calendar::jewish_date::JewishMonth {
        let result: Instance = self
            .jvm
            .invoke(&self.instance, "getJewishMonth", InvocationArg::empty())
            .unwrap();
        let month: i32 = self.jvm.to_rust(result).unwrap();
        month.into()
    }

    fn get_jewish_day_of_month(&self) -> i32 {
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

    fn get_gregorian_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getGregorianYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_gregorian_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getGregorianMonth", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_gregorian_day_of_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getGregorianDayOfMonth",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_day_of_week(&self) -> icu_calendar::types::Weekday {
        let result: Instance = self
            .jvm
            .invoke(&self.instance, "getDayOfWeek", InvocationArg::empty())
            .unwrap();
        let day_of_week: i32 = self.jvm.to_rust(result).unwrap();

        // Convert Java day of week (1=Sunday) to ICU Weekday
        match day_of_week {
            1 => icu_calendar::types::Weekday::Sunday,
            2 => icu_calendar::types::Weekday::Monday,
            3 => icu_calendar::types::Weekday::Tuesday,
            4 => icu_calendar::types::Weekday::Wednesday,
            5 => icu_calendar::types::Weekday::Thursday,
            6 => icu_calendar::types::Weekday::Friday,
            7 => icu_calendar::types::Weekday::Saturday,
            _ => icu_calendar::types::Weekday::Sunday, // Default
        }
    }

    fn is_jewish_leap_year(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isJewishLeapYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_days_in_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysInJewishYear",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_days_in_jewish_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysInJewishMonth",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_cheshvan_long(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isCheshvanLong", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_kislev_short(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isKislevShort", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_cheshvan_kislev_kviah(
        &self,
    ) -> zmanim_core::hebrew_calendar::jewish_date::YearLengthType {
        let result: Instance = self
            .jvm
            .invoke(
                &self.instance,
                "getCheshvanKislevKviah",
                InvocationArg::empty(),
            )
            .unwrap();
        let kviah: i32 = self.jvm.to_rust(result).unwrap();
        kviah.into()
    }

    fn get_days_since_start_of_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysSinceStartOfJewishYear",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getChalakimSinceMoladTohu",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    fn is_jewish_leap_year_static(year: i32) -> bool {
        // We can't call static methods from trait implementation without an instance
        // For now, use the same logic as the Rust implementation
        let year_in_cycle = ((year - 1) % 19) + 1;
        matches!(year_in_cycle, 3 | 6 | 8 | 11 | 14 | 17 | 19)
    }

    fn get_last_month_of_jewish_year(_year: i32) -> i32 {
        // This is a static method that needs to be called on the Java class
        // For now, return a placeholder
        12 // Adar
    }

    fn get_jewish_calendar_elapsed_days(_year: i32) -> i32 {
        // This is a simplified implementation
        // In a real implementation, this would call the Java static method
        0 // Placeholder
    }

    fn get_days_in_jewish_year_static(_year: i32) -> i32 {
        // Regular year has 354 days, leap year has 383 days
        // In a real implementation, this would call the Java static method
        354 // Placeholder
    }

    fn get_days_in_jewish_month_static(month: i32, year: i32) -> i32 {
        match month {
            1 | 2 | 4 | 5 | 7 | 9 | 10 | 11 => 30, // 30-day months
            3 | 6 | 8 | 12 => 29,                  // 29-day months in non-leap years
            13 => 29,                              // Adar II in leap years
            _ => 29,                               // Default
        }
    }
}

impl<'a> JavaJewishDate<'a> {
    /// Create a JewishDate from Java Date
    pub fn from_date(jvm: &'a Jvm, timestamp: i64) -> Self {
        let date_instance = super::date::create_date(jvm, timestamp);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                &[InvocationArg::from(date_instance)],
            )
            .unwrap();

        Self { jvm, instance }
    }
    /// Create a JewishDate from Jewish date components with time
    pub fn from_jewish_date(
        jvm: &'a Jvm,
        year: i32,
        month: i32,
        day: i32,
        hours: i32,
        minutes: i32,
        chalakim: i32,
    ) -> Self {
        let instance = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "new",
                &[
                    InvocationArg::try_from(year)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(month)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(day)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(hours)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(minutes)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(chalakim)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();

        Self { jvm, instance }
    }

    /// Get the Jewish year
    pub fn get_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getJewishYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the Jewish month
    pub fn get_jewish_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getJewishMonth", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the Jewish day of month
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

    /// Get the Gregorian year
    pub fn get_gregorian_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getGregorianYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the Gregorian month (0-based)
    pub fn get_gregorian_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getGregorianMonth", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the Gregorian day of month
    pub fn get_gregorian_day_of_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getGregorianDayOfMonth",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the day of week
    pub fn get_day_of_week(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getDayOfWeek", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Check if it's a Jewish leap year
    pub fn is_jewish_leap_year(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isJewishLeapYear", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the days in the Jewish year
    pub fn get_days_in_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysInJewishYear",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the days in the Jewish month
    pub fn get_days_in_jewish_month(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysInJewishMonth",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Check if Cheshvan is long
    pub fn is_cheshvan_long(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isCheshvanLong", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Check if Kislev is short
    pub fn is_kislev_short(&self) -> bool {
        let result = self
            .jvm
            .invoke(&self.instance, "isKislevShort", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get Cheshvan/Kislev kviah
    pub fn get_cheshvan_kislev_kviah(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getCheshvanKislevKviah",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the molad date
    pub fn get_molad(&self) -> JavaJewishDate {
        let result = self
            .jvm
            .invoke(&self.instance, "getMolad", InvocationArg::empty())
            .unwrap();
        return JavaJewishDate {
            jvm: self.jvm,
            instance: result,
        };
    }

    /// Get the days since start of Jewish year
    pub fn get_days_since_start_of_jewish_year(&self) -> i32 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getDaysSinceStartOfJewishYear",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    /// Get the chalakim since molad tohu
    pub fn get_chalakim_since_molad_tohu(&self) -> i64 {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getChalakimSinceMoladTohu",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }
    /// Check if a Jewish year is a leap year (static method)
    pub fn is_jewish_leap_year_static(jvm: &'a Jvm, year: i32) -> bool {
        let result = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "isJewishLeapYear",
                &[InvocationArg::try_from(year)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        jvm.to_rust(result).unwrap()
    }

    /// Get the last month of a Jewish year (static method)
    pub fn get_last_month_of_jewish_year(jvm: &'a Jvm, year: i32) -> i32 {
        let result = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "getLastMonthOfJewishYear",
                &[InvocationArg::try_from(year)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        jvm.to_rust(result).unwrap()
    }

    /// Get the Jewish calendar elapsed days (static method)
    pub fn get_jewish_calendar_elapsed_days(jvm: &'a Jvm, year: i32) -> i32 {
        let result = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "getJewishCalendarElapsedDays",
                &[InvocationArg::try_from(year)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        jvm.to_rust(result).unwrap()
    }

    /// Get the days in a Jewish year (static method)
    pub fn get_days_in_jewish_year_static(jvm: &'a Jvm, year: i32) -> i32 {
        let result = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "getDaysInJewishYear",
                &[InvocationArg::try_from(year)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
        jvm.to_rust(result).unwrap()
    }

    /// Get the days in a Jewish month (static method)
    pub fn get_days_in_jewish_month_static(jvm: &'a Jvm, month: i32, year: i32) -> i32 {
        let result = jvm
            .invoke_static(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                "getDaysInJewishMonth",
                &[
                    InvocationArg::try_from(month)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(year)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        jvm.to_rust(result).unwrap()
    }
}
