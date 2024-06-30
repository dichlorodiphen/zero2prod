use crate::routes;
use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

/// Returns the HTTP server for our application running at the given address.
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
