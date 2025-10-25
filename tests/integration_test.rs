use assert_fs::prelude::*;
use carpet_cli::config::ScriptConfig;
use carpet_cli::env::EnvManager;
use carpet_cli::script::ScriptRunner;
use std::collections::HashMap;

#[test]
fn test_env_manager_basic() {
    let mut env_manager = EnvManager::new();
    env_manager.set_for_test("TEST_KEY", "test_value");

    assert_eq!(env_manager.get("TEST_KEY"), Some(&"test_value".to_string()));
}

#[test]
fn test_env_manager_merge() {
    let mut env_manager = EnvManager::new();
    env_manager.set_for_test("GLOBAL_KEY", "global_value");

    let mut script_env = HashMap::new();
    script_env.insert("SCRIPT_KEY".to_string(), "script_value".to_string());

    let merged = env_manager.merge_with(Some(&script_env));

    assert_eq!(merged.get("GLOBAL_KEY"), Some(&"global_value".to_string()));
    assert_eq!(merged.get("SCRIPT_KEY"), Some(&"script_value".to_string()));
}

#[test]
fn test_script_runner_with_env() {
    let temp = assert_fs::TempDir::new().unwrap();
    let script_path = temp.child("test_script.sh").path().to_path_buf();

    std::fs::write(&script_path, "echo $TEST_VAR").unwrap();

    let mut env_manager = EnvManager::new();
    env_manager.set_for_test("TEST_VAR", "hello_test");

    let config = ScriptConfig {
        command: format!("sh {}", script_path.display()),
        description: None,
        env: None,
        working_dir: None,
    };

    let runner = ScriptRunner::with_env_manager(config, env_manager);
    assert!(runner.run().is_ok());
}

#[test]
fn test_config_loading() {
    let temp = assert_fs::TempDir::new().unwrap();

    // Create test TOML config
    let config_content = r#"
    version = "0.1.0"
    
    [scripts.test]
    command = "echo test"
    description = "Test command"
    
    [scripts.build]
    command = "cargo build"
    description = "Build project"
    "#;

    temp.child("carpet.toml").write_str(config_content).unwrap();

    let config_path = temp.child("carpet.toml").path().to_path_buf();

    // TODO: Implement actual config loading test when config module is ready
    assert!(config_path.exists());
}
