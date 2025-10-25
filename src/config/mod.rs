use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

mod loader;
pub use loader::ConfigLoader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptConfig {
    pub command: String,
    pub description: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub working_dir: Option<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: Option<String>,
    pub scripts: HashMap<String, ScriptConfig>,
}

impl Config {
    pub fn new() -> Self {
        Config { version: Some(env!("CARGO_PKG_VERSION").to_string()), scripts: HashMap::new() }
    }
}
