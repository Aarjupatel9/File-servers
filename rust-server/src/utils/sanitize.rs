/// Sanitizes a filename to make it safe for storage
pub fn sanitize_filename(filename: &str) -> String {
    sanitize_filename::sanitize(filename)
}
