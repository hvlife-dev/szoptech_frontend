use leptos::{IntoView, component, view};
use leptos::prelude::*;

#[component]
pub fn SpecTable(
    specification: Vec<(String, String)>,
) -> impl IntoView {
    let spec_len = specification.len();
    let (is_shown, set_is_shown) = signal(false);

    view! {
        <>
            <table class="spec-table">
                <tbody>
                    {move || {
                        let limit = if is_shown.get() || spec_len <= 5 {
                            specification.len()
                        } else {
                            5
                        };
                        specification
                            .iter()
                            .take(limit)
                            .map(|(key, value)| {
                                let key_formated = if key.ends_with(':') {
                                    key.to_string()
                                } else {
                                    format!("{}:", key)
                                };

                                view! {
                                    <tr class="spec-row">
                                        <td class="spec-key">{key_formated}</td>
                                        <td class="spec-value">{value.clone()}</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                    }}
                </tbody>
            </table>

            {move || {
                if spec_len > 5 {
                    let expanded = is_shown.get();
                    view! {
                        <div class="show-more-wrapper">
                            <button
                                class="show-more"
                                on:click=move |_| { set_is_shown.set(!expanded) }
                            >
                                <span>
                                    {if expanded { "Pokaż mniej" } else { "Pokaż więcej" }}
                                </span>
                            </button>
                        </div>
                    }
                        .into_any()
                } else {
                    view! { <></> }.into_any()
                }
            }}
        </>
    }
}