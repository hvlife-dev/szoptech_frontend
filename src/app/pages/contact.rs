use crate::app::{components::divider::Divider, metatags::seomt::SeoMetaTags};
use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        // @oskif
        <SeoMetaTags />

        <div class="page-container">
            <div class="contact-hero">
                <h1 class="hero-title">"Kontakt"</h1>
                <p class="hero-subtitle">
                    "Rozmawiacie Państwo tylko i wyłącznie z osobami technicznymi."
                    <br/>
                    "Nie istnieje u nas coś takiego jak dział realizacji."
                </p>
            </div>

            <Divider />

            <div class="contact-grid">
                <div class="card contact-info-card">
                    <div class="info-group">
                        <label>"Adres Siedziby"</label>
                        <p class="info-value">
                            "ul. Głożyńska 239a"<br/>
                            "44-310 Radlin, Polska"<br/>
                            "NIP: PL6472613339"
                        </p>
                    </div>

                    <div class="info-group">
                        <label>"Komunikacja 24h/7"</label>
                        <p class="info-value">
                            <span class="keyword">"tel: "</span> "+48 664 124 850" <br/>
                            <span class="keyword">"email: "</span> "serwis@szoptech.com" <br/>
                            <span class="keyword">"WhatsApp: "</span> "+48 664 124 850" <br/>
                        </p>
                    </div>

                    <div class="info-group">
                        <span class="info-label">"Jesteśmy do Państwa dyspozycji o każdej porze."</span>
                    </div>

                    <div class="panel-divider"></div>

                    <p style="font-size: 0.85rem; color: var(--color-fg-muted); font-family: var(--font-mono);">
                        "Przed przyjazdem osobistym prosimy o kontakt telefoniczny."
                    </p>
                </div>

                <div class="card map-card">
                    <iframe
                        src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d333.56544676469144!2d18.437891029661493!3d50.039069088560026!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x47114555a4e12851%3A0x4341456fed503387!2zR8WCb8W8ecWEc2thIDI0MSwgNDQtMzEwIFJhZGxpbg!5e0!3m2!1spl!2spl!4v1777313206837!5m2!1spl!2spl"
                        width="100%"
                        height="100%"
                        style="border:0;"
                        allowfullscreen=""
                        loading="lazy"
                        referrerpolicy="no-referrer-when-downgrade">
                    </iframe>
                </div>
            </div>
        </div>
    }
}
