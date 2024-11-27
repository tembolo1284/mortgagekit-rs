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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App, http};
    use rust_decimal_macros::dec;
    use chrono::NaiveDate;
    use crate::models::MortgageInput;
    use crate::models::RepaymentType;

    #[actix_web::test]
    async fn test_routes() {
        let app = test::init_service(
            App::new().configure(configure_routes)
        ).await;

        // Test health check
        let req = test::TestRequest::get()
            .uri("/api/v1/health")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Test repayment types
        let req = test::TestRequest::get()
            .uri("/api/v1/repayment-types")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Test mortgage calculation
        let input = MortgageInput {
            principal: dec!(300000),
            annual_interest_rate: dec!(0.05),
            term_years: 30,
            repayment_type: RepaymentType::StandardPrincipalAndInterest,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            balloon_payment_percentage: dec!(0),
        };

        let req = test::TestRequest::post()
            .uri("/api/v1/calculate")
            .set_json(&input)
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
