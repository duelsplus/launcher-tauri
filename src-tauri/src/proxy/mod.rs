//! Proxy management module.
//!
//! This module handles downloading, updating, and running the Duels+ proxy executable.

pub mod download;
pub mod error;
pub mod manager;
pub mod models;

pub use manager::ProxyManager;
