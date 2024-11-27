use actix_web::{test, App};
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use mortgagekit_rs::{
    models::{MortgageInput, RepaymentType},
    api::configure_routes,
};

#[actix_web::test]
async fn test_full_mortgage_calculation() {
    // Initialize test application
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    // Create test input
    let input = MortgageInput {
        principal: dec!(300000),
        annual_interest_rate: dec!(0.05),
        term_years: 30,
        repayment_type: RepaymentType::StandardPrincipalAndInterest,
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        balloon_payment_percentage: dec!(0),
    };

    // Test full schedule calculation
    let req = test::TestRequest::post()
        .uri("/api/v1/calculate")
        .set_json(&input)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Parse and verify response
    let schedule: serde_json::Value = test::read_body_json(resp).await;
    assert!(schedule["schedule"].as_array().unwrap().len() == 360); // 30 years * 12 months
}

#[actix_web::test]
async fn test_all_repayment_types() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    let base_input = MortgageInput {
        principal: dec!(300000),
        annual_interest_rate: dec!(0.05),
        term_years: 30,
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        balloon_payment_percentage: dec!(0),
        repayment_type: RepaymentType::StandardPrincipalAndInterest,
    };

    // Test each repayment type
    for repayment_type in [
        RepaymentType::StandardPrincipalAndInterest,
        RepaymentType::InterestOnly,
        RepaymentType::AcceleratedBiweekly,
        RepaymentType::BalloonPayment,
        RepaymentType::FloatingRate,
    ].iter() {
        let mut input = base_input.clone();
        input.repayment_type = *repayment_type;

        if *repayment_type == RepaymentType::BalloonPayment {
            input.balloon_payment_percentage = dec!(20);
        }

        let req = test::TestRequest::post()
            .uri("/api/v1/calculate")
            .set_json(&input)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}

#[actix_web::test]
async fn test_invalid_inputs() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    // Test negative principal
    let invalid_input = MortgageInput {
        principal: dec!(-100000),
        annual_interest_rate: dec!(0.05),
        term_years: 30,
        repayment_type: RepaymentType::StandardPrincipalAndInterest,
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        balloon_payment_percentage: dec!(0),
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/calculate")
        .set_json(&invalid_input)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());

    // Test invalid interest rate
    let invalid_input = MortgageInput {
        principal: dec!(100000),
        annual_interest_rate: dec!(101), // Over 100%
        term_years: 30,
        repayment_type: RepaymentType::StandardPrincipalAndInterest,
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        balloon_payment_percentage: dec!(0),
    };

    let req = test::TestRequest::post()
        .uri("/api/v1/calculate")
        .set_json(&invalid_input)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_client_error());
}

#[actix_web::test]
async fn test_repayment_types_endpoint() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/repayment-types")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let types: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert!(!types.is_empty());
}

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(
        App::new().configure(configure_routes)
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/v1/health")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
