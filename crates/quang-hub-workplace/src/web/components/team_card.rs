//! TeamCard — displays a team with member avatars and role distribution.

use dioxus::prelude::*;

/// Stub member info for display purposes.
#[derive(Clone, PartialEq)]
pub struct MemberInfo {
    pub name: String,
    pub avatar: String,
    pub is_agent: bool,
}

/// Props for the TeamCard component.
#[derive(Clone, PartialEq, Props)]
pub struct TeamCardProps {
    pub id: String,
    pub name: String,
    pub description: String,
    pub member_count: usize,
    pub human_count: usize,
    pub agent_count: usize,
    pub members: Vec<MemberInfo>,
    pub on_click: EventHandler<String>,
}

/// A card representing a team, showing member composition and roles.
#[component]
pub fn TeamCard(props: TeamCardProps) -> Element {
    let human_pct = if props.member_count > 0 {
        (props.human_count as f64 / props.member_count as f64) * 100.0
    } else {
        0.0
    };

    // Show up to 5 avatars, with a +N overflow
    let visible_members: Vec<_> = props.members.iter().take(5).collect();
    let overflow = props.member_count.saturating_sub(5);

    rsx! {
        div {
            class: "team-card",
            onclick: move |_| props.on_click.call(props.id.clone()),
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 12px;
                padding: 1.25rem;
                cursor: pointer;
                transition: all 0.2s ease;
            ",

            // Name + count
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    margin-bottom: 0.35rem;
                ",
                h3 {
                    style: "
                        font-size: 1rem;
                        font-weight: 600;
                        margin: 0;
                        color: var(--q-text, #e0e0e0);
                    ",
                    "{props.name}"
                }
                span {
                    style: "
                        font-size: 0.8rem;
                        color: var(--q-text-muted, #555);
                    ",
                    "{props.member_count} members"
                }
            }

            // Description
            p {
                style: "
                    font-size: 0.82rem;
                    color: var(--q-text-secondary, #888);
                    margin: 0 0 0.75rem 0;
                    line-height: 1.5;
                ",
                "{props.description}"
            }

            // Human / Agent composition bar
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 0.5rem;
                    margin-bottom: 0.75rem;
                ",
                // Composition bar
                div {
                    style: "
                        flex: 1;
                        height: 6px;
                        background: var(--q-bg, #0f0f1a);
                        border-radius: 3px;
                        overflow: hidden;
                        display: flex;
                    ",
                    div {
                        style: "
                            height: 100%;
                            width: {human_pct.to_string()}%;
                            background: var(--q-primary, #6c5ce7);
                            border-radius: 3px 0 0 3px;
                            transition: width 0.3s;
                        "
                    }
                    div {
                        style: "
                            height: 100%;
                            width: {(100.0 - human_pct).to_string()}%;
                            background: var(--q-accent, #00cec9);
                            border-radius: 0 3px 3px 0;
                            transition: width 0.3s;
                        "
                    }
                }
                span {
                    style: "
                        font-size: 0.75rem;
                        color: var(--q-primary, #6c5ce7);
                        font-weight: 500;
                    ",
                    "{props.human_count}H"
                }
                span {
                    style: "
                        font-size: 0.75rem;
                        color: var(--q-accent, #00cec9);
                        font-weight: 500;
                    ",
                    "{props.agent_count}A"
                }
            }

            // Member avatars row
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 0.35rem;
                ",
                for member in &visible_members {
                    div {
                        style: if member.is_agent {
                            "
                                width: 28px;
                                height: 28px;
                                border-radius: 6px;
                                background: var(--q-accent, #00cec9);
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 0.65rem;
                                font-weight: 600;
                                color: #fff;
                                border: 2px solid var(--q-bg, #0f0f1a);
                            "
                        } else {
                            "
                                width: 28px;
                                height: 28px;
                                border-radius: 6px;
                                background: var(--q-primary, #6c5ce7);
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 0.65rem;
                                font-weight: 600;
                                color: #fff;
                                border: 2px solid var(--q-bg, #0f0f1a);
                            "
                        },
                        "{member.name.chars().next().unwrap_or('?')}"
                    }
                }
                if overflow > 0 {
                    div {
                        style: "
                            width: 28px;
                            height: 28px;
                            border-radius: 6px;
                            background: var(--q-surface-border, #333);
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            font-size: 0.65rem;
                            font-weight: 500;
                            color: var(--q-text-secondary, #888);
                        ",
                        "+{overflow}"
                    }
                }
            }
        }
    }
}
