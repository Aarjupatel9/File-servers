use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub upload_dir: String,
    pub max_upload_size: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let port = env::var("SERVER_PORT").unwrap_or_else(|_| "3031".to_string());
        let port = port.parse().unwrap_or(3031);
        
        let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
        
        let max_upload_size = env::var("MAX_UPLOAD_SIZE").unwrap_or_else(|_| "5242880".to_string());
        let max_upload_size = max_upload_size.parse().unwrap_or(5 * 1024 * 1024); // 5MB default
        
        Ok(Config {
            port,
            upload_dir,
            max_upload_size,
        })
    }
}
