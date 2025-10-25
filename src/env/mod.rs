use anyhow::Result;
use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct EnvManager {
    env_files: Vec<PathBuf>,
    env_vars: HashMap<String, String>,
}

impl EnvManager {
    pub fn new() -> Self {
        Self { env_files: vec![PathBuf::from(".env")], env_vars: HashMap::new() }
    }

    pub fn add_env_file<P: AsRef<Path>>(&mut self, path: P) {
        self.env_files.push(path.as_ref().to_path_buf());
    }

    pub fn load(&mut self) -> Result<()> {
        // Load .env files
        for env_file in &self.env_files { if env_file.exists() { dotenv().ok(); } }
        // Collect all environment variables
        for (key, value) in env::vars() { self.env_vars.insert(key, value); }

        Ok(())
    }

    pub fn merge_with(
        &self,
        script_env: Option<&HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut final_env = self.env_vars.clone();
        if let Some(script_vars) = script_env {
            for (key, value) in script_vars {
                final_env.insert(key.clone(), value.clone());
            }
        }
        final_env
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.env_vars.get(key)
    }

    #[cfg(any(test, feature = "test-utils"))]
    pub fn set_for_test(&mut self, key: &str, value: &str) {
        self.env_vars.insert(key.to_string(), value.to_string());
    }
}
