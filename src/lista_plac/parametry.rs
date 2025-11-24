#[derive(Clone, Copy)]
#[allow(dead_code, unused)]
pub enum KosztyUzyskania {
    Brak,
    Podstawowe,
    Podwyzszone,
}

impl KosztyUzyskania {
    pub fn wartosc(self) -> f32 {
        match self {
            Self::Brak => 0.0,
            Self::Podstawowe => 250.0,
            Self::Podwyzszone => 300.0,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code, unused)]
pub enum StawkaPodatku {
    Dwanascie,
    TrzydziesciDwa,
}

impl StawkaPodatku {
    pub fn wartosc(self) -> f32 {
        match self {
            Self::Dwanascie => 0.12,
            Self::TrzydziesciDwa => 0.32,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code, unused)]
pub enum UlgaPodatkowa {
    Brak,
    U300,
    U150,
    U100,
}

impl UlgaPodatkowa {
    pub fn wartosc(self) -> f32 {
        match self {
            Self::Brak => 0.0,
            Self::U300 => 300.0,
            Self::U150 => 150.0,
            Self::U100 => 100.0,
        }
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code, unused)]
pub struct ParametryNaliczania {
    pub brutto_zus_zdr_pod: f32,
    pub brutto_zdr_pod: f32,
    pub brutto_pod: f32,
    pub brutto_netto: f32,
    pub brutto_nie_zus_zdr_pod: f32,
    pub brutto_nie_zdr_pod: f32,
    pub brutto_nie_pod: f32,
    pub brutto_nie_netto: f32,
    pub potr_dod: f32,
    pub pod_zwol: bool,
    pub kup: KosztyUzyskania,
    pub pod_proc: StawkaPodatku,
    pub ulga: UlgaPodatkowa,
}

impl ParametryNaliczania {
    pub fn minimalne() -> Self {
        Self {
            brutto_zus_zdr_pod: 4666.0,
            brutto_zdr_pod: 0.0,
            brutto_pod: 0.0,
            brutto_netto: 0.0,
            brutto_nie_zus_zdr_pod: 0.0,
            brutto_nie_zdr_pod: 0.0,
            brutto_nie_pod: 0.0,
            brutto_nie_netto: 0.0,
            potr_dod: 0.0,
            pod_zwol: false,
            kup: KosztyUzyskania::Podwyzszone,
            pod_proc: StawkaPodatku::Dwanascie,
            ulga: UlgaPodatkowa::U300,
        }
    }
}
