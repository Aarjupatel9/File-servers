pub mod files;
pub mod index;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index::index)
        .service(files::upload)
        .service(files::download)
        .service(files::list_files);
}
