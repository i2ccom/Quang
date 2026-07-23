//! AgentGoalPanel — an agent-assisted goal management panel.
//!
//! Shows AI-generated goal suggestions, auto-progress tracking, and
//! allows humans to assign goals to agents for automated tracking.

use dioxus::prelude::*;

use crate::web::components::goal_progress::GoalProgress;

/// A goal suggestion or tracked goal from the agent.
#[derive(Clone, PartialEq, Debug)]
pub struct AgentGoalItem {
    pub id: String,
    pub title: String,
    pub description: String,
    pub progress: f64,
    pub is_ai_generated: bool,
    pub confidence: Option<f64>,
}

/// Props for the AgentGoalPanel component.
#[derive(Clone, PartialEq, Props)]
pub struct AgentGoalPanelProps {
    pub goals: Vec<AgentGoalItem>,
    pub on_generate: EventHandler<String>,
    pub on_assign_to_agent: EventHandler<String>,
}

/// Panel for agent-assisted goal management.
#[component]
pub fn AgentGoalPanel(props: AgentGoalPanelProps) -> Element {
    let mut prompt = use_signal(|| String::new());
    let mut expanded_goal = use_signal(|| None::<String>);

    let handle_generate = move |_| {
        let text = prompt().trim().to_string();
        if !text.is_empty() {
            props.on_generate.call(text);
            prompt.set(String::new());
        }
    };

    rsx! {
        div {
            class: "agent-goal-panel",
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 12px;
                padding: 1.25rem;
                display: flex;
                flex-direction: column;
                gap: 1rem;
            ",

            // Header
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                ",
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    span { style: "font-size: 1.1rem;", "🎯" }
                    h2 {
                        style: "
                            font-size: 1rem;
                            font-weight: 600;
                            margin: 0;
                            color: var(--q-text, #e0e0e0);
                        ",
                        "Agent Goal Assistant"
                    }
                }
                span {
                    style: "
                        font-size: 0.75rem;
                        padding: 0.15rem 0.45rem;
                        border-radius: 4px;
                        background: rgba(0, 206, 201, 0.15);
                        color: var(--q-accent, #00cec9);
                        font-weight: 500;
                    ",
                    "AI-Powered"
                }
            }

            // AI prompt input
            div {
                style: "
                    display: flex;
                    gap: 0.5rem;
                ",
                input {
                    placeholder: "Describe a goal for the agent to track...",
                    value: prompt(),
                    oninput: move |e| prompt.set(e.value()),
                    style: "
                        flex: 1;
                        padding: 0.55rem 0.75rem;
                        border-radius: 8px;
                        border: 1px solid var(--q-surface-border, #333);
                        background: var(--q-bg, #0f0f1a);
                        color: var(--q-text, #e0e0e0);
                        font-size: 0.85rem;
                        outline: none;
                    "
                }
                button {
                    onclick: move |_| handle_generate(),
                    style: "
                        padding: 0.55rem 1rem;
                        border-radius: 8px;
                        border: none;
                        background: var(--q-accent, #00cec9);
                        color: #fff;
                        font-size: 0.85rem;
                        font-weight: 500;
                        cursor: pointer;
                        white-space: nowrap;
                    ",
                    "Generate"
                }
            }

            // Goal list
            div {
                style: "
                    display: flex;
                    flex-direction: column;
                    gap: 0.75rem;
                ",
                if props.goals.is_empty() {
                    div {
                        style: "
                            text-align: center;
                            padding: 2rem;
                            color: var(--q-text-muted, #555);
                            font-size: 0.85rem;
                        ",
                        "No goals yet. Describe a goal and let the AI assist."
                    }
                } else {
                    for goal in &props.goals {
                        let is_expanded = expanded_goal() == Some(goal.id.clone());
                        AgentGoalCard {
                            goal: goal.clone(),
                            is_expanded,
                            on_toggle: {
                                let id = goal.id.clone();
                                move || {
                                    if expanded_goal() == Some(id.clone()) {
                                        expanded_goal.set(None);
                                    } else {
                                        expanded_goal.set(Some(id.clone()));
                                    }
                                }
                            },
                            on_assign: {
                                let id = goal.id.clone();
                                move || props.on_assign_to_agent.call(id.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}

/// A single goal card in the agent goal panel.
#[component]
fn AgentGoalCard(
    goal: AgentGoalItem,
    is_expanded: bool,
    on_toggle: EventHandler<()>,
    on_assign: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "agent-goal-card",
            style: "
                background: var(--q-bg, #0f0f1a);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 8px;
                padding: 0.85rem;
                transition: all 0.2s;
            ",

            // Header
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    cursor: pointer;
                ",
                onclick: move |_| on_toggle.call(()),

                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    if goal.is_ai_generated {
                        span {
                            style: "
                                font-size: 0.65rem;
                                padding: 0.1rem 0.35rem;
                                border-radius: 3px;
                                background: rgba(0, 206, 201, 0.15);
                                color: var(--q-accent, #00cec9);
                                font-weight: 500;
                            ",
                            "AI"
                        }
                    }
                    span {
                        style: "
                            font-size: 0.85rem;
                            font-weight: 500;
                            color: var(--q-text, #e0e0e0);
                        ",
                        "{goal.title}"
                    }
                }

                div {
                    style: "
                        font-size: 0.85rem;
                        color: var(--q-text-muted, #555);
                    ",
                    if is_expanded { "▲" } else { "▼" }
                }
            }

            // Expanded content
            if is_expanded {
                div {
                    style: "
                        margin-top: 0.75rem;
                        display: flex;
                        flex-direction: column;
                        gap: 0.5rem;
                    ",
                    p {
                        style: "
                            font-size: 0.8rem;
                            color: var(--q-text-secondary, #888);
                            margin: 0;
                        ",
                        "{goal.description}"
                    }

                    // Progress
                    GoalProgress {
                        progress: goal.progress,
                        compact: true,
                    }

                    // Confidence (AI-generated)
                    if let Some(conf) = goal.confidence {
                        div {
                            style: "
                                font-size: 0.7rem;
                                color: var(--q-text-muted, #555);
                            ",
                            "Confidence: {conf:.0}%"
                        }
                    }

                    // Assign button
                    button {
                        onclick: move |_| on_assign.call(()),
                        style: "
                            padding: 0.4rem 0.85rem;
                            border-radius: 6px;
                            border: 1px solid var(--q-accent, #00cec9);
                            background: transparent;
                            color: var(--q-accent, #00cec9);
                            font-size: 0.8rem;
                            font-weight: 500;
                            cursor: pointer;
                            align-self: flex-start;
                        ",
                        "Assign to Agent"
                    }
                }
            }
        }
    }
}
