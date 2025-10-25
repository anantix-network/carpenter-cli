use crate::config::ScriptConfig;
use crate::env::EnvManager;
use anyhow::Result;
use std::process::Command;

pub struct ScriptRunner {
    config: ScriptConfig,
    env_manager: EnvManager,
}

impl ScriptRunner {
    pub fn new(config: ScriptConfig) -> Self {
        let mut env_manager = EnvManager::new();
        env_manager.load().unwrap_or_default();

        ScriptRunner { config, env_manager }
    }

    pub fn with_env_manager(config: ScriptConfig, env_manager: EnvManager) -> Self {
        ScriptRunner { config, env_manager }
    }

    pub fn run(&self) -> Result<()> {
        let mut command = if cfg!(target_os = "windows") {
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &self.config.command]);
            cmd
        } else {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &self.config.command]);
            cmd
        };
        
        // Merge environment variables
        let env_vars = self.env_manager.merge_with(self.config.env.as_ref());
        command.envs(&env_vars);
        // Set working directory if specified
        if let Some(dir) = &self.config.working_dir { command.current_dir(dir); }
        let status = command.spawn()?.wait()?;

        if !status.success() { return Err(anyhow::anyhow!("Script failed with exit code: {:?}", status.code())); }
        Ok(())
    }
}
