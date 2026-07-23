//! MeetHub — the orchestrator for meeting rooms, participants, and media.
//!
//! Manages the lifecycle of meetings and coordinates participants,
//! media streams, recordings, and in-call chat.

use std::collections::HashMap;

use quang_hub_workplace::graph::{ActorId, HyperGraph, NodeId, NodeKind};

use crate::chat::{InCallMessage, Poll};
use crate::event::{MeetingEvent, MeetingEventEnvelope};
use crate::media::MediaStream;
use crate::participant::{Participant, ParticipantRole};
use crate::recording::{Recording, Transcript};
use crate::room::MeetingRoom;

/// The meeting orchestrator.
pub struct MeetHub {
    pub rooms: HashMap<NodeId, MeetingRoom>,
    pub participants: HashMap<NodeId, Vec<Participant>>,
    pub media_streams: HashMap<NodeId, Vec<MediaStream>>,
    pub recordings: HashMap<NodeId, Vec<Recording>>,
    pub transcripts: HashMap<NodeId, Vec<Transcript>>,
    pub chat_messages: HashMap<NodeId, Vec<InCallMessage>>,
    pub polls: HashMap<NodeId, Vec<Poll>>,
    pub event_log: Vec<MeetingEventEnvelope>,
    sequence: u64,
    /// Reference to the workplace graph for linking meetings to channels/projects
    pub graph: Option<HyperGraph>,
}

impl MeetHub {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            participants: HashMap::new(),
            media_streams: HashMap::new(),
            recordings: HashMap::new(),
            transcripts: HashMap::new(),
            chat_messages: HashMap::new(),
            polls: HashMap::new(),
            event_log: Vec::new(),
            sequence: 0,
            graph: None,
        }
    }

    pub fn with_graph(graph: HyperGraph) -> Self {
        let mut hub = Self::new();
        hub.graph = Some(graph);
        hub
    }

    fn emit(&mut self, event: MeetingEvent) {
        self.sequence += 1;
        let envelope = MeetingEventEnvelope {
            id: uuid::Uuid::new_v4().to_string(),
            event,
            timestamp: quang_hub_workplace::graph::now(),
            sequence: self.sequence,
        };
        tracing::debug!("[MeetHub] {}", envelope.event.description());
        self.event_log.push(envelope);
    }

    // ── Room lifecycle ──

    pub fn create_room(&mut self, title: &str, topic: &str, host: ActorId) -> NodeId {
        let room = MeetingRoom::new(title, topic, host.clone());
        let id = room.id.clone();
        self.rooms.insert(id.clone(), room);

        // Link to graph if available
        if let Some(graph) = &mut self.graph {
            graph.add_node(id.clone(), NodeKind::MeetingRoom, &self.rooms[&id]);
        }

        self.emit(MeetingEvent::RoomCreated {
            room_id: id.clone(),
            actor: host,
        });
        id
    }

    pub fn start_room(&mut self, room_id: &NodeId, actor: ActorId) -> bool {
        if let Some(room) = self.rooms.get_mut(room_id) {
            room.start();
            self.emit(MeetingEvent::RoomStarted {
                room_id: room_id.clone(),
                actor,
            });
            true
        } else {
            false
        }
    }

    pub fn end_room(&mut self, room_id: &NodeId, actor: ActorId) -> bool {
        if let Some(room) = self.rooms.get_mut(room_id) {
            room.end();
            self.emit(MeetingEvent::RoomEnded {
                room_id: room_id.clone(),
                actor,
            });
            true
        } else {
            false
        }
    }

    // ── Participant management ──

    pub fn join_room(
        &mut self,
        room_id: &NodeId,
        actor: ActorId,
        display_name: &str,
    ) -> Option<NodeId> {
        if !self.rooms.contains_key(room_id) {
            return None;
        }

        let participant = Participant::new(
            room_id.clone(),
            actor.clone(),
            ParticipantRole::Participant,
            display_name,
        );
        let id = participant.id.clone();
        self.participants
            .entry(room_id.clone())
            .or_default()
            .push(participant);

        self.emit(MeetingEvent::ParticipantJoined {
            room_id: room_id.clone(),
            participant_id: id.clone(),
            actor,
        });
        Some(id)
    }

    pub fn leave_room(&mut self, room_id: &NodeId, participant_id: &NodeId) {
        if let Some(participants) = self.participants.get_mut(room_id) {
            if let Some(p) = participants.iter_mut().find(|p| p.id == *participant_id) {
                let actor = p.actor.clone();
                p.disconnect();
                self.emit(MeetingEvent::ParticipantLeft {
                    room_id: room_id.clone(),
                    participant_id: participant_id.clone(),
                    actor,
                });
            }
        }
    }

    pub fn get_participants(&self, room_id: &NodeId) -> Vec<&Participant> {
        self.participants
            .get(room_id)
            .map(|ps| ps.iter().filter(|p| p.is_connected).collect())
            .unwrap_or_default()
    }

    // ── Media ──

    pub fn add_media_stream(
        &mut self,
        room_id: &NodeId,
        participant_id: &NodeId,
    ) -> Option<NodeId> {
        if !self.rooms.contains_key(room_id) {
            return None;
        }
        let stream = MediaStream::new(participant_id.clone(), room_id.clone());
        let id = stream.id.clone();
        self.media_streams
            .entry(room_id.clone())
            .or_default()
            .push(stream);
        Some(id)
    }

    pub fn toggle_mic(&mut self, room_id: &NodeId, participant_id: &NodeId) {
        let enabled = if let Some(participants) = self.participants.get_mut(room_id) {
            if let Some(p) = participants.iter_mut().find(|p| p.id == *participant_id) {
                p.toggle_mic();
                Some(p.media.mic_enabled)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(enabled) = enabled {
            self.emit(MeetingEvent::MicToggled {
                room_id: room_id.clone(),
                participant_id: participant_id.clone(),
                enabled,
            });
        }
    }

    pub fn toggle_camera(&mut self, room_id: &NodeId, participant_id: &NodeId) {
        let enabled = if let Some(participants) = self.participants.get_mut(room_id) {
            if let Some(p) = participants.iter_mut().find(|p| p.id == *participant_id) {
                p.toggle_camera();
                Some(p.media.camera_enabled)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(enabled) = enabled {
            self.emit(MeetingEvent::CameraToggled {
                room_id: room_id.clone(),
                participant_id: participant_id.clone(),
                enabled,
            });
        }
    }

    // ── Recording ──

    pub fn start_recording(&mut self, room_id: &NodeId, actor: ActorId) -> Option<NodeId> {
        if !self.rooms.contains_key(room_id) {
            return None;
        }
        let recording = Recording::new(room_id.clone(), actor.clone());
        let id = recording.id.clone();
        self.recordings
            .entry(room_id.clone())
            .or_default()
            .push(recording);
        self.emit(MeetingEvent::RecordingStarted {
            room_id: room_id.clone(),
            actor,
        });
        Some(id)
    }

    pub fn stop_recording(&mut self, room_id: &NodeId, recording_id: &NodeId, actor: ActorId) {
        if let Some(recordings) = self.recordings.get_mut(room_id) {
            if let Some(r) = recordings.iter_mut().find(|r| r.id == *recording_id) {
                r.stop();
                self.emit(MeetingEvent::RecordingStopped {
                    room_id: room_id.clone(),
                    actor,
                });
            }
        }
    }

    // ── In-call chat ──

    pub fn send_chat_message(
        &mut self,
        room_id: &NodeId,
        author: ActorId,
        text: &str,
    ) -> Option<String> {
        if !self.rooms.contains_key(room_id) {
            return None;
        }
        let msg = InCallMessage::new(room_id.clone(), author.clone(), text);
        let id = msg.id.clone();
        self.chat_messages
            .entry(room_id.clone())
            .or_default()
            .push(msg);
        self.emit(MeetingEvent::ChatMessageSent {
            room_id: room_id.clone(),
            message_id: id.clone(),
            actor: author,
        });
        Some(id)
    }

    // ── Polls ──

    pub fn create_poll(
        &mut self,
        room_id: &NodeId,
        question: &str,
        options: Vec<String>,
        created_by: ActorId,
    ) -> Option<String> {
        if !self.rooms.contains_key(room_id) {
            return None;
        }
        let poll = Poll::new(room_id.clone(), question, options, created_by.clone());
        let id = poll.id.clone();
        self.polls.entry(room_id.clone()).or_default().push(poll);
        self.emit(MeetingEvent::PollCreated {
            room_id: room_id.clone(),
            poll_id: id.clone(),
            actor: created_by,
        });
        Some(id)
    }

    pub fn vote_poll(
        &mut self,
        room_id: &NodeId,
        poll_id: &str,
        voter: ActorId,
        option: usize,
    ) -> bool {
        if let Some(polls) = self.polls.get_mut(room_id) {
            if let Some(poll) = polls.iter_mut().find(|p| p.id == poll_id) {
                let result = poll.vote(voter.clone(), option);
                if result {
                    self.emit(MeetingEvent::PollVoted {
                        room_id: room_id.clone(),
                        poll_id: poll_id.to_string(),
                        actor: voter,
                    });
                }
                return result;
            }
        }
        false
    }

    // ── Events ──

    pub fn recent_events(&self, since_sequence: u64) -> Vec<&MeetingEventEnvelope> {
        self.event_log
            .iter()
            .filter(|e| e.sequence > since_sequence)
            .collect()
    }
}

impl Default for MeetHub {
    fn default() -> Self {
        Self::new()
    }
}
