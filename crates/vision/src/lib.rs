use opencv::{prelude::*, videoio, core, imgcodecs};
use common::{Frame, EngineError};
use chrono::Utc;

/// VideoIngestor is our "Eye". It converts a raw video stream 
/// into a format our Rust engine can understand.
pub struct VideoIngestor {
    capture: videoio::VideoCapture,
    frame_count: u64,
}

impl VideoIngestor {
    /// Initialize the camera or video file.
    pub fn new(source: &str) -> Result<Self, EngineError> {
        let cap = videoio::VideoCapture::from_file(source, videoio::CAP_ANY)
            .map_err(|e| EngineError::VideoCaptureError(format!("Failed to open video: {}", e)))?;
        
        Ok(Self {
            capture: cap,
            frame_count: 0,
        })
    }

    /// Pull the next image and package it as a 'Frame'.
    pub fn next_frame(&mut self) -> Result<Option<Frame>, EngineError> {
        let mut mat = core::Mat::default();
        
        match self.capture.read(&mut mat) {
            Ok(true) => {
                if mat.empty() {
                    return Ok(None);
                }
                self.frame_count += 1;
                
                // Convert C++ image to a safe Rust Vector
                let mut buffer = core::Vector::<u8>::new();
                imgcodecs::imencode(".jpg", &mat, &mut buffer, &core::Vector::new())
                    .map_err(|e| EngineError::VideoCaptureError(e.to_string()))?;

                Ok(Some(Frame {
                    id: self.frame_count,
                    timestamp: Utc::now(),
                    buffer: buffer.to_vec(),
                    width: mat.cols() as u32,
                    height: mat.rows() as u32,
                }))
            }
            Ok(false) => Ok(None), // End of video
            Err(e) => Err(EngineError::VideoCaptureError(e.to_string())),
        }
    }
}