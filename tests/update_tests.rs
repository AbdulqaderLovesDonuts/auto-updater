// src/updater/mod.rs

// Import the UpdateConfig struct from the config module
use auto_updater::config::schema::UpdateConfig;

#[cfg(test)]
mod tests {
    use super::*; // Import everything from the outer scope (updater module)

    #[test]
    fn test_import_update_config_default() {
        // Use the default function from the UpdateConfig struct
        let default_config = UpdateConfig::default();

        // Verify that the default config is created as expected
        assert_eq!(default_config.global.update_interval, 3600); // The default interval is 3600
        assert_eq!(default_config.global.log_level, "info".to_string()); // Default log level is "info"
        assert!(default_config.programs.is_empty()); // Default programs list should be empty
    }
}
