mod config;
mod helper;
mod jobs;
mod models;
mod nodes;
mod state;

use crate::state::AppState;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::new().await.map_err(|e| {
        eprintln!("[FATAL] Could not load initial state: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
    })?;

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone())) // This is now correct!
            .configure(jobs::config)
            .configure(nodes::config)
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("Hello from Rust!") }),
            )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
