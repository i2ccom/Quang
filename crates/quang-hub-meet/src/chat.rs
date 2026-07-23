//! Chat — in-call chat messages, handraise, and polls.

use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{now, ActorId, NodeId, Timestamp};

/// An in-call chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InCallMessage {
    pub id: String,
    pub room_id: NodeId,
    pub author: ActorId,
    pub text: String,
    pub is_private: bool, // true for direct message to specific participant
    pub target_participant: Option<NodeId>,
    pub created_at: Timestamp,
}

impl InCallMessage {
    pub fn new(room_id: NodeId, author: ActorId, text: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            room_id,
            author,
            text: text.to_string(),
            is_private: false,
            target_participant: None,
            created_at: now(),
        }
    }

    pub fn private_to(mut self, participant_id: NodeId) -> Self {
        self.is_private = true;
        self.target_participant = Some(participant_id);
        self
    }
}

/// A poll created during a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: String,
    pub room_id: NodeId,
    pub question: String,
    pub options: Vec<String>,
    pub votes: Vec<PollVote>,
    pub is_active: bool,
    pub created_by: ActorId,
    pub created_at: Timestamp,
}

impl Poll {
    pub fn new(room_id: NodeId, question: &str, options: Vec<String>, created_by: ActorId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            room_id,
            question: question.to_string(),
            options,
            votes: Vec::new(),
            is_active: true,
            created_by,
            created_at: now(),
        }
    }

    pub fn vote(&mut self, voter: ActorId, option_index: usize) -> bool {
        if option_index >= self.options.len() || !self.is_active {
            return false;
        }
        // Remove previous vote if exists
        self.votes.retain(|v| v.voter != voter);
        self.votes.push(PollVote {
            voter,
            option_index,
            voted_at: now(),
        });
        true
    }

    pub fn results(&self) -> Vec<(String, usize)> {
        let mut counts = vec![0usize; self.options.len()];
        for vote in &self.votes {
            if vote.option_index < counts.len() {
                counts[vote.option_index] += 1;
            }
        }
        self.options
            .iter()
            .enumerate()
            .map(|(i, opt)| (opt.clone(), counts[i]))
            .collect()
    }

    pub fn close(&mut self) {
        self.is_active = false;
    }
}

/// A single vote in a poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollVote {
    pub voter: ActorId,
    pub option_index: usize,
    pub voted_at: Timestamp,
}
