mod lista_plac;
use lista_plac::*;

fn main() {
    let mut x = ListaPlac::nalicz(ParametryNaliczania::minimalne());
    x.wyswietl();

    x.dane_wejsciowe.pod_zwol = true;
    x.przelicz_i_wyswietl();

    let y = ListaPlac::nalicz(ParametryNaliczania {
        brutto_zus_zdr_pod: 0.0,
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
    });
    y.wyswietl();
}
