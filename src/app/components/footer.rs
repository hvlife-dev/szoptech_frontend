use chrono::{Datelike, Utc};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let year = Utc::now().year();

    view! {
        <footer class="app-footer">
            <div class="footer-left">
                <a href="/services" class="footer-link">"Usługi"</a>
                <span class="divider">"·"</span>
                <a href="/projects" class="footer-link">"Realizacje"</a>
                <span class="divider">"·"</span>
                <a href="/pricing" class="footer-link">"Wycena"</a>
                <span class="divider">"·"</span>
                <a href="/shopping" class="footer-link">"Zakupy"</a>
            </div>

            <div class="footer-center">
                <span>"SzopTech ™ " {year}</span>
            </div>

            <div class="footer-right">
                <a href="/contact" class="footer-link">"Kontakt"</a>
                <span class="divider">"·"</span>
                <a href="/sitemap.xml" class="footer-link" target="_blank">
                    "Sitemap"
                </a>
            </div>
        </footer>
    }
}
