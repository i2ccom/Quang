//! LoadingSpinner — shared loading indicator.

use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(label: Option<String>) -> Element {
    let label = label.unwrap_or_else(|| "Loading...".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 48px; gap: 16px;",
            div {
                style: "width: 32px; height: 32px; border: 3px solid var(--q-border); border-top-color: var(--q-primary); border-radius: 50%; animation: q-spin 0.8s linear infinite;",
            }
            span { style: "color: var(--q-text-secondary); font-size: 14px;", "{label}" }
        }
    }
}
