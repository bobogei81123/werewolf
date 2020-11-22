use crate::common::*;
use actix_files::NamedFile;
use actix_web::{dev::HttpServiceFactory, get, web, HttpRequest, HttpResponse, Responder};
use std::path::{Path, PathBuf};

#[get("/")]
async fn index_redirect(_: HttpRequest) -> impl Responder {
    HttpResponse::MovedPermanently()
        .header("Location", "/app/index.html")
        .finish()
}

#[get("/{filename:.*}")]
async fn app_static(req: HttpRequest) -> HttpResult<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = Path::new("dist").join(path);
    Ok(NamedFile::open(path).or_not_found()?)
}

pub fn get_service() -> impl HttpServiceFactory + 'static {
    web::scope("/app")
        .service(index_redirect)
        .service(app_static)
}
