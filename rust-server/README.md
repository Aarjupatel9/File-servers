# Rust Web Server

A robust file upload and download web server built with Rust and Actix Web.

## Features

- File upload with secure filename handling
- File listing with download links
- Clean HTML interface
- Protection against directory traversal attacks

## Setup

1. Install Rust and Cargo (https://rustup.rs)
2. Clone this repository
3. Run the server:

```bash
cargo run --release
```

4. Visit http://localhost:3031 in your browser

## Configuration

You can configure the server using environment variables or the `.env` file:

- `SERVER_PORT` - Port to listen on (default: 3031)
- `UPLOAD_DIR` - Directory to store uploads (default: ./uploads)
- `MAX_UPLOAD_SIZE` - Maximum upload size in bytes (default: 5242880)

## Project Structure

- `src/main.rs` - Application entry point
- `src/config.rs` - Configuration settings
- `src/handlers/` - Request handlers
- `src/services/` - Business logic
- `src/utils/` - Utility functions
- `src/templates/` - HTML templates

## License

MIT
