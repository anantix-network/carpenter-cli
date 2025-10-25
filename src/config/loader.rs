use super::Config;
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct ConfigLoader {
    config_paths: Vec<PathBuf>,
    app_dir: PathBuf,
}

impl ConfigLoader {
    pub fn new() -> Result<Self> {
        let project = ProjectDirs::from("network", "anantix", "carpet-cli")
            .context("Failed to determine project directories")?;

        let app_dir = project.data_dir().to_path_buf();
        fs::create_dir_all(&app_dir)?;

        let config_paths = vec![
            PathBuf::from("carpet.toml"),
            PathBuf::from("carpet.yaml"),
            PathBuf::from("carpet.yml"),
        ];

        Ok(ConfigLoader { config_paths, app_dir })
    }

    pub fn load(&self) -> Result<Config> {
        for path in &self.config_paths {
            if path.exists() {
                return self.load_from_path(path);
            }
        }

        Ok(Config::new())
    }

    fn load_from_path(&self, path: &Path) -> Result<Config> {
        let content = fs::read_to_string(path)?;

        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => Ok(toml::from_str(&content)?),
            Some("yaml") | Some("yml") => Ok(serde_yaml::from_str(&content)?),
            _ => Err(anyhow::anyhow!("Unsupported config file format")),
        }
    }

    pub fn get_log_path(&self) -> PathBuf {
        self.app_dir.join("carpet.log")
    }
}
