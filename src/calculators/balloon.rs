use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::Duration;

use crate::models::{MortgageInput, MortgageSchedule, PaymentScheduleEntry, MortgageSummary, RepaymentType};
use crate::utils::DecimalUtils;
use super::MortgageCalculator;

pub struct BalloonCalculator;

impl MortgageCalculator for BalloonCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule {
        let monthly_rate = input.annual_interest_rate / dec!(12);
        let num_payments = input.term_years * 12;
        
        let balloon_amount = input.principal * input.balloon_payment_percentage / dec!(100);
        let amortizing_amount = input.principal - balloon_amount;
        
        let rate_factor = DecimalUtils::power(dec!(1) + monthly_rate, num_payments as i64);
        let monthly_payment = amortizing_amount * 
            (monthly_rate * rate_factor) /
            (rate_factor - dec!(1));

        let mut schedule = Vec::with_capacity(num_payments as usize);
        let mut remaining_principal = input.principal;
        let mut total_interest = Decimal::ZERO;

        for payment_number in 1..=num_payments {
            let is_final_payment = payment_number == num_payments;
            let interest_component = remaining_principal * monthly_rate;
            let mut principal_component = monthly_payment - interest_component;
            
            if is_final_payment {
                principal_component += balloon_amount;
            }
            
            total_interest += interest_component;
            remaining_principal -= principal_component;

            let payment_amount = if is_final_payment {
                monthly_payment + balloon_amount
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
                remaining_principal,
                current_rate: Some(input.annual_interest_rate),
            });
        }

        MortgageSchedule {
            monthly_payment,
            total_payments: (monthly_payment * Decimal::from(num_payments - 1)) + 
                          (monthly_payment + balloon_amount),
            total_interest,
            schedule,
        }
    }

    fn calculate_summary(input: &MortgageInput) -> MortgageSummary {
        let schedule = Self::calculate_schedule(input);
        let balloon_amount = input.principal * input.balloon_payment_percentage / dec!(100);
        
        MortgageSummary::new(
            RepaymentType::BalloonPayment,
            schedule.monthly_payment,
            schedule.total_payments,
            schedule.total_interest,
            input.principal,
            input.annual_interest_rate,
            input.term_years * 12,
        ).with_balloon_payment(balloon_amount)
    }
}
