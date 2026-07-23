//! Recording — meeting recordings, transcripts, and AI-generated notes.

use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{now, ActorId, NodeId, Timestamp};

/// Status of a recording.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    Recording,
    Processing,
    Ready,
    Failed,
    Deleted,
}

/// A recording of a meeting session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recording {
    pub id: NodeId,
    pub room_id: NodeId,
    pub started_by: ActorId,
    pub status: RecordingStatus,
    pub started_at: Timestamp,
    pub ended_at: Option<Timestamp>,
    pub duration_seconds: Option<u64>,
    pub file_url: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Recording {
    pub fn new(room_id: NodeId, started_by: ActorId) -> Self {
        Self {
            id: NodeId::new("rec"),
            room_id,
            started_by,
            status: RecordingStatus::Recording,
            started_at: now(),
            ended_at: None,
            duration_seconds: None,
            file_url: None,
            file_size_bytes: None,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn stop(&mut self) {
        self.status = RecordingStatus::Processing;
        let end = now();
        self.ended_at = Some(end);
        let dur = end - self.started_at;
        self.duration_seconds = Some(dur.num_seconds().max(0) as u64);
    }

    pub fn mark_ready(&mut self, file_url: &str, size_bytes: u64) {
        self.status = RecordingStatus::Ready;
        self.file_url = Some(file_url.to_string());
        self.file_size_bytes = Some(size_bytes);
    }

    pub fn kind() -> &'static str {
        "recording"
    }
}

/// A segment of a transcript with speaker attribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub start_time_ms: u64,
    pub end_time_ms: u64,
    pub is_agent: bool,
}

/// A full transcript for a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcript {
    pub id: NodeId,
    pub room_id: NodeId,
    pub segments: Vec<TranscriptSegment>,
    pub language: String,
    pub created_at: Timestamp,
}

impl Transcript {
    pub fn new(room_id: NodeId) -> Self {
        Self {
            id: NodeId::new("trans"),
            room_id,
            segments: Vec::new(),
            language: "en".to_string(),
            created_at: now(),
        }
    }

    pub fn add_segment(&mut self, segment: TranscriptSegment) {
        self.segments.push(segment);
    }

    pub fn full_text(&self) -> String {
        self.segments
            .iter()
            .map(|s| format!("[{}] {}: {}", s.start_time_ms, s.speaker, s.text))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
