use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::repayment_type::RepaymentType;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct MortgageInput {
    #[validate(custom = "validate_principal")]
    pub principal: Decimal,
    
    #[validate(custom = "validate_rate")]
    pub annual_interest_rate: Decimal,
    
    #[validate(range(min = 1, max = 50))]
    pub term_years: u32,
    
    pub repayment_type: RepaymentType,
    pub start_date: NaiveDate,
    
    #[serde(default)]
    #[validate(custom = "validate_percentage")]
    pub balloon_payment_percentage: Decimal,
}

fn validate_principal(principal: &Decimal) -> Result<(), ValidationError> {
    if *principal > dec!(0) && *principal <= dec!(1_000_000_000) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_principal"))
    }
}

fn validate_rate(rate: &Decimal) -> Result<(), ValidationError> {
    if *rate >= dec!(0) && *rate <= dec!(100) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_rate"))
    }
}

fn validate_percentage(percentage: &Decimal) -> Result<(), ValidationError> {
    if *percentage >= dec!(0) && *percentage <= dec!(100) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_percentage"))
    }
}
