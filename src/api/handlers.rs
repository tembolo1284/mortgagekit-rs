use actix_web::{web, HttpResponse};
use validator::Validate;

use crate::models::{MortgageInput, RepaymentType, RepaymentTypeInfo};
use crate::calculators::{
    MortgageCalculator,
    StandardCalculator,
    InterestOnlyCalculator,
    AcceleratedCalculator,
    BalloonCalculator,
    FloatingRateCalculator,
};
use super::errors::ApiError;

pub async fn calculate_mortgage(
    input: web::Json<MortgageInput>
) -> Result<HttpResponse, ApiError> {
    let input_data = input.into_inner();
    input_data.validate()?;

    let schedule = match input_data.repayment_type {
        RepaymentType::StandardPrincipalAndInterest => 
            StandardCalculator::calculate_schedule(&input_data),
        RepaymentType::InterestOnly => 
            InterestOnlyCalculator::calculate_schedule(&input_data),
        RepaymentType::AcceleratedBiweekly => 
            AcceleratedCalculator::calculate_schedule(&input_data),
        RepaymentType::BalloonPayment => 
            BalloonCalculator::calculate_schedule(&input_data),
        RepaymentType::FloatingRate =>
            FloatingRateCalculator::calculate_schedule(&input_data),
    };

    Ok(HttpResponse::Ok().json(schedule))
}

pub async fn calculate_mortgage_summary(
    input: web::Json<MortgageInput>
) -> Result<HttpResponse, ApiError> {
    let input_data = input.into_inner();
    input_data.validate()?;

    let summary = match input_data.repayment_type {
        RepaymentType::StandardPrincipalAndInterest => 
            StandardCalculator::calculate_summary(&input_data),
        RepaymentType::InterestOnly => 
            InterestOnlyCalculator::calculate_summary(&input_data),
        RepaymentType::AcceleratedBiweekly => 
            AcceleratedCalculator::calculate_summary(&input_data),
        RepaymentType::BalloonPayment => 
            BalloonCalculator::calculate_summary(&input_data),
        RepaymentType::FloatingRate =>
            FloatingRateCalculator::calculate_summary(&input_data),
    };

    Ok(HttpResponse::Ok().json(summary))
}

pub async fn get_repayment_types() -> Result<HttpResponse, ApiError> {
    Ok(HttpResponse::Ok().json(RepaymentTypeInfo::all()))
}

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
