#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BavliTractate {
    Berachos = 0,
    Shabbos = 1,
    Eruvin = 2,
    Pesachim = 3,
    Shekalim = 4,
    Yoma = 5,
    Sukkah = 6,
    Beitzah = 7,
    RoshHashana = 8,
    Taanis = 9,
    Megillah = 10,
    MoedKatan = 11,
    Chagigah = 12,
    Yevamos = 13,
    Kesubos = 14,
    Nedarim = 15,
    Nazir = 16,
    Sotah = 17,
    Gitin = 18,
    Kiddushin = 19,
    BavaKamma = 20,
    BavaMetzia = 21,
    BavaBasra = 22,
    Sanhedrin = 23,
    Makkos = 24,
    Shevuos = 25,
    AvodahZarah = 26,
    Horiyos = 27,
    Zevachim = 28,
    Menachos = 29,
    Chullin = 30,
    Bechoros = 31,
    Arachin = 32,
    Temurah = 33,
    Kerisos = 34,
    Meilah = 35,
    Kinnim = 36,
    Tamid = 37,
    Midos = 38,
    Niddah = 39,
}
impl From<i32> for BavliTractate {
    fn from(value: i32) -> Self {
        match value {
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
            _ => panic!("Invalid Bavli tractate number: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct BavliDaf {
    pub masechta: BavliTractate,
    pub daf: i32,
}

impl BavliDaf {
    pub fn new(masechta: BavliTractate, daf: i32) -> Self {
        Self { masechta, daf }
    }
}
