use rust_decimal::Decimal;
use serde::Serialize;
use super::repayment_type::RepaymentType;

/// Summary of mortgage calculation results
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MortgageSummary {
    /// Type of repayment used
    pub repayment_type: RepaymentType,
    
    /// Regular payment amount
    pub monthly_payment: Decimal,
    
    /// Total of all payments over loan term
    pub total_payments: Decimal,
    
    /// Total interest paid over loan term
    pub total_interest: Decimal,
    
    /// Total principal paid over loan term
    pub total_principal_paid: Decimal,
    
    /// Annual Percentage Rate
    pub apr: Decimal,
    
    /// Total number of payments
    pub number_of_payments: u32,
    
    /// Final balloon payment amount (if applicable)
    pub balloon_payment: Option<Decimal>,
    
    /// Interest rate range for variable rate mortgages
    pub rate_range: Option<(Decimal, Decimal)>,
}

impl MortgageSummary {
    /// Creates a new mortgage summary
    pub fn new(
        repayment_type: RepaymentType,
        monthly_payment: Decimal,
        total_payments: Decimal,
        total_interest: Decimal,
        total_principal_paid: Decimal,
        apr: Decimal,
        number_of_payments: u32,
    ) -> Self {
        Self {
            repayment_type,
            monthly_payment,
            total_payments,
            total_interest,
            total_principal_paid,
            apr,
            number_of_payments,
            balloon_payment: None,
            rate_range: None,
        }
    }

    /// Adds balloon payment information
    pub fn with_balloon_payment(mut self, amount: Decimal) -> Self {
        self.balloon_payment = Some(amount);
        self
    }

    /// Adds rate range information for variable rate mortgages
    pub fn with_rate_range(mut self, min_rate: Decimal, max_rate: Decimal) -> Self {
        self.rate_range = Some((min_rate, max_rate));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_summary_creation() {
        let summary = MortgageSummary::new(
            RepaymentType::StandardPrincipalAndInterest,
            dec!(1000),
            dec!(360000),
            dec!(60000),
            dec!(300000),
            dec!(0.05),
            360,
        );

        assert_eq!(summary.monthly_payment, dec!(1000));
        assert_eq!(summary.total_payments, dec!(360000));
        assert_eq!(summary.total_interest, dec!(60000));
        assert_eq!(summary.number_of_payments, 360);
    }

    #[test]
    fn test_summary_with_balloon() {
        let summary = MortgageSummary::new(
            RepaymentType::BalloonPayment,
            dec!(1000),
            dec!(360000),
            dec!(60000),
            dec!(300000),
            dec!(0.05),
            360,
        ).with_balloon_payment(dec!(50000));

        assert_eq!(summary.balloon_payment, Some(dec!(50000)));
    }
}
