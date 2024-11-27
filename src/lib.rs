//! mortgagekit-rs: A high-precision mortgage engine powered by Rust
//! 
//! This library provides a comprehensive mortgage calculation engine
//! with support for various types of mortgages and a REST API interface.

pub mod api;
pub mod models;
pub mod calculators;
pub mod utils;

// Re-export commonly used items
pub use models::{
    MortgageInput,
    MortgageSchedule,
    MortgageSummary,
    PaymentScheduleEntry,
    RepaymentType,
    RepaymentTypeInfo,
};

pub use calculators::{
    MortgageCalculator,
    StandardCalculator,
    InterestOnlyCalculator,
    AcceleratedCalculator,
    BalloonCalculator,
    FloatingRateCalculator,
};

pub use api::{
    configure_routes,
    ApiError,
};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Creates a new mortgage calculation with default settings
pub fn new_mortgage_input(
    principal: rust_decimal::Decimal,
    annual_interest_rate: rust_decimal::Decimal,
    term_years: u32,
) -> MortgageInput {
    use chrono::Local;
    
    MortgageInput {
        principal,
        annual_interest_rate,
        term_years,
        repayment_type: RepaymentType::StandardPrincipalAndInterest,
        start_date: Local::now().date_naive(),
        balloon_payment_percentage: rust_decimal::Decimal::ZERO,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_new_mortgage_input() {
        let input = new_mortgage_input(
            dec!(300000),
            dec!(0.05),
            30
        );

        assert_eq!(input.principal, dec!(300000));
        assert_eq!(input.annual_interest_rate, dec!(0.05));
        assert_eq!(input.term_years, 30);
        assert_eq!(input.repayment_type, RepaymentType::StandardPrincipalAndInterest);
        assert_eq!(input.balloon_payment_percentage, dec!(0));
    }
}
