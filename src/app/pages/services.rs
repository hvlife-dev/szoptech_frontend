use leptos::prelude::*;
use crate::app::components::divider::Divider;

#[component]
pub fn ServicesPage() -> impl IntoView {
    view! {
        <div class="page-container">

            <div class="t-grid">
                <div class="card">
                    <h3 class="service-title">"Elektronika, PLC, HMI, falowniki, zasilacze, komputery, enkodery i wiele więcej!"</h3>
                    <p class="service-desc">"Profesjonalne podejście do każdej naprawy. Nie zajmujemy się tylko wymianą uszkodzonych elementów. Znajdujemy źródło problemu, usuwamy je, testujemy i wtedy dopiero mamy pewność, że urządzenie wróciło do pełnej sprawności. Cała naprawiana przez nas elektronika, która pracuje w ciężkich warunkach, maszynach CNC lub jest narażona na duże zapylenie pyłem stalowym jest zabezpieczana. Przedłuża to żywotność urządzeń i niweluje przyszłościowe problemy. Podczas każdej naprawy urządzenia są myte i wracają do Państwa w jak najlepszym stanie wizualnym, zależnym od stopnia zużycia obudów oraz ich dostępności. Niedostępne części do urządzeń oraz obudowy jesteśmy w stanie wykonywać sami, umożliwiając tym naprawę jeszcze większą ilość urządzeń."</p>
                </div>

                <div class="card">
                    <h3 class="service-title">"Programowanie Sterowników PLC oraz Paneli HMI"</h3>
                    <p class="service-desc">"Posiadamy wiele lat doświadczenia w uruchamianiu maszyn przemysłowych pod wytyczne klienta. Od fizycznych modyfikacji szaf sterujących, przez tworzenie oprogramowania, aż po wizualizację."</p>
                </div>

                <div class="card">
                    <h3 class="service-title">"Maszyny Przemysłowe, Obrabiarki CNC i Utrzymanie Ruchu"</h3>
                    <p class="service-desc">"Dojazd do klienta mobilnym warsztatem w pełni wyposażonym do napraw na miejscu. Jeżeli w swoich zasobach mamy urządzenie, które akurat u Państwa nie działa, wymieniamy go od razu na sprawne. Jeżeli takiego nie posiadamy, chętnie zaczniemy naprawę już na miejscu. Przyjmujemy uszkodzone urządzenia w rozliczeniu."</p>
                </div>
            </div>

            // <Divider text="Procedury Serwisowe"/>
            //
            // <div class="workflow-section">
            //     <div class="card">
            //         <h3>"Tryb Napraw"</h3>
            //         <ul class="technical-list">
            //             <li><span class="keyword">"Tryb Standardowy:"</span> " Czas realizacji 5-10 dni roboczych."</li>
            //             <li><span class="keyword">"Pogotowie 24/7 (Ciągłość Produkcji):"</span> " Natychmiastowy wyjazd mobilnego warsztatu lub ewaluacja sprzętu na stole w 24 godziny."</li>
            //             <li><span class="keyword">"Weryfikacja:"</span> " Dynamiczne testy obciążeniowe i weryfikacja sygnałów przed oddaniem maszyny/sprzętu."</li>
            //             <li><span class="keyword">"Gwarancja:"</span> " Każda naprawa objęta jest 6-miesięczną gwarancją poprawnego działania."</li>
            //         </ul>
            //     </div>
            // </div>
        </div>
    }
}
