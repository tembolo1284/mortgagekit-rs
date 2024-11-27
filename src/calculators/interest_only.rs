use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Duration;

use crate::models::{MortgageInput, MortgageSchedule, PaymentScheduleEntry, MortgageSummary, RepaymentType};
use super::MortgageCalculator;

pub struct InterestOnlyCalculator;

impl MortgageCalculator for InterestOnlyCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule {
        let monthly_rate = input.annual_interest_rate / dec!(12);
        let num_payments = input.term_years * 12;
        let monthly_payment = input.principal * monthly_rate;
        
        let mut schedule = Vec::with_capacity(num_payments as usize);
        let mut total_interest = Decimal::ZERO;

        for payment_number in 1..=num_payments {
            let is_final_payment = payment_number == num_payments;
            let interest_component = monthly_payment;
            let principal_component = if is_final_payment {
                input.principal
            } else {
                Decimal::ZERO
            };
            
            total_interest += interest_component;

            let payment_amount = if is_final_payment {
                monthly_payment + input.principal
            } else {
                monthly_payment
            };

            let payment_date = input.start_date + 
                Duration::days(((payment_number - 1) * 30) as i64);

            schedule.push(PaymentScheduleEntry {
                payment_date,
                payment_number,
                payment_amount,
                principal_component,
                interest_component,
                remaining_principal: input.principal,
                current_rate: Some(input.annual_interest_rate),
            });
        }

        MortgageSchedule {
            monthly_payment,
            total_payments: (monthly_payment * Decimal::from(num_payments)) + input.principal,
            total_interest,
            schedule,
        }
    }

    fn calculate_summary(input: &MortgageInput) -> MortgageSummary {
        let schedule = Self::calculate_schedule(input);
        
        MortgageSummary::new(
            RepaymentType::InterestOnly,
            schedule.monthly_payment,
            schedule.total_payments,
            schedule.total_interest,
            input.principal,
            input.annual_interest_rate,
            input.term_years * 12,
        )
    }
}
