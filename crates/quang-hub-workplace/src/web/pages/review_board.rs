//! Review Board page — list pending and completed reviews.
//!
//! Shows reviews as cards grouped by status: Pending, In Progress,
//! Approved, Changes Requested, Rejected.

use dioxus::prelude::*;

/// Stub review data.
#[derive(Clone, Debug)]
struct ReviewStub {
    id: String,
    title: String,
    target_kind: String,
    status: String,
    reviewer_count: usize,
    approvals_received: u32,
    required_approvals: u32,
    comment_count: usize,
    created_by: String,
}

/// Sample reviews.
fn sample_reviews() -> Vec<ReviewStub> {
    vec![
        ReviewStub {
            id: "r1".into(), title: "Review: Task status transitions".into(),
            target_kind: "code_change".into(), status: "pending".into(),
            reviewer_count: 2, approvals_received: 0, required_approvals: 2,
            comment_count: 0, created_by: "bob".into(),
        },
        ReviewStub {
            id: "r2".into(), title: "Review: OAuth flow implementation".into(),
            target_kind: "code_change".into(), status: "in_progress".into(),
            reviewer_count: 2, approvals_received: 1, required_approvals: 2,
            comment_count: 3, created_by: "alice".into(),
        },
        ReviewStub {
            id: "r3".into(), title: "Review: Landing page design".into(),
            target_kind: "document".into(), status: "approved".into(),
            reviewer_count: 1, approvals_received: 1, required_approvals: 1,
            comment_count: 2, created_by: "carol".into(),
        },
        ReviewStub {
            id: "r4".into(), title: "Review: Agent API integration".into(),
            target_kind: "agent_output".into(), status: "changes_requested".into(),
            reviewer_count: 2, approvals_received: 0, required_approvals: 2,
            comment_count: 5, created_by: "dave".into(),
        },
        ReviewStub {
            id: "r5".into(), title: "Review: Database schema v2".into(),
            target_kind: "document".into(), status: "rejected".into(),
            reviewer_count: 2, approvals_received: 0, required_approvals: 2,
            comment_count: 4, created_by: "alice".into(),
        },
        ReviewStub {
            id: "r6".into(), title: "Review: GraphQL schema for projects".into(),
            target_kind: "code_change".into(), status: "in_progress".into(),
            reviewer_count: 1, approvals_received: 1, required_approvals: 1,
            comment_count: 1, created_by: "bob".into(),
        },
    ]
}

/// Review Board page component.
#[component]
pub fn ReviewBoard() -> Element {
    let reviews = use_signal(|| sample_reviews());
    let mut filter = use_signal(|| String::new());

    let grouped = |status: &str| -> Vec<ReviewStub> {
        reviews()
            .iter()
            .filter(|r| {
                let matches_status = r.status == status;
                if filter().is_empty() {
                    matches_status
                } else {
                    matches_status && r.title.to_lowercase().contains(&filter().to_lowercase())
                }
            })
            .cloned()
            .collect()
    };

    rsx! {
        div {
            class: "review-board",
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
                    "Review Board"
                }
                div {
                    style: "display: flex; align-items: center; gap: 1rem;",
                    input {
                        placeholder: "Filter reviews...",
                        value: filter(),
                        oninput: move |e| filter.set(e.value()),
                        style: "
                            padding: 0.45rem 0.75rem;
                            border-radius: 6px;
                            border: 1px solid var(--q-surface-border, #333);
                            background: var(--q-bg, #0f0f1a);
                            color: var(--q-text, #e0e0e0);
                            font-size: 0.85rem;
                            outline: none;
                            width: 200px;
                        "
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

                // Sections
                ReviewSection {
                    title: "Pending",
                    icon: "⏳",
                    color: "#636e72",
                    reviews: grouped("pending"),
                },
                ReviewSection {
                    title: "In Progress",
                    icon: "🔄",
                    color: "#0984e3",
                    reviews: grouped("in_progress"),
                },
                ReviewSection {
                    title: "Approved",
                    icon: "✅",
                    color: "#00b894",
                    reviews: grouped("approved"),
                },
                ReviewSection {
                    title: "Changes Requested",
                    icon: "✏️",
                    color: "#fdcb6e",
                    reviews: grouped("changes_requested"),
                },
                ReviewSection {
                    title: "Rejected",
                    icon: "❌",
                    color: "#ff4757",
                    reviews: grouped("rejected"),
                },
            }
        }
    }
}

/// A section grouping reviews by status.
#[component]
fn ReviewSection(title: String, icon: String, color: String, reviews: Vec<ReviewStub>) -> Element {
    rsx! {
        div {
            style: "margin-bottom: 1.5rem;",
            h2 {
                style: "
                    font-size: 0.9rem;
                    font-weight: 600;
                    color: var(--q-text-secondary, #888);
                    margin: 0 0 0.75rem 0;
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                ",
                span { "{icon}" }
                span { "{title}" }
                span {
                    style: "
                        background: {color}33;
                        color: {color};
                        padding: 0.1rem 0.45rem;
                        border-radius: 4px;
                        font-size: 0.75rem;
                    ",
                    "{reviews.len()}"
                }
            }

            if reviews.is_empty() {
                div {
                    style: "
                        padding: 1rem;
                        color: var(--q-text-muted, #555);
                        font-size: 0.85rem;
                        text-align: center;
                        background: var(--q-surface, #1a1a2e);
                        border: 1px solid var(--q-surface-border, #333);
                        border-radius: 8px;
                    ",
                    "No reviews in this status."
                }
            } else {
                div {
                    style: "display: flex; flex-direction: column; gap: 0.5rem;",
                    for review in &reviews {
                        ReviewCard { review: review.clone(), color: color.clone() }
                    }
                }
            }
        }
    }
}

/// A single review card.
#[component]
fn ReviewCard(review: ReviewStub, color: String) -> Element {
    let status_color = match review.status.as_str() {
        "approved" => "#00b894",
        "changes_requested" => "#fdcb6e",
        "rejected" => "#ff4757",
        "in_progress" => "#0984e3",
        _ => "#636e72",
    };

    rsx! {
        div {
            class: "review-card",
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-left: 3px solid {status_color};
                border-radius: 8px;
                padding: 1rem;
                display: flex;
                align-items: center;
                justify-content: space-between;
                transition: background 0.2s;
                cursor: pointer;
            ",

            // Left side — info
            div {
                style: "flex: 1;",
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        gap: 0.5rem;
                        margin-bottom: 0.25rem;
                    ",
                    span {
                        style: "
                            font-size: 0.75rem;
                            padding: 0.1rem 0.4rem;
                            border-radius: 3px;
                            background: var(--q-bg, #0f0f1a);
                            color: var(--q-text-secondary, #888);
                            text-transform: uppercase;
                            font-weight: 500;
                        ",
                        "{review.target_kind}"
                    }
                    span {
                        style: "
                            font-size: 0.75rem;
                            color: var(--q-text-muted, #555);
                        ",
                        "by {review.created_by}"
                    }
                }
                h3 {
                    style: "
                        font-size: 0.9rem;
                        font-weight: 500;
                        margin: 0;
                    ",
                    "{review.title}"
                }
            }

            // Right side — metadata
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                    font-size: 0.8rem;
                    color: var(--q-text-secondary, #888);
                ",
                span {
                    style: "
                        display: flex;
                        align-items: center;
                        gap: 0.25rem;
                    ",
                    "👥 {review.reviewer_count}"
                }
                span {
                    style: "
                        display: flex;
                        align-items: center;
                        gap: 0.25rem;
                    ",
                    "💬 {review.comment_count}"
                }
                span {
                    style: "
                        display: flex;
                        align-items: center;
                        gap: 0.25rem;
                        color: {status_color};
                    ",
                    "{review.approvals_received}/{review.required_approvals} ✓"
                }
            }
        }
    }
}
