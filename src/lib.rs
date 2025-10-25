pub mod config;
pub mod env;
pub mod script;
pub mod utils;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_env_basic() {
        let mut env = env::EnvManager::new();
        #[cfg(feature = "test-utils")]
        env.set_for_test("TEST_KEY", "test_value");
        env.load().unwrap();
    }

    #[test]
    fn test_config_new() {
        let config = config::Config::new();
        assert!(config.scripts.is_empty());
        assert!(config.version.is_some());
    }

    #[test]
    fn test_script_config() {
        let mut env_vars = HashMap::new();
        env_vars.insert("TEST_VAR".to_string(), "test_value".to_string());

        let script_config = config::ScriptConfig {
            command: "echo test".to_string(),
            description: Some("Test script".to_string()),
            env: Some(env_vars),
            working_dir: None,
        };
        
        assert_eq!(script_config.command, "echo test");
        assert_eq!(script_config.description, Some("Test script".to_string()));
    }
}
