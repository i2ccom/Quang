//! Avatar component — displays a user or agent round icon.

use dioxus::prelude::*;

#[component]
pub fn Avatar(
    src: Option<String>,
    name: String,
    size: Option<String>,
    is_agent: Option<bool>,
) -> Element {
    let size_val = size.unwrap_or_else(|| "32px".to_string());
    let is_agent_val = is_agent.unwrap_or(false);
    let initial = name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    if let Some(url) = src {
        rsx! {
            Fragment {
                img {
                    src: "{url}",
                    alt: "{name}",
                    width: "{size_val}",
                    height: "{size_val}",
                    style: "border-radius: 50%; object-fit: cover; border: 2px solid var(--q-border);",
                }
            }
        }
    } else {
        let border_color = if is_agent_val {
            "var(--q-primary)"
        } else {
            "var(--q-border)"
        };
        rsx! {
            div {
                style: "width: {size_val}; height: {size_val}; border-radius: 50%; background: var(--q-surface); border: 2px solid {border_color}; display: flex; align-items: center; justify-content: center; font-size: calc({size_val} * 0.4); font-weight: 600; color: var(--q-text-secondary);",
                "{initial}"
            }
        }
    }
}
