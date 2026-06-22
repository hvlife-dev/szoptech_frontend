// @oskif imports

use leptos_router::hooks::use_location;
use leptos_meta::*;
use leptos::*;
use leptos_router::*;
use leptos::prelude::Get;
pub struct  MetaTitles;

impl MetaTitles {
    pub const HOME: &'static str = "Szoptech - Elektronika Przemysłowa & Automatyka";
    pub const SERVICES: &'static str = "Szoptech - Nasze Usługi";
    pub const PROJECTS: &'static str = "Szoptech - Realizacje";
    pub const PRICING: &'static str = "Szoptech - Konkretne Wyceny";
    pub const CONTACT: &'static str = "Szoptech - Kontakt";
    pub const SHOPPING: &'static str = "Szoptech - Zakupy";
}

pub struct MetaDescriptions;

impl MetaDescriptions {
    pub const HOME: &'static str = "Naprawiamy od lat a z elektroniką lubimy się całe życie. Jesteśmy małą grupą ludzi, która po prostu kocha to co robi. Niesamowicie cieszy nas, że możemy z tego żyć";
    pub const SERVICES: &'static str = "Elektronika przemysłowa: PLC, HMI, falowniki, zasilacze i CNC. Profesjonalne naprawy, diagnostyka źródła problemu, programowanie i serwis maszyn.";
    pub const PROJECTS: &'static str = "Realizacje";
    pub const PRICING: &'static str = "Cena gwarantowana niezależnie od stanu urządzenia. Roczna gwarancja na wszystkie wymienione naprawy";
    pub const CONTACT: &'static str = "Rozmawiacie Państwo tylko i wyłącznie z osobami technicznymi. Nie istnieje u nas coś takiego jak dział rea4lizacji.";
    pub const SHOPPING:&'static str = "Wysyłka urządzeń kupionych od nas bezpośrednio jest darmowa. Dostawy na terenie całego Śląska realizujemy osobiście nawet tego samego dnia bez dodatkowych kosztów.";
}

fn get_meta_title(pathname: &str) -> &'static str {
    match pathname {
        "/services" => MetaTitles::SERVICES,
        "/projects" => MetaTitles::PROJECTS,
        "/pricing" => MetaTitles::PRICING,
        "/contact" => MetaTitles::CONTACT,
        "/shopping" => MetaTitles::SHOPPING,
        _ => MetaTitles::HOME
    }
}

fn get_meta_description(pathname: &str) -> &'static str {
    match pathname {
        "/services" => MetaDescriptions::SERVICES,
        "/projects" => MetaDescriptions::PROJECTS,
        "/pricing" => MetaDescriptions::PRICING,
        "/contact" => MetaDescriptions::CONTACT,
        "/shopping" => MetaDescriptions::SHOPPING,
        _ => MetaDescriptions::HOME,
    }
}


#[component]
pub fn TitleSelector() -> impl IntoView {
    let location = use_location();

    let title = move || get_meta_title(&location.pathname.get());
    view! {
        <Title text=title />
    }
}

#[component]
pub fn SeoMetaTags() -> impl IntoView {
    let location = use_location();

    let title = move || get_meta_title(&location.pathname.get());
    
    let description = move || get_meta_description(&location.pathname.get());
    
    view! {
        <Title text=title />
        <Meta property="og:title" content=title />
        <Meta property="og:description" content=description />
        <Meta name="description" content=description />
    }
}