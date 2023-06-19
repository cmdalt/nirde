use actix_files::{Files,NamedFile};
use actix_web::{App, HttpServer,web,HttpRequest,Result};
use std::path::{Path,PathBuf};
use std::env;

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let static_path = env::var("STATIC_PATH").unwrap_or_else(|_| "../public".to_string());
    let path: PathBuf = format!("{}/index.html",static_path).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let static_path = env::var("STATIC_PATH").unwrap_or_else(|_| "../public".to_string());

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // note move so static_path is available in closure
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/",Path::new(&format!("{}/", static_path)),).index_file("index.html"))
            .default_service(web::get().to(index))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}