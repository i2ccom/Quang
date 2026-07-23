//! Room handlers — create, join, leave, end, list meetings.
//!
//! REST endpoints mapped under `/api/meet/rooms/...`. These handlers
//! delegate state management to the `MeetHub` data model.

use serde::{Deserialize, Serialize};
use worker::*;

use quang_hub_workplace::graph::{ActorId, NodeId};

use crate::durable_object::MeetRoomDO;

// ── Request / Response types ──

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub title: String,
    pub topic: Option<String>,
    pub host_actor_id: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub actor_id: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    pub room_id: String,
    pub title: String,
    pub status: String,
    pub invite_link: String,
}

#[derive(Debug, Serialize)]
pub struct JoinResponse {
    pub participant_id: String,
    pub signaling_url: String,
    pub turn_credentials: TurnCredentials,
}

#[derive(Debug, Serialize)]
pub struct TurnCredentials {
    pub urls: Vec<String>,
    pub username: String,
    pub credential: String,
}

// ── Handlers ──

/// POST /api/meet/rooms — create a new meeting room.
pub async fn create_room(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: CreateRoomRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => return Response::error("Bad request", 400),
    };

    // Create a Durable Object stub for this room
    let room_id = NodeId::new("room").to_string();
    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(&room_id)?
        .get_stub()?;

    // Forward the create request to the DO
    let create_req = CreateRoomRequest {
        title: body.title,
        topic: body.topic,
        host_actor_id: body.host_actor_id,
    };
    let _ = do_stub.fetch_with_json(&create_req).await?;

    let response = RoomResponse {
        room_id: room_id.clone(),
        title: body.title,
        status: "waiting".to_string(),
        invite_link: format!("/meet/{}", room_id),
    };

    Response::from_json(&response)
}

/// POST /api/meet/rooms/:room_id/join — join an existing room.
pub async fn join_room(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = match ctx.param("room_id") {
        Some(id) => id,
        None => return Response::error("Missing room_id", 400),
    };

    let body: JoinRoomRequest = match req.json().await {
        Ok(b) => b,
        Err(_) => return Response::error("Bad request", 400),
    };

    // Get the DO stub for this room
    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(room_id)?
        .get_stub()?;

    // Forward join to DO
    let join_resp = do_stub.fetch_with_json(&body).await?;
    let participant_id: String = join_resp.text().await?;

    // Build signaling URL + TURN creds
    let signaling_url = format!("/api/meet/rooms/{}/signal", room_id);
    let turn_creds = crate::turns::generate_turn_credentials().await;

    let response = JoinResponse {
        participant_id,
        signaling_url,
        turn_credentials: TurnCredentials {
            urls: turn_creds.urls,
            username: turn_creds.username,
            credential: turn_creds.credential,
        },
    };

    Response::from_json(&response)
}

/// POST /api/meet/rooms/:room_id/leave — leave a room.
pub async fn leave_room(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = match ctx.param("room_id") {
        Some(id) => id,
        None => return Response::error("Missing room_id", 400),
    };

    let participant_id: String = match req.text().await {
        Ok(id) => id,
        Err(_) => return Response::error("Bad request", 400),
    };

    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(room_id)?
        .get_stub()?;

    let _ = do_stub
        .fetch_with_request(
            Request::new_with_init(
                &format!("/leave/{}", participant_id),
                RequestInit::new().with_method(Method::Post),
            )
            .unwrap(),
        )
        .await?;

    Response::ok("Left room")
}

/// POST /api/meet/rooms/:room_id/end — end a meeting (host only).
pub async fn end_room(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = match ctx.param("room_id") {
        Some(id) => id,
        None => return Response::error("Missing room_id", 400),
    };

    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(room_id)?
        .get_stub()?;

    let _ = do_stub
        .fetch_with_request(
            Request::new_with_init("/end", RequestInit::new().with_method(Method::Post)).unwrap(),
        )
        .await?;

    Response::ok("Room ended")
}

/// GET /api/meet/rooms/:room_id — get room details.
pub async fn get_room(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let room_id = match ctx.param("room_id") {
        Some(id) => id,
        None => return Response::error("Missing room_id", 400),
    };

    let do_stub = ctx
        .durable_object("MEET_ROOM")?
        .id_from_name(room_id)?
        .get_stub()?;

    let details = do_stub
        .fetch_with_request(
            Request::new_with_init("/details", RequestInit::new().with_method(Method::Get))
                .unwrap(),
        )
        .await?;

    Response::from_json(&details.json::<serde_json::Value>().await?)
}
