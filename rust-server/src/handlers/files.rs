use crate::config;
use crate::error::AppError;
use crate::services::storage;
use crate::templates;
use crate::utils::sanitize;

use std::path::{Path,PathBuf};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{get, http::header, post, web,error, Error, HttpResponse, Result};
use futures::{StreamExt, TryStreamExt};
use log::{Log, info};
use std::io::Write;

fn get_unique_filename(directory: &str, filename: &str) -> String {
    let sanitized_name = sanitize::sanitize_filename(filename);
    let mut path = PathBuf::from(directory).join(&sanitized_name);
    
    let mut count = 1;
    while path.exists() {
        let file_stem = Path::new(&sanitized_name).file_stem().unwrap().to_string_lossy();
        let extension = Path::new(&sanitized_name)
            .extension()
            .map_or(String::new(), |e| format!(".{}", e.to_string_lossy()));

        let new_filename = format!("{} ({}){}", file_stem, count, extension);
        path = PathBuf::from(directory).join(&new_filename);
        count += 1;
    }

    path.to_string_lossy().to_string()
}

#[post("/upload")]
pub async fn upload(
    config: web::Data<config::Config>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    while let Some(mut field) = payload.try_next().await? {
        if let Some(orig_filename) = field.content_disposition().get_filename() {
            let unique_filename = get_unique_filename(&config.upload_dir, orig_filename);
        //    info!("unique_filename: {}", unique_filename);
            let mut file = std::fs::File::create(&unique_filename).map_err(|e| {
                AppError::UploadError(format!("Failed to create file: {}", e))
            })?;

            while let Some(chunk) = field.next().await {
                let data = chunk.map_err(|e| {
                    AppError::UploadError(format!("Failed to read chunk: {}", e))
                })?;

                file.write_all(&data).map_err(|e| {
                    AppError::UploadError(format!("Failed to write to file: {}", e))
                })?;
            }
        }
    }

    Ok(HttpResponse::SeeOther()
        .insert_header(("Location", "/files"))
        .finish())
}

// List available files
#[get("/files")]
pub async fn list_files(config: web::Data<config::Config>) -> Result<HttpResponse, Error> {
    let files = storage::list_files(&config.upload_dir).map_err(|e| {
        AppError::IoError(e)
    })?;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::files::render(files)))
}


// Handle file downloads
// #[get("/download/{filename}")]
// pub async fn download(
//     config: web::Data<config::Config>,
//     filename: web::Path<String>,
// ) -> Result<NamedFile, Error> {
//     let filepath = format!("{}/{}", config.upload_dir, filename);
//     storage::get_file(&filepath).map_err(|e| {
//         AppError::DownloadError(format!("Failed to download file: {}", e))
//     })
// }
#[get("/download/{filename}")]
async fn download(  config: web::Data<config::Config>,filename: web::Path<String>) -> Result<NamedFile> {
    let file_path = format!("{}/{}", &config.upload_dir, filename);
    let path = Path::new(&file_path);
    
    // Verify the path is within the uploads directory to prevent directory traversal
    let canonical_upload_dir = Path::new(&config.upload_dir).canonicalize()?;
    let canonical_file_path = path.canonicalize()?;
    
    if !canonical_file_path.starts_with(canonical_upload_dir) {
        return Err(error::ErrorForbidden("Access denied").into());
    }
    
    Ok(NamedFile::open(path)?)
}

