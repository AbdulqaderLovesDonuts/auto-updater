use super::schema::{ProgramConfig, UpdateConfig};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Parse configuration from a TOML file
pub fn parse_config<P: AsRef<Path>>(path: P) -> Result<UpdateConfig> {
    let config_path = path.as_ref();

    // Read the configuration file
    let config_contents = fs::read_to_string(config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

    // Parse TOML into UpdateConfig
    let config: UpdateConfig =
        toml::from_str(&config_contents).with_context(|| "Error parsing TOML configuration")?;

    // Validate the configuration
    validate_config(&config)?;

    Ok(config)
}

/// Validate the configuration
fn validate_config(config: &UpdateConfig) -> Result<()> {
    // Check for duplicate program names
    let unique_names: std::collections::HashSet<_> =
        config.programs.iter().map(|p| &p.name).collect();

    if unique_names.len() != config.programs.len() {
        return Err(anyhow::anyhow!(
            "Duplicate program names found in configuration"
        ));
    }

    // Validate update interval
    if config.global.update_interval == 0 {
        return Err(anyhow::anyhow!("Update interval must be greater than 0"));
    }

    // Validate log level
    let valid_log_levels = ["error", "warn", "info", "debug", "trace"];
    if !valid_log_levels.contains(&config.global.log_level.as_str()) {
        return Err(anyhow::anyhow!("Invalid log level"));
    }

    Ok(())
}

/// Generate a default configuration file
pub fn generate_default_config<P: AsRef<Path>>(path: P) -> Result<()> {
    let default_config = UpdateConfig::default();

    // Add some example programs
    let example_programs = vec![
        ProgramConfig {
            name: "VSCode".to_string(),
            source: "github".to_string(),
            repo: "microsoft/vscode".to_string(),
            update_channel: "stable".to_string(),
            auto_update: true,
            install_path: None,
        },
        ProgramConfig {
            name: "Firefox".to_string(),
            source: "github".to_string(),
            repo: "mozilla/firefox".to_string(),
            update_channel: "beta".to_string(),
            auto_update: false,
            install_path: None,
        },
    ];

    let mut config_to_write = default_config.clone();
    config_to_write.programs = example_programs;

    // Convert to TOML
    let toml_string = toml::to_string_pretty(&config_to_write)
        .context("Failed to serialize default configuration")?;

    // Write to file
    fs::write(path, toml_string).context("Failed to write default configuration file")?;

    Ok(())
}

/// Load configuration, creating default if not exists
pub fn load_config<P: AsRef<Path>>(path: P) -> Result<UpdateConfig> {
    let config_path = path.as_ref();

    // Create default config if file doesn't exist
    if !config_path.exists() {
        generate_default_config(config_path)?;
    }

    // Parse and return the configuration
    parse_config(config_path)
}
