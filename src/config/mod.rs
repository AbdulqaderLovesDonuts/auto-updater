// Configuration module definition

// Expose public modules and types
pub mod parser; // TOML configuration parsing
pub mod schema; // Configuration data structures

// Re-export key types and functions
pub use parser::parse_config;
pub use schema::{GlobalConfig, ProgramConfig, UpdateConfig};

// Error handling for configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to parse configuration file")]
    ParseError,
    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}
