# mortgagekit-rs 🦀

A high-precision mortgage engine powered by Rust, built for speed and reliability.

## Features

- Multiple mortgage types supported:
  - Standard Principal and Interest
  - Interest Only
  - Accelerated Biweekly
  - Balloon Payment
  - Floating Rate
- High-precision decimal calculations
- Full amortization schedules
- REST API with OpenAPI documentation
- Comprehensive test coverage
- Docker support

## Quick Start

### Using the Library

#### Standard Principal and Interest
```rust
use mortgagekit_rs::{MortgageInput, RepaymentType, StandardCalculator};
use rust_decimal_macros::dec;
use chrono::NaiveDate;

let input = MortgageInput {
    principal: dec!(300000),
    annual_interest_rate: dec!(0.05),
    term_years: 30,
    repayment_type: RepaymentType::StandardPrincipalAndInterest,
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    balloon_payment_percentage: dec!(0),
};

let schedule = StandardCalculator::calculate_schedule(&input);
println!("Monthly Payment: ${}", schedule.monthly_payment);
```

#### Interest Only
```rust
let input = MortgageInput {
    principal: dec!(300000),
    annual_interest_rate: dec!(0.05),
    term_years: 30,
    repayment_type: RepaymentType::InterestOnly,
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    balloon_payment_percentage: dec!(0),
};

let schedule = InterestOnlyCalculator::calculate_schedule(&input);
println!("Monthly Interest Payment: ${}", schedule.monthly_payment);
println!("Final Principal Payment: ${}", input.principal);
```

#### Accelerated Biweekly
```rust
let input = MortgageInput {
    principal: dec!(300000),
    annual_interest_rate: dec!(0.05),
    term_years: 30,
    repayment_type: RepaymentType::AcceleratedBiweekly,
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    balloon_payment_percentage: dec!(0),
};

let schedule = AcceleratedCalculator::calculate_schedule(&input);
println!("Biweekly Payment: ${}", schedule.monthly_payment / dec!(2));
```

#### Balloon Payment
```rust
let input = MortgageInput {
    principal: dec!(300000),
    annual_interest_rate: dec!(0.05),
    term_years: 30,
    repayment_type: RepaymentType::BalloonPayment,
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    balloon_payment_percentage: dec!(20),
};

let schedule = BalloonCalculator::calculate_schedule(&input);
println!("Monthly Payment: ${}", schedule.monthly_payment);
println!("Balloon Payment: ${}", input.principal * dec!(0.20));
```

#### Floating Rate
```rust
let input = MortgageInput {
    principal: dec!(300000),
    annual_interest_rate: dec!(0.05), // Initial rate, will vary monthly
    term_years: 30,
    repayment_type: RepaymentType::FloatingRate,
    start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
    balloon_payment_percentage: dec!(0),
};

let schedule = FloatingRateCalculator::calculate_schedule(&input);
println!("Average Monthly Payment: ${}", schedule.monthly_payment);
```

### Using the API

```bash
# Start the server
cargo run

# Calculate a standard mortgage
curl -X POST http://localhost:8080/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "principal": 300000.00,
    "annualInterestRate": 0.0495,
    "termYears": 30,
    "repaymentType": "standardPrincipalAndInterest",
    "startDate": "2024-11-25"
  }'

# Calculate an interest-only mortgage
curl -X POST http://localhost:8080/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "principal": 300000.00,
    "annualInterestRate": 0.0495,
    "termYears": 30,
    "repaymentType": "interestOnly",
    "startDate": "2024-11-25"
  }'

# Calculate an accelerated biweekly mortgage
curl -X POST http://localhost:8080/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "principal": 300000.00,
    "annualInterestRate": 0.0495,
    "termYears": 30,
    "repaymentType": "acceleratedBiweekly",
    "startDate": "2024-11-25"
  }'

# Calculate a balloon payment mortgage
curl -X POST http://localhost:8080/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "principal": 300000.00,
    "annualInterestRate": 0.0495,
    "termYears": 30,
    "repaymentType": "balloonPayment",
    "balloonPaymentPercentage": 20.0,
    "startDate": "2024-11-25"
  }'

# Calculate a floating rate mortgage
curl -X POST http://localhost:8080/api/v1/calculate \
  -H "Content-Type: application/json" \
  -d '{
    "principal": 300000.00,
    "annualInterestRate": 0.0495,
    "termYears": 30,
    "repaymentType": "floatingRate",
    "startDate": "2024-11-25"
  }'

# Get available repayment types
curl -X GET http://localhost:8080/api/v1/repayment-types

# Check API health
curl -X GET http://localhost:8080/api/v1/health
```

### Using Docker

```bash
# Build and run with Docker Compose
docker-compose up --build

# Or use the production image directly
docker run -p 8080:8080 ghcr.io/yourusername/mortgagekit-rs:latest
```

## Installation

### From crates.io
```bash
cargo add mortgagekit-rs
```

### From source
```bash
git clone https://github.com/yourusername/mortgagekit-rs.git
cd mortgagekit-rs
cargo build --release
```

## API Documentation

API documentation is available at `http://localhost:8080/api/docs` when running the server.

### Available Endpoints

- `POST /api/v1/calculate` - Calculate full mortgage schedule
- `POST /api/v1/calculate/summary` - Calculate mortgage summary
- `GET /api/v1/repayment-types` - List available repayment types
- `GET /api/v1/health` - Health check endpoint

## Development

### Prerequisites

- Rust 1.75 or later
- Docker (optional)

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage
cargo llvm-cov

# Run integration tests only
cargo test --test integration_tests
```

### Local Development

```bash
# Start development server with auto-reload
cargo watch -x run

# Or use Docker Compose for development
docker-compose up dev
```
## Security

Please report security issues to [nik.lopez381@example.com](mailto:nik.lopez381@example.com).

## Acknowledgments

- Built with Rust and Actix-web
- Uses rust_decimal for precise financial calculations
- Inspired by modern mortgage calculation needs

## Citation

If you use this software in your research, please cite:

```bibtex
@software{mortgagekit_rs,
  author = {Paul Nikholas Lopez},
  title = {mortgagekit-rs: A High-Precision Mortgage Engine},
  year = {2024},
  url = {https://github.com/tembolo1284/mortgagekit-rs}
}
```

