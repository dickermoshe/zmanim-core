/// Represents a tractate in the Babylonian Talmud (Bavli) for Daf Yomi
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BavliMasechta {
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

/// Represents a tractate in the Jerusalem Talmud (Yerushalmi)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum YerushalmiMasechta {
    Berachos = 0,
    Peah = 1,
    Demai = 2,
    Kilayim = 3,
    Sheviis = 4,
    Terumos = 5,
    Maasros = 6,
    MaaserSheni = 7,
    Chalah = 8,
    Orlah = 9,
    Bikurim = 10,
    Shabbos = 11,
    Eruvin = 12,
    Pesachim = 13,
    Beitzah = 14,
    RoshHashanah = 15,
    Yoma = 16,
    Sukah = 17,
    Taanis = 18,
    Shekalim = 19,
    Megilah = 20,
    Chagigah = 21,
    MoedKatan = 22,
    Yevamos = 23,
    Kesuvos = 24,
    Sotah = 25,
    Nedarim = 26,
    Nazir = 27,
    Gitin = 28,
    Kidushin = 29,
    BavaKama = 30,
    BavaMetzia = 31,
    BavaBasra = 32,
    Shevuos = 33,
    Makos = 34,
    Sanhedrin = 35,
    AvodahZarah = 36,
    Horayos = 37,
    Nidah = 38,
    NoDafToday = 39,
}

/// Represents a daf (page) in the Daf Yomi cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BavliDaf {
    pub tractate: BavliMasechta,
    pub page: u16,
}

/// Represents a daf (page) in the Yerushalmi Daf Yomi cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YerushalmiDaf {
    pub tractate: YerushalmiMasechta,
    pub page: u16,
}

impl BavliDaf {
    /// Create a new Bavli Daf with the specified tractate and page number
    pub const fn new(tractate: BavliMasechta, page: u16) -> Self {
        Self { tractate, page }
    }
}

impl YerushalmiDaf {
    /// Create a new Yerushalmi Daf with the specified tractate and page number
    pub const fn new(tractate: YerushalmiMasechta, page: u16) -> Self {
        Self { tractate, page }
    }
}
