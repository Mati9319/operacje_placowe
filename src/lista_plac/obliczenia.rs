use super::ParametryNaliczania;
use super::pomocnicze::*;

#[allow(dead_code, unused)]
pub struct ListaPlac {
    pub dane_wejsciowe: ParametryNaliczania,

    pub brutto_cal: f32,
    pub brutto_wyp: f32,
    pub pd_zus: f32,
    pub pd_zdr: f32,
    pub pd_pod: f32,
    pub zus_emerytalna: f32,
    pub zus_rentowa: f32,
    pub zus_chorobowa: f32,
    pub zus: f32,
    pub zdrowotna: f32,
    pub podatek: f32,
    pub netto: f32,
    pub najwieksza_liczba: u8,
}

impl ListaPlac {
    pub fn nalicz(dane_wejsciowe: ParametryNaliczania) -> Self {
        let mut najwieksza_liczba: u8 = 0;

        let kup = dane_wejsciowe.kup.wartosc();
        let pod_proc = dane_wejsciowe.pod_proc.wartosc();
        let ulga = dane_wejsciowe.ulga.wartosc();

        let brutto_cal = zaokr(
            dane_wejsciowe.brutto_zus_zdr_pod
                + dane_wejsciowe.brutto_zdr_pod
                + dane_wejsciowe.brutto_pod
                + dane_wejsciowe.brutto_netto
                + dane_wejsciowe.brutto_nie_zus_zdr_pod
                + dane_wejsciowe.brutto_nie_zdr_pod
                + dane_wejsciowe.brutto_nie_pod
                + dane_wejsciowe.brutto_nie_netto,
        );

        najwieksza_liczba = nowa_najwieksza_liczba(brutto_cal, najwieksza_liczba);

        let brutto_wyp = zaokr(
            dane_wejsciowe.brutto_zus_zdr_pod
                + dane_wejsciowe.brutto_zdr_pod
                + dane_wejsciowe.brutto_pod
                + dane_wejsciowe.brutto_netto,
        );

        najwieksza_liczba = nowa_najwieksza_liczba(brutto_wyp, najwieksza_liczba);

        let pd_zus =
            zaokr(dane_wejsciowe.brutto_zus_zdr_pod + dane_wejsciowe.brutto_nie_zus_zdr_pod);
        let pd_zdr = zaokr(
            dane_wejsciowe.brutto_zus_zdr_pod
                + dane_wejsciowe.brutto_zdr_pod
                + dane_wejsciowe.brutto_nie_zus_zdr_pod
                + dane_wejsciowe.brutto_nie_zdr_pod,
        );

        let pd_pod = zaokr(
            dane_wejsciowe.brutto_zus_zdr_pod
                + dane_wejsciowe.brutto_zdr_pod
                + dane_wejsciowe.brutto_pod
                + dane_wejsciowe.brutto_nie_zus_zdr_pod
                + dane_wejsciowe.brutto_nie_zdr_pod
                + dane_wejsciowe.brutto_nie_pod,
        );

        let zus_emerytalna = zaokr(pd_zus * 0.0976);
        najwieksza_liczba = nowa_najwieksza_liczba(zus_emerytalna, najwieksza_liczba);

        let zus_rentowa = zaokr(pd_zus * 0.015);
        najwieksza_liczba = nowa_najwieksza_liczba(zus_rentowa, najwieksza_liczba);

        let zus_chorobowa = zaokr(pd_zus * 0.0245);
        najwieksza_liczba = nowa_najwieksza_liczba(zus_chorobowa, najwieksza_liczba);

        let zus = zaokr(zus_emerytalna + zus_rentowa + zus_chorobowa);

        let zdrowotna = zaokr((pd_zdr - zus) * 0.09);
        najwieksza_liczba = nowa_najwieksza_liczba(zdrowotna, najwieksza_liczba);

        let mut podatek = if dane_wejsciowe.pod_zwol {
            0.0
        } else {
            ((pd_pod - zus - kup).round() * pod_proc - ulga).round()
        };

        if podatek < 0.0 {
            podatek = 0.0;
        }

        najwieksza_liczba = nowa_najwieksza_liczba(podatek, najwieksza_liczba);

        let netto = zaokr(brutto_wyp - zus - zdrowotna - podatek - dane_wejsciowe.potr_dod);
        najwieksza_liczba = nowa_najwieksza_liczba(netto, najwieksza_liczba);

        Self {
            dane_wejsciowe,
            brutto_cal,
            brutto_wyp,
            pd_zus,
            pd_zdr,
            pd_pod,
            zus_emerytalna,
            zus_rentowa,
            zus_chorobowa,
            zus,
            zdrowotna,
            podatek,
            netto,
            najwieksza_liczba,
        }
    }

    pub fn przelicz(&mut self) {
        *self = Self::nalicz(self.dane_wejsciowe);
    }

    pub fn wyswietl(&self) {
        println!(
            "
--------------------------------------------{}
Całkowite wynagrodzenie brutto:      {}{:.2} zł
Wypłacane wynagrodzenie brutto:      {}{:.2} zł

Składka na ubezpieczenie emerytalne: {}{:.2} zł
Składka na ubezpieczenie rentowe:    {}{:.2} zł
Składka na ubezpieczenie chorobowe:  {}{:.2} zł

Składka na ubezpieczenie zdrowotne:  {}{:.2} zł

Zaliczka na podatek dochodowy:       {}{:.2} zł

Wypłacane wynagrodzenie netto:       {}{:.2} zł
--------------------------------------------{}
",
            
            przesuniecie(0.0, self.najwieksza_liczba).1,
            przesuniecie(self.brutto_cal, self.najwieksza_liczba).0,
            self.brutto_cal,
            przesuniecie(self.brutto_wyp, self.najwieksza_liczba).0,
            self.brutto_wyp,
            przesuniecie(self.zus_emerytalna, self.najwieksza_liczba).0,
            self.zus_emerytalna,
            przesuniecie(self.zus_rentowa, self.najwieksza_liczba).0,
            self.zus_rentowa,
            przesuniecie(self.zus_chorobowa, self.najwieksza_liczba).0,
            self.zus_chorobowa,
            przesuniecie(self.zdrowotna, self.najwieksza_liczba).0,
            self.zdrowotna,
            przesuniecie(self.podatek, self.najwieksza_liczba).0,
            self.podatek,
            przesuniecie(self.netto, self.najwieksza_liczba).0,
            self.netto,
            przesuniecie(0.0, self.najwieksza_liczba).1,
        );
    }

    pub fn przelicz_i_wyswietl(&mut self) {
        self.przelicz();
        self.wyswietl();
    }
}
