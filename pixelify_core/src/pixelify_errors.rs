use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PixelifyError {
    // For later use
}

#[derive(Debug)]
pub struct ImageProcessingError {
    pub op: &'static str,
    pub message: String,
}

impl ImageProcessingError {
    pub fn failed(op: &'static str, message: impl Into<String>) -> Self {
        Self {
            op,
            message: message.into(),
        }
    }
}

impl fmt::Display for ImageProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} failed: {}", self.op, self.message)
    }
}

impl Error for ImageProcessingError {}
