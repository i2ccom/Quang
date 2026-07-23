//! GoalProgress — a reusable animated progress bar for goals.
//!
//! Renders a horizontal progress bar with color transitions based on
//! the completion percentage. Supports compact and full display modes.

use dioxus::prelude::*;

/// Props for the GoalProgress component.
#[derive(Clone, PartialEq, Props)]
pub struct GoalProgressProps {
    /// Progress value from 0.0 to 1.0
    pub progress: f64,
    /// Optional label override (e.g., "65%")
    pub label: Option<String>,
    /// Compact mode (smaller bar, no label) vs. full mode
    pub compact: Option<bool>,
    /// Optional custom color override
    pub color: Option<String>,
}

/// An animated progress bar for goal tracking.
#[component]
pub fn GoalProgress(props: GoalProgressProps) -> Element {
    let pct = (props.progress * 100.0).clamp(0.0, 100.0);
    let is_compact = props.compact.unwrap_or(false);

    let bar_color = props.color.clone().unwrap_or_else(|| {
        if pct >= 100.0 {
            "#6c5ce7".to_string() // completed
        } else if pct >= 75.0 {
            "#00b894".to_string() // on track
        } else if pct >= 50.0 {
            "#fdcb6e".to_string() // at risk
        } else {
            "#e17055".to_string() // behind
        }
    });

    let display_label = props
        .label
        .clone()
        .unwrap_or_else(|| format!("{:.0}%", pct));

    if is_compact {
        rsx! {
            div {
                class: "goal-progress-compact",
                style: "
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                ",
                div {
                    style: "
                        flex: 1;
                        height: 6px;
                        background: var(--q-bg, #0f0f1a);
                        border-radius: 3px;
                        overflow: hidden;
                    ",
                    div {
                        style: "
                            height: 100%;
                            width: {pct.to_string()}%;
                            background: {bar_color};
                            border-radius: 3px;
                            transition: width 0.5s ease;
                        "
                    }
                }
                span {
                    style: "
                        font-size: 0.75rem;
                        font-weight: 500;
                        color: {bar_color};
                        min-width: 35px;
                        text-align: right;
                    ",
                    "{display_label}"
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "goal-progress",
                style: "
                    width: 100%;
                ",

                // Label row
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        font-size: 0.8rem;
                        margin-bottom: 0.35rem;
                        color: var(--q-text-secondary, #888);
                    ",
                    span { "Progress" }
                    span {
                        style: "
                            font-weight: 600;
                            color: {bar_color};
                        ",
                        "{display_label}"
                    }
                }

                // Bar track
                div {
                    style: "
                        height: 10px;
                        background: var(--q-bg, #0f0f1a);
                        border-radius: 5px;
                        overflow: hidden;
                        position: relative;
                    ",
                    // Filled portion
                    div {
                        style: "
                            height: 100%;
                            width: {pct.to_string()}%;
                            background: linear-gradient(90deg, {bar_color}cc, {bar_color});
                            border-radius: 5px;
                            transition: width 0.6s cubic-bezier(0.4, 0, 0.2, 1);
                            position: relative;
                        ",
                        // Shimmer effect
                        div {
                            style: "
                                position: absolute;
                                inset: 0;
                                background: linear-gradient(
                                    90deg,
                                    transparent,
                                    rgba(255, 255, 255, 0.15),
                                    transparent
                                );
                                animation: shimmer 2s infinite;
                            "
                        }
                    }
                }
            }
        }
    }
}
