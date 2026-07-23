//! Welcome page — the landing page for QuangHub Workplace.
//!
//! Features a hero section with QuangHub branding, a tagline about
//! agent-native collaboration, and OAuth login buttons (Google, GitHub).

use dioxus::prelude::*;

/// The CSS variable prefix used throughout QuangHub UI.
const CSS_VAR: &str = "--q";

/// Welcome page component — hero, branding, and login entry points.
#[component]
pub fn Welcome() -> Element {
    rsx! {
        div {
            class: "welcome-page",
            style: "
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                min-height: 100vh;
                background: var({CSS_VAR}-bg, #0f0f1a);
                color: var({CSS_VAR}-text, #e0e0e0);
                font-family: 'Inter', system-ui, -apple-system, sans-serif;
                padding: 2rem;
            ",

            // ── Hero Section ──
            div {
                class: "hero",
                style: "
                    text-align: center;
                    max-width: 720px;
                    animation: fadeIn 0.8s ease-out;
                ",

                // Logo / Brand mark
                div {
                    class: "brand-logo",
                    style: "
                        font-size: 3.5rem;
                        font-weight: 800;
                        background: linear-gradient(135deg, var({CSS_VAR}-primary, #6c5ce7), var({CSS_VAR}-accent, #00cec9));
                        -webkit-background-clip: text;
                        -webkit-text-fill-color: transparent;
                        background-clip: text;
                        margin-bottom: 0.5rem;
                        letter-spacing: -0.03em;
                    ",
                    "QuangHub"
                }

                // Tagline
                p {
                    class: "tagline",
                    style: "
                        font-size: 1.25rem;
                        color: var({CSS_VAR}-text-secondary, #888);
                        margin-bottom: 0.25rem;
                        font-weight: 400;
                    ",
                    "The collaboration graph for humans and AI agents"
                }

                p {
                    class: "subtitle",
                    style: "
                        font-size: 1rem;
                        color: var({CSS_VAR}-text-muted, #666);
                        margin-bottom: 2.5rem;
                        line-height: 1.6;
                    ",
                    "Work together. Ship faster. Let agents handle the rest."
                }

                // ── Login Buttons ──
                div {
                    class: "login-buttons",
                    style: "
                        display: flex;
                        flex-direction: column;
                        gap: 0.75rem;
                        align-items: center;
                    ",

                    // Google Login
                    button {
                        class: "btn-google",
                        onclick: move |_| {
                            let _ = navigator().push("/login?provider=google");
                        },
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.75rem;
                            padding: 0.75rem 2rem;
                            border-radius: 8px;
                            border: 1px solid var({CSS_VAR}-surface-border, #333);
                            background: var({CSS_VAR}-surface, #1a1a2e);
                            color: var({CSS_VAR}-text, #e0e0e0);
                            font-size: 1rem;
                            font-weight: 500;
                            cursor: pointer;
                            transition: all 0.2s ease;
                            width: 280px;
                            justify-content: center;
                        ",
                        onmouseenter: move |_| {},
                        "Login with Google"
                    }

                    // GitHub Login
                    button {
                        class: "btn-github",
                        onclick: move |_| {
                            let _ = navigator().push("/login?provider=github");
                        },
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.75rem;
                            padding: 0.75rem 2rem;
                            border-radius: 8px;
                            border: 1px solid var({CSS_VAR}-surface-border, #333);
                            background: var({CSS_VAR}-surface, #1a1a2e);
                            color: var({CSS_VAR}-text, #e0e0e0);
                            font-size: 1rem;
                            font-weight: 500;
                            cursor: pointer;
                            transition: all 0.2s ease;
                            width: 280px;
                            justify-content: center;
                        ",
                        "Login with GitHub"
                    }
                }

                // ── Feature Highlights ──
                div {
                    class: "features",
                    style: "
                        display: grid;
                        grid-template-columns: repeat(3, 1fr);
                        gap: 1.5rem;
                        margin-top: 3.5rem;
                        text-align: left;
                    ",

                    FeatureCard {
                        icon: "⚡",
                        title: "Agent-Native",
                        description: "AI agents as first-class team members with capabilities, budgets, and trust scores."
                    },
                    FeatureCard {
                        icon: "📊",
                        title: "HyperGraph",
                        description: "Everything is a node. Views project your data as Kanban, Gantt, charts, or tables."
                    },
                    FeatureCard {
                        icon: "🔗",
                        title: "Event-Driven",
                        description: "Every action emits an event. Real-time sync across humans, agents, and external tools."
                    }
                }

                // ── Footer ──
                div {
                    class: "footer",
                    style: "
                        margin-top: 3rem;
                        font-size: 0.8rem;
                        color: var({CSS_VAR}-text-muted, #555);
                    ",
                    span { "© 2026 QuangHub — Open Agent-Native Platform" }
                }
            }
        }
    }
}

/// A small feature highlight card used on the welcome page.
#[component]
fn FeatureCard(icon: String, title: String, description: String) -> Element {
    rsx! {
        div {
            class: "feature-card",
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 12px;
                padding: 1.25rem;
                transition: transform 0.2s ease, box-shadow 0.2s ease;
            ",
            div {
                style: "font-size: 1.75rem; margin-bottom: 0.5rem;",
                "{icon}"
            }
            h3 {
                style: "
                    font-size: 1rem;
                    font-weight: 600;
                    margin: 0 0 0.35rem 0;
                    color: var(--q-text, #e0e0e0);
                ",
                "{title}"
            }
            p {
                style: "
                    font-size: 0.85rem;
                    color: var(--q-text-secondary, #888);
                    margin: 0;
                    line-height: 1.5;
                ",
                "{description}"
            }
        }
    }
}
