//! Media — media streams and tracks for meeting participants.

use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{NodeId, Timestamp, now};

/// The type of a media track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackKind {
    Audio,
    Video,
    ScreenShare,
    Presentation,
}

impl std::fmt::Display for TrackKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrackKind::Audio => write!(f, "audio"),
            TrackKind::Video => write!(f, "video"),
            TrackKind::ScreenShare => write!(f, "screen_share"),
            TrackKind::Presentation => write!(f, "presentation"),
        }
    }
}

/// The quality level of a track.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// A single media track (audio, video, or screen).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTrack {
    pub id: String,
    pub participant_id: NodeId,
    pub kind: TrackKind,
    pub quality: TrackQuality,
    pub is_active: bool,
    pub is_muted: bool,
    pub sdp: Option<String>, // SDP offer/answer
    pub created_at: Timestamp,
}

/// A collection of media tracks for a participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaStream {
    pub id: NodeId,
    pub participant_id: NodeId,
    pub room_id: NodeId,
    pub tracks: Vec<MediaTrack>,
    pub created_at: Timestamp,
}

impl MediaStream {
    pub fn new(participant_id: NodeId, room_id: NodeId) -> Self {
        Self {
            id: NodeId::new("ms"),
            participant_id,
            room_id,
            tracks: Vec::new(),
            created_at: now(),
        }
    }

    pub fn add_track(&mut self, kind: TrackKind, quality: TrackQuality) -> String {
        let track_id = uuid::Uuid::new_v4().to_string();
        self.tracks.push(MediaTrack {
            id: track_id.clone(),
            participant_id: self.participant_id.clone(),
            kind,
            quality,
            is_active: true,
            is_muted: false,
            sdp: None,
            created_at: now(),
        });
        track_id
    }

    pub fn remove_track(&mut self, track_id: &str) {
        self.tracks.retain(|t| t.id != track_id);
    }

    pub fn mute_track(&mut self, track_id: &str) {
        if let Some(track) = self.tracks.iter_mut().find(|t| t.id == track_id) {
            track.is_muted = true;
        }
    }

    pub fn unmute_track(&mut self, track_id: &str) {
        if let Some(track) = self.tracks.iter_mut().find(|t| t.id == track_id) {
            track.is_muted = false;
        }
    }

    pub fn kind() -> &'static str {
        "media_stream"
    }
}
