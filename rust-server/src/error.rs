use actix_web::{error, HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    IoError(std::io::Error),
    UploadError(String),
    DownloadError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "I/O error: {}", e),
            AppError::UploadError(e) => write!(f, "Upload error: {}", e),
            AppError::DownloadError(e) => write!(f, "Download error: {}", e),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::IoError(_) => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            AppError::UploadError(msg) => {
                HttpResponse::BadRequest().json(format!("Upload failed: {}", msg))
            }
            AppError::DownloadError(msg) => {
                HttpResponse::NotFound().json(format!("Download failed: {}", msg))
            }
        }
    }
}
