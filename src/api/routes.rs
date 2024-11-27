use actix_web::web;
use super::handlers;

/// Configure all routes for the mortgage calculator API
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            // Mortgage calculation endpoints
            .service(
                web::resource("/calculate")
                    .route(web::post().to(handlers::calculate_mortgage))
            )
            .service(
                web::resource("/calculate/summary")
                    .route(web::post().to(handlers::calculate_mortgage_summary))
            )
            // Information endpoints
            .service(
                web::resource("/repayment-types")
                    .route(web::get().to(handlers::get_repayment_types))
            )
            // Health check endpoint
            .service(
                web::resource("/health")
                    .route(web::get().to(handlers::health_check))
            )
    );
}
