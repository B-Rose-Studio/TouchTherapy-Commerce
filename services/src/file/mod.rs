use domain::error::ErrorTrait;
use serde::{Deserialize, Serialize};

mod local_storage;
mod s3_storage;
pub use local_storage::*;
pub use s3_storage::*;

pub enum FileAction {
    Open {
        path: String,
    },
    Move {
        src: String,
        dst: String,
        copy: bool,
    },
    Delete {
        src: String,
    },
    Save {
        bytes: Vec<u8>,
        dst: String,
    },
}

fn content_type(key: &str) -> &str {
    let extension = key.split('.').next_back().unwrap_or("");

    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "pdf" => "application/pdf",
        "json" => "application/json",
        "txt" => "text/plain",
        "html" => "text/html",
        "mp4" => "video/mp4",
        "wav" => "audio/wav",
        "mp3" => "audio/mpeg",
        _ => "application/octet-stream",
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileServiceError(pub String);

impl ErrorTrait for FileServiceError {
    fn error<T: Sized + Serialize>(&self) -> domain::error::Error {
        domain::error::Error::new(
            "IO_SERVER_ERROR",
            format!("file service error: {}", self.0),
            (),
        )
    }
}
