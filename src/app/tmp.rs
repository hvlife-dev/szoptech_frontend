// @oskif imports
use leptos_meta::*;
use std::collections::HashMap;

#[component]
pub fn SeoMetaTags() {

    // @oskif
    let mut meta_titles: HashMap<&str, &str> = HashMap::from([
        ("", "Szoptech - Elektronika Przemysłowa & Automatyka"),
        ("services", "Szoptech - Nasze Usługi"),
        ("projects", "Szoptech - Realizacje"),
        ("pricing", "Szoptech - Konkretne Wyceny"),
        ("contact", "Szoptech - Kontakt"),
        ("shopping", "Szoptech - Zakupy")
    ]);

    // @oskif
    let mut meta_descritpions = HashMap::from([
        ("",         "Naprawiamy od lat a z elektroniką lubimy się całe życie. Jesteśmy małą grupą ludzi, która po prostu kocha to co robi. Niesamowicie cieszy nas, że możemy z tego żyć."),
        ("services", "Elektronika, PLC, HMI, falowniki, zasilacze, komputery, enkodery, Maszyny Przemysłowe, CNC"),
        ("projects", "NAPRAWA PŁYTY GŁÓWNEJ CHŁODZIARKI WRZECIONA"),
        ("pricing",  "Cena gwarantowana niezależnie od stanu urządzenia. Roczna gwarancja na wszystkie wymienione naprawy."),
        ("contact",  "Rozmawiacie Państwo tylko i wyłącznie z osobami technicznymi. Nie istnieje u nas coś takiego jak dział realizacji."),
        ("shopping", "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa. Dostawy na terenie całego Śląska realizujemy osobiście nawet tego samego dnia bez dodatkowych kosztów.")
    ]);

    let pathname = use_location().pathname.get();

}