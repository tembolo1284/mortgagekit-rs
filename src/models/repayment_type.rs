use serde::{Deserialize, Serialize};

/// Available types of mortgage repayment
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RepaymentType {
    /// Standard principal and interest payments
    StandardPrincipalAndInterest,
    
    /// Interest-only payments with principal due at end
    InterestOnly,
    
    /// Accelerated biweekly payments
    AcceleratedBiweekly,
    
    /// Regular payments with balloon payment at end
    BalloonPayment,
    
    /// Variable rate mortgage
    FloatingRate,
}

impl std::fmt::Display for RepaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StandardPrincipalAndInterest => write!(f, "Standard Principal and Interest"),
            Self::InterestOnly => write!(f, "Interest Only"),
            Self::AcceleratedBiweekly => write!(f, "Accelerated Biweekly"),
            Self::BalloonPayment => write!(f, "Balloon Payment"),
            Self::FloatingRate => write!(f, "Floating Rate"),
        }
    }
}

impl RepaymentType {
    /// Returns all available repayment types
    pub fn all() -> Vec<RepaymentType> {
        vec![
            RepaymentType::StandardPrincipalAndInterest,
            RepaymentType::InterestOnly,
            RepaymentType::AcceleratedBiweekly,
            RepaymentType::BalloonPayment,
            RepaymentType::FloatingRate,
        ]
    }

    /// Returns whether this repayment type requires a balloon payment percentage
    pub fn requires_balloon_percentage(&self) -> bool {
        matches!(self, RepaymentType::BalloonPayment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            RepaymentType::StandardPrincipalAndInterest.to_string(),
            "Standard Principal and Interest"
        );
    }

    #[test]
    fn test_all_types() {
        assert_eq!(RepaymentType::all().len(), 5);
    }

    #[test]
    fn test_balloon_requirement() {
        assert!(RepaymentType::BalloonPayment.requires_balloon_percentage());
        assert!(!RepaymentType::StandardPrincipalAndInterest.requires_balloon_percentage());
    }
}
