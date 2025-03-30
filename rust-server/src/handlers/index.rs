use actix_web::{get, HttpResponse, Result};
use crate::templates;

#[get("/")]
pub async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(templates::index::render()))
}
