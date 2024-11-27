use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Duration;

use crate::models::{MortgageInput, MortgageSchedule, PaymentScheduleEntry, MortgageSummary, RepaymentType};
use crate::utils::DecimalUtils;
use super::MortgageCalculator;

pub struct AcceleratedCalculator;

impl MortgageCalculator for AcceleratedCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule {
        let monthly_rate = input.annual_interest_rate / dec!(12);
        let biweekly_rate = monthly_rate / dec!(2);
        let num_payments = input.term_years * 26;
        
        let rate_factor = DecimalUtils::power(dec!(1) + monthly_rate, (input.term_years * 12) as i64);
        let monthly_equivalent = input.principal * 
            (monthly_rate * rate_factor) /
            (rate_factor - dec!(1));
        let biweekly_payment = monthly_equivalent / dec!(2);

        let mut schedule = Vec::with_capacity(num_payments as usize);
        let mut remaining_principal = input.principal;
        let mut total_interest = Decimal::ZERO;

        for payment_number in 1..=num_payments {
            let interest_component = remaining_principal * biweekly_rate;
            let principal_component = biweekly_payment - interest_component;
            
            total_interest += interest_component;
            remaining_principal -= principal_component;

            let payment_date = input.start_date + 
                Duration::days(((payment_number - 1) * 14) as i64);

            schedule.push(PaymentScheduleEntry {
                payment_date,
                payment_number,
                payment_amount: biweekly_payment,
                principal_component,
                interest_component,
                remaining_principal,
                current_rate: Some(input.annual_interest_rate),
            });
        }

        MortgageSchedule {
            monthly_payment: biweekly_payment * dec!(2),
            total_payments: biweekly_payment * Decimal::from(num_payments),
            total_interest,
            schedule,
        }
    }

    fn calculate_summary(input: &MortgageInput) -> MortgageSummary {
        let schedule = Self::calculate_schedule(input);
        
        MortgageSummary::new(
            RepaymentType::AcceleratedBiweekly,
            schedule.monthly_payment,
            schedule.total_payments,
            schedule.total_interest,
            input.principal,
            input.annual_interest_rate,
            input.term_years * 26,
        )
    }
}
