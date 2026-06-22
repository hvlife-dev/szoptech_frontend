use chrono::{Datelike, Utc};
use leptos::prelude::*;

// #[component]
// pub fn Footer() -> impl IntoView {
//     let year = Utc::now().year();

//     view! {
//         <footer class="app-footer">
//             <div class="footer-left"/>
//             <div class="footer-center">
//                 <span>"SzopTech ™ " {year}</span>
//             </div>
//             <div class="footer-right"/>
//         </footer>
//     }
// }

// @oskif

#[component]
pub fn Footer() -> impl IntoView {
    let year = Utc::now().year();

    view! {
        <footer class="app-footer">
            <div>
                <h3>"SzopTech ™ " {year}</h3>
                <p>"Elektronika przemysłowa, automatyka, serwis PLC, HMI i CNC."</p>
            </div>
            <div>
            <h3>Usługi</h3>
                <ul>
                    <li>"Naprawa elektroniki przemysłowej"</li>
                    <li>"Programowanie PLC"</li>
                    <li>"Serwis HMI"</li>
                    <li>"Diagnostyka maszyn CNC"</li>
                </ul>
            </div>
            <div>
                <h3>"Kontakt"</h3>
                <p>"Śląsk"</p>
                <p>"Email: kontakt@szoptech.com"</p>
                <p>"Telefon: +48 664 124 850"</p>
            </div>
            <div>
                <h3>
                    "Obsługujemy firmy na terenie Śląska:"
                    <br/>                    
                    "Rybnik, Katowice, Gliwice, Żory, Tychy i okolice."
                </h3>
            </div>
        </footer>
    }
}
