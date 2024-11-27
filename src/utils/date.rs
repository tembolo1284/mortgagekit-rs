use chrono::{NaiveDate, Duration, Datelike};
use std::ops::Add;

/// Date-related utilities for mortgage calculations
pub struct DateUtils;

impl DateUtils {
    /// Calculates the payment dates for a mortgage
    pub fn generate_payment_dates(
        start_date: NaiveDate,
        num_payments: u32,
        is_biweekly: bool,
    ) -> Vec<NaiveDate> {
        let interval = if is_biweekly {
            Duration::days(14)
        } else {
            Duration::days(30)
        };

        (0..num_payments)
            .map(|payment_number| {
                start_date.add(interval * payment_number as i32)
            })
            .collect()
    }

    /// Calculates the number of days between two payment dates
    pub fn days_between_payments(date1: NaiveDate, date2: NaiveDate) -> i64 {
        (date2 - date1).num_days()
    }

    /// Returns the next payment date given a current date
    pub fn next_payment_date(current_date: NaiveDate, is_biweekly: bool) -> NaiveDate {
        if is_biweekly {
            current_date + Duration::days(14)
        } else {
            let mut next_month = current_date + Duration::days(30);
            if next_month.day() != current_date.day() {
                next_month = NaiveDate::from_ymd_opt(
                    next_month.year(),
                    next_month.month(),
                    current_date.day().min(next_month.day())
                ).unwrap_or(next_month);
            }
            next_month
        }
    }

    /// Validates if a date is a valid payment date
    pub fn is_valid_payment_date(date: NaiveDate) -> bool {
        let current_date = chrono::Local::now().date_naive();
        date >= current_date
    }
}
