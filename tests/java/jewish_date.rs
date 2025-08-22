/// Java JewishDate wrapper using JNI
pub struct JavaJewishDate<'a> {
    pub jvm: &'a Jvm,
    pub instance: Instance,
}

use j4rs::{Instance, InvocationArg, Jvm};
use zmanim_core::hebrew_calendar::MoladData;

use crate::java::calendar::create_calendar;

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

    fn get_day_of_week(&self) -> zmanim_core::hebrew_calendar::jewish_date::DayOfWeek {
        let result: Instance = self
            .jvm
            .invoke(&self.instance, "getDayOfWeek", InvocationArg::empty())
            .unwrap();
        let day_of_week: i32 = self.jvm.to_rust(result).unwrap();

        day_of_week.into()
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

    fn get_molad(
        &self,
    ) -> (
        impl zmanim_core::hebrew_calendar::JewishDateTrait,
        zmanim_core::hebrew_calendar::MoladData,
    ) {
        let molad_date = self
            .jvm
            .invoke(&self.instance, "getMolad", InvocationArg::empty())
            .unwrap();

        let molad_date = JavaJewishDate {
            jvm: self.jvm,
            instance: molad_date,
        };
        let molad_hours_result = self
            .jvm
            .invoke(
                &molad_date.instance,
                "getMoladHours",
                InvocationArg::empty(),
            )
            .unwrap();
        let molad_hours: i64 = self.jvm.to_rust(molad_hours_result).unwrap();
        let molad_minutes_result = self
            .jvm
            .invoke(
                &molad_date.instance,
                "getMoladMinutes",
                InvocationArg::empty(),
            )
            .unwrap();
        let molad_minutes: i64 = self.jvm.to_rust(molad_minutes_result).unwrap();
        let molad_chalakim_result = self
            .jvm
            .invoke(
                &molad_date.instance,
                "getMoladChalakim",
                InvocationArg::empty(),
            )
            .unwrap();
        let molad_chalakim: i64 = self.jvm.to_rust(molad_chalakim_result).unwrap();
        (
            molad_date,
            MoladData {
                hours: molad_hours,
                minutes: molad_minutes,
                chalakim: molad_chalakim,
            },
        )
    }
}

impl<'a> JavaJewishDate<'a> {
    /// Create a JewishDate from Java Date
    pub fn from_date(jvm: &'a Jvm, timestamp: i64, tz_offset: i64) -> Self {
        let date_instance = create_calendar(jvm, timestamp + tz_offset);
        // let r = jvm
        //     .invoke(&date_instance, "toString", InvocationArg::empty())
        //     .unwrap();
        // let r_str: String = jvm.to_rust(r).unwrap();
        // println!("r: {:?}", r_str);
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.JewishDate",
                &[InvocationArg::from(date_instance)],
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
    pub fn get_molad(&self) -> JavaJewishDate<'_> {
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
    // Note: Static methods isJewishLeapYear, getLastMonthOfJewishYear, etc. are private in Java
    // so we cannot access them via JNI. We use instance methods as workarounds in tests.

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

    /// Check if a Jewish year is a leap year (static method)
    pub fn is_jewish_leap_year_static_java(jvm: &'a Jvm, year: i32) -> bool {
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
    pub fn get_last_month_of_jewish_year_java(jvm: &'a Jvm, year: i32) -> i32 {
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
    pub fn get_jewish_calendar_elapsed_days_java(jvm: &'a Jvm, year: i32) -> i32 {
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
    pub fn get_days_in_jewish_year_static_java(jvm: &'a Jvm, year: i32) -> i32 {
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
}
