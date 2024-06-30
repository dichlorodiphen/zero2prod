use actix_web::HttpResponse;

/// Handler for the health check endpoint.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
