//! Authentication module for handling user tokens and API interactions.
//!
//! This module provides functionality for:
//! - Token storage and management (save, retrieve, delete)
//! - API communication for user authentication and data retrieval
//! - Discord OAuth sign-in flow
//! - Error handling for authentication operations

pub mod api;
pub mod discord;
pub mod error;
pub mod models;
pub mod token;

/// Base URL for the authentication API
pub const API_BASE_URL: &str = "https://api.venxm.uk";
