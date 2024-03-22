use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use fs::NamedFile;
use lazy_static::lazy_static;
use log::info;

lazy_static! {
    static ref STATIC_DIR: String = std::env::var("STATIC_DIR").unwrap();
}

async fn index() -> impl Responder {
    NamedFile::open(format!("{}/200.html", STATIC_DIR.as_str()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!(target: "server", "Server started successfully!");

    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/", STATIC_DIR.as_str())
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            // Automatically navigates to correct file
            .default_service(web::get().to(index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
