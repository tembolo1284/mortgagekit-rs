use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::ops::RangeInclusive;

/// Common financial calculation utilities for decimal numbers
pub struct DecimalUtils;

impl DecimalUtils {
    /// Rounds a decimal to 2 decimal places
    pub fn round_currency(value: Decimal) -> Decimal {
        value.round_dp(2)
    }

    /// Converts a percentage to a decimal rate (e.g., 5.5% -> 0.055)
    pub fn percentage_to_rate(percentage: Decimal) -> Decimal {
        percentage / dec!(100)
    }

    /// Converts an annual rate to a monthly rate
    pub fn annual_to_monthly_rate(annual_rate: Decimal) -> Decimal {
        annual_rate / dec!(12)
    }

    /// Raises a decimal to an integer power
    pub fn power(base: Decimal, exp: i64) -> Decimal {
        if exp == 0 {
            return dec!(1);
        }
        let mut result = base;
        for _ in 1..exp {
            result *= base;
        }
        result
    }

    /// Checks if a decimal is within an inclusive range
    pub fn is_within_range(value: Decimal, range: RangeInclusive<Decimal>) -> bool {
        range.contains(&value)
    }

    /// Calculates the monthly payment factor for a given rate and term
    pub fn monthly_payment_factor(monthly_rate: Decimal, num_payments: u32) -> Decimal {
        let base = dec!(1) + monthly_rate;
        let factor = Self::power(base, num_payments as i64);
        (monthly_rate * factor) / (factor - dec!(1))
    }
}
