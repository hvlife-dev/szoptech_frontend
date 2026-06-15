use crate::app::components::footer::Footer;
use crate::app::components::toaster::{ToastData, Toaster};
use crate::app::hooks::use_engine::use_engine;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::components::Outlet;

#[component]
pub fn Navbar() -> impl IntoView {
    let engine = use_engine();
    let (toasts, set_toasts) = signal(Vec::<ToastData>::new());
    let last_toast_id = RwSignal::new(None::<usize>);

    Effect::new(move |_| {
        if let Some(Ok(data)) = engine.status.get() {
            let current_id = last_toast_id.get_untracked();
            match current_id {
                None => {
                    last_toast_id.set(Some(data.toast_counter));
                }
                Some(current_id) => {
                    if data.toast_counter > current_id {
                        last_toast_id.set(Some(data.toast_counter));
                        if let Some((level, msg)) = data.latest_toast.clone()
                            && level > 0
                        {
                            let new_toast = ToastData {
                                id: data.toast_counter,
                                level,
                                message: msg,
                            };
                            set_toasts.update(|t| t.push(new_toast.clone()));
                            let id_to_remove = new_toast.id;
                            set_timeout(
                                move || {
                                    set_toasts.update(|t| t.retain(|x| x.id != id_to_remove));
                                },
                                std::time::Duration::from_millis(3500),
                            );
                        }
                    }
                }
            }
        }
    });

    view! {
        <div class="app-container">
            <header class="top-navbar">
                <div class="utility-bar">
                    <div class="utility-container">
                        <div class="utility-left">
                            <a href="tel:+48123456789" class="utility-item">
                                <svg xmlns="http://www.w3.org/2000/svg" class="utility-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path>
                                </svg>
                                <span>"+48 664 124 850"</span>
                            </a>
                            <a href="mailto:serwis@szoptech.pl" class="utility-item">
                                <svg xmlns="http://www.w3.org/2000/svg" class="utility-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                    <polyline points="22,6 12,13 2,6"></polyline>
                                </svg>
                                <span>"serwis@szoptech.com"</span>
                            </a>
                        </div>

                        <div class="utility-right">
                            <a href="https://instagram.com/szoptech" target="_blank" rel="noopener noreferrer" class="social-icon" aria-label="Instagram">
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <rect x="2" y="2" width="20" height="20" rx="5" ry="5"></rect>
                                    <path d="M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z"></path>
                                    <line x1="17.5" y1="6.5" x2="17.51" y2="6.5"></line>
                                </svg>
                                "_@szoptech"
                            </a>
                        </div>
                    </div>
                </div>
                // ---------------------------

                <div class="nav-container">
                    <div class="brand">
                        <A href="/" attr:class="brand-text">"SzopTech"</A>
                    </div>

                    <nav class="nav-links-container">
                        <A href="/" attr:class="nav-link" exact=true>
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path>
                                <polyline points="9 22 9 12 15 12 15 22"></polyline>
                            </svg>
                            <span>"Główna"</span>
                        </A>

                        <A href="/services" attr:class="nav-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="3"></circle>
                                <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path>
                            </svg>
                            <span>"Usługi"</span>
                        </A>

                        <A href="/projects" attr:class="nav-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline>
                            </svg>
                            <span>"Realizacje"</span>
                        </A>

                        <A href="/pricing" attr:class="nav-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"></path>
                                <line x1="7" y1="7" x2="7.01" y2="7"></line>
                            </svg>
                            <span>"Wyceny"</span>
                        </A>

                        <A href="/contact" attr:class="nav-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"></path>
                                <polyline points="22,6 12,13 2,6"></polyline>
                            </svg>
                            <span>"Kontakt"</span>
                        </A>

                        <A href="/shopping" attr:class="nav-link">
                            <svg xmlns="http://www.w3.org/2000/svg" class="nav-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="9" cy="21" r="1"></circle>
                                <circle cx="20" cy="21" r="1"></circle>
                                <path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"></path>
                            </svg>
                            <span>"Zakupy"</span>
                        </A>
                    </nav>
                </div>
            </header>

            <main class="main-content">
                <div class="content-wrapper">
                    <Outlet/>
                </div>
                <Footer />
            </main>

            <Toaster toasts=toasts />
        </div>
    }
}
