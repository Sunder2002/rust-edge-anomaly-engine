use thiserror::Error;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// These are the only things the whole system needs to agree on.
#[derive(Error, Debug)]
pub enum EngineError {
    #[error("Inference Engine Error: {0}")]
    InferenceError(String),

    #[error("Hardware/IO Failure: {0}")]
    IoError(String), // Changed to String to avoid linking std::io everywhere

    #[error("Video Capture Error: {0}")]
    VideoCaptureError(String),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub buffer: Vec<u8>, 
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detection {
    pub label: String,
    pub confidence: f32,
    pub bounding_box: [f32; 4],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnomalyContext {
    pub frame_id: u64,
    pub detections: Vec<Detection>,
    pub timestamp: DateTime<Utc>,
}