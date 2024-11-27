use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Duration;

use crate::models::{MortgageInput, MortgageSchedule, PaymentScheduleEntry, MortgageSummary, RepaymentType};
use crate::utils::DecimalUtils;
use super::MortgageCalculator;

pub struct StandardCalculator;

impl MortgageCalculator for StandardCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule {
        let monthly_rate = input.annual_interest_rate / dec!(12);
        let num_payments = input.term_years * 12;
        
        // Use DecimalUtils::power instead of powi
        let rate_factor = DecimalUtils::power(dec!(1) + monthly_rate, num_payments as i64);
        let monthly_payment = input.principal * 
            (monthly_rate * rate_factor) /
            (rate_factor - dec!(1));

        let mut schedule = Vec::with_capacity(num_payments as usize);
        let mut remaining_principal = input.principal;
        let mut total_interest = Decimal::ZERO;

        for payment_number in 1..=num_payments {
            let interest_component = remaining_principal * monthly_rate;
            let principal_component = monthly_payment - interest_component;
            
            total_interest += interest_component;
            remaining_principal -= principal_component;

            let payment_date = input.start_date + 
                Duration::days(((payment_number - 1) * 30) as i64);

            schedule.push(PaymentScheduleEntry {
                payment_date,
                payment_number,
                payment_amount: monthly_payment,
                principal_component,
                interest_component,
                remaining_principal,
                current_rate: Some(input.annual_interest_rate),
            });
        }

        MortgageSchedule {
            monthly_payment,
            total_payments: monthly_payment * Decimal::from(num_payments),
            total_interest,
            schedule,
        }
    }

    fn calculate_summary(input: &MortgageInput) -> MortgageSummary {
        let schedule = Self::calculate_schedule(input);
        
        MortgageSummary::new(
            RepaymentType::StandardPrincipalAndInterest,
            schedule.monthly_payment,
            schedule.total_payments,
            schedule.total_interest,
            input.principal,
            input.annual_interest_rate,
            input.term_years * 12,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_standard_calculator() {
        let input = MortgageInput {
            principal: dec!(300000),
            annual_interest_rate: dec!(0.05),
            term_years: 30,
            repayment_type: RepaymentType::StandardPrincipalAndInterest,
            start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            balloon_payment_percentage: dec!(0),
        };

        let schedule = StandardCalculator::calculate_schedule(&input);
        assert_eq!(schedule.schedule.len(), 360);
        
        let final_payment = schedule.schedule.last().unwrap();
        assert!(final_payment.remaining_principal.abs() < dec!(0.01));
    }
}
