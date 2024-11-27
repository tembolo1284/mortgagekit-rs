use serde::Serialize;
use super::repayment_type::RepaymentType;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepaymentTypeInfo {
    pub repayment_type: RepaymentType,
    pub name: String,
    pub description: String,
    pub requires_balloon_percentage: bool,
}

impl RepaymentTypeInfo {
    pub fn all() -> Vec<RepaymentTypeInfo> {
        vec![
            RepaymentTypeInfo {
                repayment_type: RepaymentType::StandardPrincipalAndInterest,
                name: "Standard Principal and Interest".to_string(),
                description: "Regular monthly payments of both principal and interest over the loan term.".to_string(),
                requires_balloon_percentage: false,
            },
            RepaymentTypeInfo {
                repayment_type: RepaymentType::InterestOnly,
                name: "Interest Only".to_string(),
                description: "Pay only interest during the loan term with full principal due at the end.".to_string(),
                requires_balloon_percentage: false,
            },
            RepaymentTypeInfo {
                repayment_type: RepaymentType::AcceleratedBiweekly,
                name: "Accelerated Biweekly".to_string(),
                description: "Payments every two weeks, resulting in one extra monthly payment per year.".to_string(),
                requires_balloon_percentage: false,
            },
            RepaymentTypeInfo {
                repayment_type: RepaymentType::BalloonPayment,
                name: "Balloon Payment".to_string(),
                description: "Regular payments with a large final balloon payment at the end of the term.".to_string(),
                requires_balloon_percentage: true,
            },
            RepaymentTypeInfo {
                repayment_type: RepaymentType::FloatingRate,
                name: "Floating Rate".to_string(),
                description: "Variable interest rate that changes monthly between 1-10% APR.".to_string(),
                requires_balloon_percentage: false,
            },
        ]
    }
}
