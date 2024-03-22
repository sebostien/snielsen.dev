use actix_files as fs;
use actix_web::{web, App, HttpServer, Responder};
use fs::NamedFile;
use log::info;

const PORT: u16 = 8000;
const IP: &str = if cfg!(debug_assertions) {
    "127.0.0.1"
} else {
    "0.0.0.0"
};
const STATIC_DIR: &str = if cfg!(debug_assertions) {
    "./static"
} else {
    "/static"
};

async fn index() -> impl Responder {
    NamedFile::open(format!("{STATIC_DIR}/200.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!(target: "snielsen.dev", "Server listening on {IP}:{PORT}");
    info!(target: "snielsen.dev", "Using static files in {STATIC_DIR}");

    HttpServer::new(|| {
        App::new()
            .service(
                fs::Files::new("/", STATIC_DIR)
                    .index_file("index.html")
                    .use_last_modified(true),
            )
            // Automatically navigates to correct file
            .default_service(web::get().to(index))
    })
    .bind((IP, PORT))?
    .run()
    .await
}
