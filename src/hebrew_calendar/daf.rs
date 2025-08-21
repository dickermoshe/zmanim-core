/// Represents a Daf (page) in the Babylonian Talmud
/// Each daf consists of a Masechta (tractate) and a page number
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Daf {
    masechta_number: i32,
    daf: i32,
}

impl Daf {
    /// Create a new Daf with the given masechta number and daf number
    pub fn new(masechta_number: i32, daf: i32) -> Self {
        Self {
            masechta_number,
            daf,
        }
    }

    /// Get the masechta (tractate) number
    pub fn get_masechta_number(&self) -> i32 {
        self.masechta_number
    }

    /// Set the masechta (tractate) number
    pub fn set_masechta_number(&mut self, masechta_number: i32) {
        self.masechta_number = masechta_number;
    }

    /// Get the daf (page) number
    pub fn get_daf(&self) -> i32 {
        self.daf
    }

    /// Set the daf (page) number
    pub fn set_daf(&mut self, daf: i32) {
        self.daf = daf;
    }

    /// Get the transliterated masechta name
    pub fn get_masechta_transliterated(&self) -> &'static str {
        Self::get_bavli_masechtos_transliterated()
            .get(self.masechta_number as usize)
            .unwrap_or(&"Unknown")
    }

    /// Get the Hebrew masechta name
    pub fn get_masechta(&self) -> &'static str {
        Self::get_bavli_masechtos()
            .get(self.masechta_number as usize)
            .unwrap_or(&"Unknown")
    }

    /// Get the Yerushalmi (Jerusalem Talmud) masechta transliterated name
    pub fn get_yerushalmi_masechta_transliterated(&self) -> &'static str {
        Self::get_yerushalmi_masechtos_transliterated()
            .get(self.masechta_number as usize)
            .unwrap_or(&"Unknown")
    }

    /// Get the Yerushalmi (Jerusalem Talmud) masechta Hebrew name
    pub fn get_yerushalmi_masechta(&self) -> &'static str {
        Self::get_yerushalmi_masechtos()
            .get(self.masechta_number as usize)
            .unwrap_or(&"Unknown")
    }

    /// Get all Bavli (Babylonian Talmud) masechtos in transliterated form
    pub fn get_bavli_masechtos_transliterated() -> &'static [&'static str] {
        &[
            "Berachos",
            "Shabbos",
            "Eruvin",
            "Pesachim",
            "Shekalim",
            "Yoma",
            "Sukkah",
            "Beitzah",
            "Rosh Hashana",
            "Taanis",
            "Megillah",
            "Moed Katan",
            "Chagigah",
            "Yevamos",
            "Kesubos",
            "Nedarim",
            "Nazir",
            "Sotah",
            "Gitin",
            "Kiddushin",
            "Bava Kamma",
            "Bava Metzia",
            "Bava Basra",
            "Sanhedrin",
            "Makkos",
            "Shevuos",
            "Avodah Zarah",
            "Horiyos",
            "Zevachim",
            "Menachos",
            "Chullin",
            "Bechoros",
            "Arachin",
            "Temurah",
            "Kerisos",
            "Meilah",
            "Kinnim",
            "Tamid",
            "Midos",
            "Niddah",
        ]
    }

    /// Get all Bavli (Babylonian Talmud) masechtos in Hebrew
    pub fn get_bavli_masechtos() -> &'static [&'static str] {
        &[
            "ברכות",
            "שבת",
            "עירובין",
            "פסחים",
            "שקלים",
            "יומא",
            "סוכה",
            "ביצה",
            "ראש השנה",
            "תענית",
            "מגילה",
            "מועד קטן",
            "חגיגה",
            "יבמות",
            "כתובות",
            "נדרים",
            "נזיר",
            "סוטה",
            "גיטין",
            "קידושין",
            "בבא קמא",
            "בבא מציעא",
            "בבא בתרא",
            "סנהדרין",
            "מכות",
            "שבועות",
            "עבודה זרה",
            "הוריות",
            "זבחים",
            "מנחות",
            "חולין",
            "בכורות",
            "ערכין",
            "תמורה",
            "כריתות",
            "מעילה",
            "קינים",
            "תמיד",
            "מידות",
            "נדה",
        ]
    }

    /// Get all Yerushalmi (Jerusalem Talmud) masechtos in transliterated form
    pub fn get_yerushalmi_masechtos_transliterated() -> &'static [&'static str] {
        &[
            "Berachos",
            "Peah",
            "Demai",
            "Kilayim",
            "Sheviis",
            "Terumos",
            "Maasros",
            "Maaser Sheni",
            "Challah",
            "Orlah",
            "Bikurim",
            "Shabbos",
            "Eruvin",
            "Pesachim",
            "Shekalim",
            "Yoma",
            "Sukkah",
            "Beitzah",
            "Rosh Hashana",
            "Taanis",
            "Megillah",
            "Chagigah",
            "Moed Katan",
            "Yevamos",
            "Kesubos",
            "Nedarim",
            "Nazir",
            "Sotah",
            "Gitin",
            "Kiddushin",
            "Bava Kamma",
            "Bava Metzia",
            "Bava Basra",
            "Shevuos",
            "Makkos",
            "Sanhedrin",
            "Avodah Zarah",
            "Horiyos",
            "Niddah",
        ]
    }

    /// Get all Yerushalmi (Jerusalem Talmud) masechtos in Hebrew
    pub fn get_yerushalmi_masechtos() -> &'static [&'static str] {
        &[
            "ברכות",
            "פאה",
            "דמאי",
            "כלאים",
            "שביעית",
            "תרומות",
            "מעשרות",
            "מעשר שני",
            "חלה",
            "ערלה",
            "ביכורים",
            "שבת",
            "עירובין",
            "פסחים",
            "שקלים",
            "יומא",
            "סוכה",
            "ביצה",
            "ראש השנה",
            "תענית",
            "מגילה",
            "חגיגה",
            "מועד קטן",
            "יבמות",
            "כתובות",
            "נדרים",
            "נזיר",
            "סוטה",
            "גיטין",
            "קידושין",
            "בבא קמא",
            "בבא מציעא",
            "בבא בתרא",
            "שבועות",
            "מכות",
            "סנהדרין",
            "עבודה זרה",
            "הוריות",
            "נדה",
        ]
    }
}
