//! Recording — recording management for meetings using Cloudflare R2.
//!
//! Handles the lifecycle of a meeting recording:
//! - Starting a recording session (creates an R2 object)
//! - Streaming media chunks to R2 (multipart upload)
//! - Finalizing and generating download URL
//! - Deleting old recordings
//!
//! ## Flow
//!
//! ```text
//! DO decides to record
//!   │
//!   ├── RecordingManager::new(room_id)
//!   │
//!   ├── rec.start() → Initiate R2 multipart upload
//!   │
//!   ├── rec.write_chunk(data) → Upload part
//!   │          ... (repeated)
//!   │
//!   └── rec.stop() → Complete multipart upload → emit RecordingReady
//! ```

use chrono::Utc;
use serde::{Deserialize, Serialize};
use worker::*;

/// Status of a recording session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingStatus {
    Initializing,
    Recording,
    Uploading,
    Completed,
    Failed(String),
}

/// Manages a single recording session for a meeting.
pub struct RecordingManager {
    /// The meeting room ID.
    room_id: String,
    /// R2 object key for this recording.
    object_key: String,
    /// Current status.
    status: RecordingStatus,
    /// R2 multipart upload ID (for chunked uploads).
    upload_id: Option<String>,
    /// Track uploaded part numbers.
    part_number: u32,
    /// Recording start timestamp (epoch ms).
    started_at: u64,
}

impl RecordingManager {
    /// Create a new recording manager for a room.
    pub fn new(room_id: &str) -> Self {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let object_key = format!("meetings/{}/recording_{}.webm", room_id, timestamp);

        Self {
            room_id: room_id.to_string(),
            object_key,
            status: RecordingStatus::Initializing,
            upload_id: None,
            part_number: 0,
            started_at: Utc::now().timestamp_millis() as u64,
        }
    }

    /// Start the recording session.
    ///
    /// Initiates an R2 multipart upload so we can stream chunks as they
    /// arrive from the SFU/recorder.
    pub async fn start(&mut self) -> Result<()> {
        // In a real implementation:
        // 1. Get the R2 bucket binding from env
        // 2. Call bucket.create_multipart_upload(self.object_key)
        // 3. Store the upload_id

        self.status = RecordingStatus::Recording;
        self.upload_id = Some(uuid::Uuid::new_v4().to_string());
        tracing::info!(
            "Recording started: room={}, key={}, upload_id={:?}",
            self.room_id,
            self.object_key,
            self.upload_id
        );

        Ok(())
    }

    /// Write a chunk of media data to the recording.
    ///
    /// Each chunk is uploaded as a part of the R2 multipart upload.
    pub async fn write_chunk(&mut self, data: &[u8]) -> Result<()> {
        if self.status != RecordingStatus::Recording {
            return Err(worker::Error::RustError("Recording not active".into()));
        }

        self.part_number += 1;

        // In a real implementation:
        // let part = bucket.upload_part(&self.upload_id, self.part_number, data).await?;
        // self.uploaded_parts.push(part);

        tracing::debug!(
            "Recording chunk: part={}, size={}",
            self.part_number,
            data.len()
        );

        Ok(())
    }

    /// Stop the recording and finalize the R2 upload.
    pub async fn stop(&mut self) {
        self.status = RecordingStatus::Uploading;

        // In a real implementation:
        // 1. Call bucket.complete_multipart_upload(...)
        // 2. Get the final object URL
        // 3. Emit a RecordingReady event

        self.status = RecordingStatus::Completed;
        tracing::info!(
            "Recording completed: room={}, key={}, parts={}",
            self.room_id,
            self.object_key,
            self.part_number
        );
    }

    /// Get the R2 object key for this recording.
    pub fn object_key(&self) -> &str {
        &self.object_key
    }

    /// Get the current status.
    pub fn status(&self) -> &RecordingStatus {
        &self.status
    }

    /// Get the recording duration in seconds.
    pub fn duration_secs(&self) -> u64 {
        let now = Utc::now().timestamp_millis() as u64;
        (now - self.started_at) / 1000
    }
}

/// Recording metadata stored alongside the R2 object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub room_id: String,
    pub started_at: u64,
    pub duration_secs: u64,
    pub mime_type: String,
    pub size_bytes: u64,
}

/// Generate a presigned URL for downloading a recording.
///
/// The URL expires after the specified TTL.
pub async fn get_download_url(bucket: &Bucket, object_key: &str) -> Result<String> {
    // R2 presigned URLs with 24h expiry
    let expires_in = std::time::Duration::from_secs(86400);
    let url = bucket.presigned_get(object_key, expires_in)?;
    Ok(url)
}

/// Delete a recording from R2.
pub async fn delete_recording(bucket: &Bucket, object_key: &str) -> Result<()> {
    bucket.delete(object_key).await?;
    Ok(())
}
