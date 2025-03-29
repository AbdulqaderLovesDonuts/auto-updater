use serde::{Deserialize, Serialize};
use std::path::PathBuf;

//we will have a global, program and update config

//Global Configuration Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    pub update_interval: u64,
    pub log_level: String,
}

//Program Config Settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgramConfig {
    pub name: String,
    pub source: String,
    pub repo: String,

    #[serde(default = "default_update_channel")]
    pub update_channel: String,

    #[serde(default = "default_auto_update")]
    pub auto_update: bool,

    //Optional Custom Installation Path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_path: Option<PathBuf>,
}

// Top-level configuration structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateConfig {
    #[serde(flatten)]
    pub global: GlobalConfig,

    #[serde(default)]
    pub programs: Vec<ProgramConfig>,
}

/// Default configuration values
pub fn default_update_interval() -> u64 {
    3600 // 1 hour in seconds
}

pub fn default_log_level() -> String {
    "info".to_string()
}

pub fn default_update_channel() -> String {
    "stable".to_string()
}

pub fn default_auto_update() -> bool {
    false
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            global: GlobalConfig {
                update_interval: default_update_interval(),
                log_level: default_log_level(),
            },
            programs: Vec::new(),
        }
    }
}
