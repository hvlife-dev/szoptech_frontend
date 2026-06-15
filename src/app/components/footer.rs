use chrono::{Datelike, Utc};
use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    let year = Utc::now().year();

    view! {
        <footer class="app-footer">
            <div class="footer-left"/>
            <div class="footer-center">
                <span>"SzopTech ™ " {year}</span>
            </div>
            <div class="footer-right"/>
        </footer>
    }
}
