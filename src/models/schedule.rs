use rust_decimal::Decimal;
use chrono::NaiveDate;
use serde::Serialize;

/// Represents a single payment in the mortgage schedule
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentScheduleEntry {
    /// Date when the payment is due
    pub payment_date: NaiveDate,
    
    /// Payment number in the sequence
    pub payment_number: u32,
    
    /// Total amount of this payment
    pub payment_amount: Decimal,
    
    /// Amount of payment going to principal
    pub principal_component: Decimal,
    
    /// Amount of payment going to interest
    pub interest_component: Decimal,
    
    /// Remaining principal after this payment
    pub remaining_principal: Decimal,
    
    /// Current interest rate (for variable rate mortgages)
    pub current_rate: Option<Decimal>,
}

/// Complete mortgage amortization schedule
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MortgageSchedule {
    /// Regular payment amount
    pub monthly_payment: Decimal,
    
    /// Sum of all payments over the loan term
    pub total_payments: Decimal,
    
    /// Total interest paid over the loan term
    pub total_interest: Decimal,
    
    /// Complete schedule of all payments
    pub schedule: Vec<PaymentScheduleEntry>,
}

impl MortgageSchedule {
    /// Creates a new empty mortgage schedule
    pub fn new(
        monthly_payment: Decimal,
        total_payments: Decimal,
        total_interest: Decimal,
    ) -> Self {
        Self {
            monthly_payment,
            total_payments,
            total_interest,
            schedule: Vec::new(),
        }
    }

    /// Adds a payment entry to the schedule
    pub fn add_payment(&mut self, entry: PaymentScheduleEntry) {
        self.schedule.push(entry);
    }

    /// Returns the total number of payments in the schedule
    pub fn payment_count(&self) -> usize {
        self.schedule.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_schedule_creation() {
        let schedule = MortgageSchedule::new(
            dec!(1000),
            dec!(360000),
            dec!(60000),
        );
        
        assert_eq!(schedule.monthly_payment, dec!(1000));
        assert_eq!(schedule.total_payments, dec!(360000));
        assert_eq!(schedule.total_interest, dec!(60000));
        assert_eq!(schedule.payment_count(), 0);
    }

    #[test]
    fn test_adding_payments() {
        let mut schedule = MortgageSchedule::new(
            dec!(1000),
            dec!(360000),
            dec!(60000),
        );

        let payment = PaymentScheduleEntry {
            payment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            payment_number: 1,
            payment_amount: dec!(1000),
            principal_component: dec!(800),
            interest_component: dec!(200),
            remaining_principal: dec!(299200),
            current_rate: None,
        };

        schedule.add_payment(payment);
        assert_eq!(schedule.payment_count(), 1);
    }
}
