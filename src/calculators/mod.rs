mod standard;
mod interest_only;
mod accelerated;
mod balloon;
mod floating;

pub use standard::StandardCalculator;
pub use interest_only::InterestOnlyCalculator;
pub use accelerated::AcceleratedCalculator;
pub use balloon::BalloonCalculator;
pub use floating::FloatingRateCalculator;

use crate::models::{MortgageInput, MortgageSchedule, MortgageSummary};

pub trait MortgageCalculator {
    fn calculate_schedule(input: &MortgageInput) -> MortgageSchedule;
    fn calculate_summary(input: &MortgageInput) -> MortgageSummary;
}
