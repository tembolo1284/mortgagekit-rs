//! API module for the mortgage calculator
//! 
//! This module contains all the API-related functionality including
//! request handlers, route configuration, and error handling.

mod handlers;
mod routes;
mod errors;

pub use routes::configure_routes;
pub use errors::ApiError;

// Re-export handlers if needed for testing
#[cfg(test)]
pub use handlers::*;
