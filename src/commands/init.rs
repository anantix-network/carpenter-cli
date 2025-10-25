use crate::config::{Config, ScriptConfig};
use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn init(force: bool, yaml: bool) -> Result<()> {
    let config_file = if yaml { "carpet.yaml" } else { "carpet.toml" };

    if Path::new(config_file).exists() && !force {
        println!("Config file {} already exists. Use --force to overwrite.", config_file);
        return Ok(());
    }

    let mut scripts = HashMap::new();
    scripts.insert(
        "build".to_string(),
        ScriptConfig {
            command: "cargo build".to_string(),
            description: Some("Build the project".to_string()),
            env: None,
            working_dir: None,
        },
    );
    scripts.insert(
        "test".to_string(),
        ScriptConfig {
            command: "cargo test".to_string(),
            description: Some("Run tests".to_string()),
            env: Some(HashMap::from([("RUST_BACKTRACE".to_string(), "1".to_string())])),
            working_dir: None,
        },
    );
    scripts.insert(
        "run".to_string(),
        ScriptConfig {
            command: "cargo run".to_string(),
            description: Some("Run the project".to_string()),
            env: None,
            working_dir: Some("target/debug".into()),
        },
    );

    let config = Config { version: Some(env!("CARGO_PKG_VERSION").to_string()), scripts };

    let content = if yaml {
        serde_yaml::to_string(&config)?
    } else {
        toml::to_string_pretty(&config)?
            .replace("[[", "\n[[") 
            .replace("[scripts", "\n[scripts")
    };

    fs::write(config_file, content)?;
    println!("Created new config file: {}", config_file);

    Ok(())
}
