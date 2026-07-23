//! Summary — AI-generated or human-written digests of work, meetings, or projects.
//!
//! Summaries are HyperGraph nodes connected via GeneratedFrom edges to their
//! source (tasks, meetings, channels, projects). They provide quick overviews
//! for dashboards, reports, and notifications.

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

pub type SummaryId = NodeId;

/// What kind of content this summary covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SummaryKind {
    DailyStandup,
    WeeklyReport,
    SprintReview,
    MeetingNotes,
    ProjectStatus,
    TaskDigest,
    ChannelDigest,
    AgentActivity,
    Custom(String),
}

impl std::fmt::Display for SummaryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SummaryKind::DailyStandup => write!(f, "daily_standup"),
            SummaryKind::WeeklyReport => write!(f, "weekly_report"),
            SummaryKind::SprintReview => write!(f, "sprint_review"),
            SummaryKind::MeetingNotes => write!(f, "meeting_notes"),
            SummaryKind::ProjectStatus => write!(f, "project_status"),
            SummaryKind::TaskDigest => write!(f, "task_digest"),
            SummaryKind::ChannelDigest => write!(f, "channel_digest"),
            SummaryKind::AgentActivity => write!(f, "agent_activity"),
            SummaryKind::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

/// A section within a summary (structured content).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarySection {
    pub title: String,
    pub content: String,
    /// Optional list of node IDs referenced in this section
    pub references: Vec<NodeId>,
}

impl SummarySection {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            references: Vec::new(),
        }
    }

    pub fn with_references(mut self, refs: Vec<NodeId>) -> Self {
        self.references = refs;
        self
    }
}

/// An AI-generated or human-written summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub id: SummaryId,
    pub title: String,
    pub kind: SummaryKind,
    /// The NodeId of the source (meeting, project, channel, etc.)
    pub source_id: NodeId,
    pub sections: Vec<SummarySection>,
    pub generated_by: ActorId, // Human writer or AI agent id
    pub is_ai_generated: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Summary {
    pub fn new(
        title: &str,
        kind: SummaryKind,
        source_id: NodeId,
        generated_by: ActorId,
        is_ai_generated: bool,
    ) -> Self {
        Self {
            id: NodeId::new("sum"),
            title: title.to_string(),
            kind,
            source_id,
            sections: Vec::new(),
            generated_by,
            is_ai_generated,
            created_at: now(),
            updated_at: now(),
            metadata: serde_json::Map::new(),
        }
    }

    pub fn add_section(&mut self, section: SummarySection) {
        self.sections.push(section);
        self.updated_at = now();
    }

    /// Get the full text of the summary by concatenating sections.
    pub fn full_text(&self) -> String {
        self.sections
            .iter()
            .map(|s| format!("## {}\n\n{}\n", s.title, s.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn kind() -> &'static str {
        "summary"
    }
}
