use actix_web::{App, HttpServer};
use actix_cors::Cors;
use env_logger;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment logger with timestamp
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let host = "0.0.0.0";
    let port = 8080;
    
    info!("Starting mortgagekit-rs server on http://{}:{}", host, port);
    info!("Documentation available at http://{}:{}/api/docs", host, port);

    HttpServer::new(|| {
        // Configure CORS - in production, you'd want to restrict this
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            // Configure routes under /api/v1
            .configure(mortgagekit_rs::api::configure_routes)
            // Add middleware for logging, compression, etc.
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            // Add error handlers
            .app_data(actix_web::web::JsonConfig::default()
                .limit(4096) // Limit payload size to 4kb
                .error_handler(|err, _| {
                    let error_message = format!("JSON Error: {}", err);
                    actix_web::error::InternalError::from_response(
                        err,
                        actix_web::HttpResponse::BadRequest()
                            .json(error_message)
                    ).into()
                })
            )
    })
    .bind((host, port))?
    .run()
    .await
}
