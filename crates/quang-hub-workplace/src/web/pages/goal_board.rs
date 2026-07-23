//! Goal Board page — track OKR-aligned goals with progress bars.
//!
//! Displays goals in two sections: Active Goals and Completed Goals.
//! Each goal shows its key results with individual progress bars.

use dioxus::prelude::*;

use crate::web::components::goal_progress::GoalProgress;

/// Stub key result data.
#[derive(Clone, Debug)]
struct KeyResultStub {
    title: String,
    current: f64,
    target: f64,
    unit: String,
}

/// Stub goal data.
#[derive(Clone, Debug)]
struct GoalStub {
    id: String,
    title: String,
    period: String,
    progress: f64,
    status: String,
    key_results: Vec<KeyResultStub>,
}

/// Sample goals for the board.
fn sample_goals() -> Vec<GoalStub> {
    vec![
        GoalStub {
            id: "g1".into(),
            title: "Launch MVP of agent collaboration platform".into(),
            period: "2026-Q2".into(),
            progress: 0.65,
            status: "on_track".into(),
            key_results: vec![
                KeyResultStub { title: "Ship task management".into(), current: 80.0, target: 100.0, unit: "%".into() },
                KeyResultStub { title: "Onboard 10 beta teams".into(), current: 4.0, target: 10.0, unit: "teams".into() },
                KeyResultStub { title: "Agent chat integration".into(), current: 3.0, target: 5.0, unit: "channels".into() },
            ],
        },
        GoalStub {
            id: "g2".into(),
            title: "Achieve 99.9% API uptime".into(),
            period: "2026-Q2".into(),
            progress: 0.92,
            status: "on_track".into(),
            key_results: vec![
                KeyResultStub { title: "Reduce p95 latency <200ms".into(), current: 145.0, target: 200.0, unit: "ms".into() },
                KeyResultStub { title: "Zero critical incidents".into(), current: 1.0, target: 0.0, unit: "incidents".into() },
            ],
        },
        GoalStub {
            id: "g3".into(),
            title: "Open source HyperGraph engine".into(),
            period: "2026-Q2".into(),
            progress: 0.3,
            status: "at_risk".into(),
            key_results: vec![
                KeyResultStub { title: "Publish crate to crates.io".into(), current: 0.0, target: 1.0, unit: "publish".into() },
                KeyResultStub { title: "Write API documentation".into(), current: 20.0, target: 100.0, unit: "%".into() },
            ],
        },
        GoalStub {
            id: "g4".into(),
            title: "Q1 Infrastructure migration".into(),
            period: "2026-Q1".into(),
            progress: 1.0,
            status: "completed".into(),
            key_results: vec![
                KeyResultStub { title: "Migrate to Cloudflare Workers".into(), current: 1.0, target: 1.0, unit: "done".into() },
                KeyResultStub { title: "Set up D1 databases".into(), current: 3.0, target: 3.0, unit: "dbs".into() },
            ],
        },
    ]
}

/// Goal Board page component.
#[component]
pub fn GoalBoard() -> Element {
    let goals = use_signal(|| sample_goals());

    let active_goals: Vec<_> = goals().iter().filter(|g| g.status != "completed").cloned().collect();
    let completed_goals: Vec<_> = goals().iter().filter(|g| g.status == "completed").cloned().collect();

    rsx! {
        div {
            class: "goal-board",
            style: "
                min-height: 100vh;
                background: var(--q-bg, #0f0f1a);
                color: var(--q-text, #e0e0e0);
                font-family: 'Inter', system-ui, sans-serif;
            ",

            // Top bar
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0.75rem 2rem;
                    background: var(--q-surface, #1a1a2e);
                    border-bottom: 1px solid var(--q-surface-border, #333);
                ",
                h1 {
                    style: "font-size: 1.25rem; font-weight: 600; margin: 0;",
                    "Goals Board"
                }
                div {
                    style: "display: flex; align-items: center; gap: 0.75rem;",
                    span {
                        style: "font-size: 0.85rem; color: var(--q-text-secondary, #888);",
                        "2026-Q2"
                    }
                    button {
                        style: "
                            padding: 0.45rem 1rem;
                            border-radius: 6px;
                            border: none;
                            background: var(--q-primary, #6c5ce7);
                            color: #fff;
                            font-size: 0.85rem;
                            font-weight: 500;
                            cursor: pointer;
                        ",
                        "+ New Goal"
                    }
                }
            }

            // Content
            div {
                style: "
                    max-width: 800px;
                    margin: 0 auto;
                    padding: 1.5rem 2rem;
                ",

                // Active goals
                h2 {
                    style: "
                        font-size: 1rem;
                        font-weight: 600;
                        color: var(--q-text-secondary, #888);
                        margin: 0 0 1rem 0;
                        text-transform: uppercase;
                        letter-spacing: 0.05em;
                    ",
                    "Active Goals ({active_goals.len()})"
                }

                if active_goals.is_empty() {
                    div {
                        style: "
                            text-align: center;
                            padding: 3rem;
                            color: var(--q-text-muted, #555);
                        ",
                        "No active goals. Create one to get started."
                    }
                } else {
                    div {
                        style: "display: flex; flex-direction: column; gap: 1rem; margin-bottom: 2.5rem;",
                        for goal in &active_goals {
                            GoalCard { goal: goal.clone() }
                        }
                    }
                }

                // Completed goals
                if !completed_goals.is_empty() {
                    h2 {
                        style: "
                            font-size: 1rem;
                            font-weight: 600;
                            color: var(--q-text-secondary, #888);
                            margin: 0 0 1rem 0;
                            text-transform: uppercase;
                            letter-spacing: 0.05em;
                        ",
                        "Completed ({completed_goals.len()})"
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.75rem; opacity: 0.7;",
                        for goal in &completed_goals {
                            GoalCard { goal: goal.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// A single goal card showing title, period, progress, and key results.
#[component]
fn GoalCard(goal: GoalStub) -> Element {
    let status_color = match goal.status.as_str() {
        "on_track" => "#00b894",
        "at_risk" => "#fdcb6e",
        "behind" => "#e17055",
        "completed" => "#6c5ce7",
        _ => "#636e72",
    };

    let status_label = match goal.status.as_str() {
        "on_track" => "On Track",
        "at_risk" => "At Risk",
        "behind" => "Behind",
        "completed" => "Completed",
        _ => "Active",
    };

    rsx! {
        div {
            class: "goal-card",
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 12px;
                padding: 1.25rem;
                transition: box-shadow 0.2s;
            ",

            // Goal header
            div {
                style: "
                    display: flex;
                    align-items: flex-start;
                    justify-content: space-between;
                    margin-bottom: 0.75rem;
                ",
                div {
                    h3 {
                        style: "
                            font-size: 1rem;
                            font-weight: 600;
                            margin: 0 0 0.25rem 0;
                        ",
                        "{goal.title}"
                    }
                    span {
                        style: "
                            font-size: 0.8rem;
                            color: var(--q-text-muted, #555);
                        ",
                        "{goal.period}"
                    }
                }
                div {
                    style: "
                        padding: 0.2rem 0.6rem;
                        border-radius: 4px;
                        font-size: 0.75rem;
                        font-weight: 500;
                        background: {status_color}22;
                        color: {status_color};
                        border: 1px solid {status_color}44;
                    ",
                    "{status_label}"
                }
            }

            // Progress bar
            GoalProgress {
                progress: goal.progress,
            }

            // Key Results
            div {
                style: "
                    margin-top: 1rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.6rem;
                ",
                for kr in &goal.key_results {
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.75rem;
                        ",
                        div {
                            style: "flex: 1;",
                            div {
                                style: "
                                    display: flex;
                                    justify-content: space-between;
                                    font-size: 0.8rem;
                                    margin-bottom: 0.2rem;
                                ",
                                span { "{kr.title}" }
                                span {
                                    style: "color: var(--q-text-secondary, #888);",
                                    "{kr.current:.1} / {kr.target:.0} {kr.unit}"
                                }
                            }
                            // Mini progress bar
                            div {
                                style: "
                                    height: 4px;
                                    background: var(--q-bg, #0f0f1a);
                                    border-radius: 2px;
                                    overflow: hidden;
                                ",
                                div {
                                    style: "
                                        height: 100%;
                                        width: {((kr.current / kr.target.max(1.0)) * 100.0).min(100.0).to_string()}%;
                                        background: {status_color};
                                        border-radius: 2px;
                                        transition: width 0.3s ease;
                                    "
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
