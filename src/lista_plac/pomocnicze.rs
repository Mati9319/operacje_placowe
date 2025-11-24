pub fn zaokr(x: f32) -> f32 {
    (x * 100.0).round() / 100.0
}

pub fn dlugosc(liczba: f32) -> u8 {
    liczba
        .floor()
        .to_string()
        .chars()
        .count()
        .try_into()
        .unwrap()
}

pub fn nowa_najwieksza_liczba(liczba: f32, najwieksza_liczba: u8) -> u8 {
    if dlugosc(liczba) > najwieksza_liczba {
        dlugosc(liczba)
    } else {
        najwieksza_liczba
    }
}

pub fn przesuniecie(liczba: f32, najwieksza_liczba: u8) -> (String, String) {
    let dlugosc_liczby: u8 = dlugosc(liczba);
    let liczba_miejsc: u8 = najwieksza_liczba - dlugosc_liczby;
    let mut miejsca: String = String::new();
    let mut ramka: String = String::new();
    for _i in 0..liczba_miejsc {
        miejsca += " ";
        ramka += "-";
    }
    (miejsca, ramka)
}
