//! Signaling handlers — WebSocket upgrade for WebRTC signaling.
//!
//! When a participant joins a room, they connect via WebSocket to
//! `/api/meet/rooms/:room_id/signal`. The DO relays signaling messages
//! (SDP offers/answers, ICE candidates) between peers.

use serde::{Deserialize, Serialize};
use worker::*;

use quang_hub_workplace::graph::NodeId;

/// WebRTC signaling message types exchanged through the DO relay.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SignalingMessage {
    /// SDP offer from client to DO (to be relayed to remote peer).
    SdpOffer {
        from: String,
        to: String,
        sdp: String,
    },
    /// SDP answer from DO to client (relayed from remote peer).
    SdpAnswer {
        from: String,
        to: String,
        sdp: String,
    },
    /// ICE candidate from client to DO.
    IceCandidate {
        from: String,
        to: String,
        candidate: String,
        sdp_mid: Option<String>,
        sdp_mline_index: Option<u16>,
    },
    /// Join notification sent to all existing participants.
    ParticipantJoined {
        participant_id: String,
        display_name: String,
    },
    /// Leave notification.
    ParticipantLeft { participant_id: String },
    /// Request the DO to send the list of current participants.
    RequestParticipants,
    /// Response with current participant list.
    ParticipantList { participants: Vec<ParticipantInfo> },
    /// Error message.
    Error { message: String },
}

/// Info about a single participant in the room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
    pub id: String,
    pub display_name: String,
    pub mic_enabled: bool,
    pub camera_enabled: bool,
    pub screen_sharing: bool,
}

/// Handle WebSocket upgrade for the signaling channel.
///
/// Expects a `:room_id` route parameter and a `participant_id` query param.
/// Upgrades the connection and delegates message handling to the DO.
pub async fn handle_signaling(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = match ctx.param("room_id") {
        Some(id) => id,
        None => return Response::error("Missing room_id", 400),
    };

    let participant_id = match req
        .url()?
        .query_pairs()
        .find(|(k, _)| k == "participant_id")
    {
        Some((_, v)) => v.to_string(),
        None => return Response::error("Missing participant_id query param", 400),
    };

    // Get the DO stub
    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(room_id)?
        .get_stub()?;

    // Forward WebSocket upgrade to the DO
    let do_response = do_stub.fetch_with_request(req).await?;

    if do_response.status() == 101 {
        // WebSocket upgrade successful
        Ok(do_response)
    } else {
        Response::error("Failed to establish signaling channel", 502)
    }
}
