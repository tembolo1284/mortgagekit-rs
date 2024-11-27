use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use rand::Rng;
use chrono::Duration;

use crate::models::{MortgageInput, MortgageSchedule, PaymentScheduleEntry, MortgageSummary, RepaymentType};
use super::MortgageCalculator;

pub struct FloatingRateCalculator;

impl MortgageCalculator for FloatingRateCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule {
        let num_payments = input.term_years * 12;
        let mut rng = rand::thread_rng();
        
        let mut schedule = Vec::with_capacity(num_payments as usize);
        let mut remaining_principal = input.principal;
        let mut total_interest = Decimal::ZERO;
        let mut total_payments = Decimal::ZERO;

        for payment_number in 1..=num_payments {
            let annual_rate = Decimal::from(rng.gen_range(1..=10));
            let monthly_rate = annual_rate / dec!(100) / dec!(12);
            
            let monthly_payment = remaining_principal * 
                (monthly_rate * (dec!(1) + monthly_rate)) /
                (dec!(1) + monthly_rate - dec!(1));
            
            let interest_component = remaining_principal * monthly_rate;
            let principal_component = monthly_payment - interest_component;
            
            total_interest += interest_component;
            total_payments += monthly_payment;
            remaining_principal -= principal_component;

            let payment_date = input.start_date + Duration::days(((payment_number - 1) * 30) as i64);
            
            schedule.push(PaymentScheduleEntry {
                payment_date,
                payment_number,
                payment_amount: monthly_payment,
                principal_component,
                interest_component,
                remaining_principal,
                current_rate: Some(annual_rate),
            });
        }

        let avg_monthly_payment = total_payments / Decimal::from(schedule.len());

        MortgageSchedule {
            monthly_payment: avg_monthly_payment,
            total_payments,
            total_interest,
            schedule,
        }
    }

    fn calculate_summary(input: &MortgageInput) -> MortgageSummary {
        let schedule = Self::calculate_schedule(input);
        
        MortgageSummary::new(
            RepaymentType::FloatingRate,
            schedule.monthly_payment,
            schedule.total_payments,
            schedule.total_interest,
            input.principal,
            dec!(5.5),
            input.term_years * 12,
        )
    }
}
