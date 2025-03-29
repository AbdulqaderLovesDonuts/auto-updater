// Configuration module definition

// Expose public modules and types
pub mod parser; // TOML configuration parsing
pub mod schema; // Configuration data structures

// Re-export key types and functions
pub use parser::{generate_default_config, load_config, parse_config, validate_config};
pub use schema::{
    GlobalConfig, ProgramConfig, UpdateConfig, default_auto_update, default_log_level,
    default_update_channel, default_update_interval,
};

// Error handling for configuration-related errors
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to parse configuration file")]
    ParseError,
    #[error("Invalid configuration: {0}")]
    ValidationError(String),
}
