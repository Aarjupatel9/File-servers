use actix_files::NamedFile;
use std::fs::{self, File};
use std::io;
use std::path::Path;

/// Ensures the upload directory exists
pub fn ensure_upload_dir(dir: &str) -> io::Result<()> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}

/// Lists all files in a directory
pub fn list_files(dir: &str) -> io::Result<Vec<String>> {
    // Ensure directory exists
    ensure_upload_dir(dir)?;
    
    let mut files = Vec::new();
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(file_name) = entry.file_name().to_str() {
                        files.push(file_name.to_string());
                    }
                }
            }
        }
    }
    
    // Sort files alphabetically
    files.sort();
    Ok(files)
}

/// Gets a file for download, with security checks
pub fn get_file(filepath: &str) -> io::Result<NamedFile> {
    let path = Path::new(filepath);
    
    // Security check: verify path is an absolute path and exists
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "File not found",
        ));
    }
    
    // Get canonical paths to check if the file is within the uploads directory
    let canonical_path = path.canonicalize()?;
    let parent_dir = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path")
    })?;
    let canonical_parent = parent_dir.canonicalize()?;
    
    // Security check: verify file is within the uploads directory
    if !canonical_path.starts_with(canonical_parent) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Access denied",
        ));
    }
    
    NamedFile::open(path)
}
