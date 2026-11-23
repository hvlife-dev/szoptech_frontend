use leptos::svg::view;
use leptos::{IntoView, component, view};
use leptos::prelude::*;

#[component]
pub fn SpecTable(
    specification: Vec<(String, String)>
) -> impl IntoView {
    view! {
        <table class="spec-table">
            <tbody>
                {specification
                    .iter()
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
                    .collect_view()}
            </tbody>
        </table>
    }
}