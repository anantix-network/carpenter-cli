use crate::config::ConfigLoader;
use crate::script::ScriptRunner;
use anyhow::Result;
use inquire::Select;

pub async fn run(script_name: Option<String>, config_loader: &ConfigLoader) -> Result<()> {
    let config = config_loader.load()?;
    let script_names: Vec<&String> = config.scripts.keys().collect();

    if script_names.is_empty() {
        println!("No scripts found in configuration");
        return Ok(());
    }

    let script_to_run = if let Some(name) = script_name {
        name
    } else {
        let selection = Select::new("Select a script to run:", script_names).prompt()?;
        selection.to_string()
    };

    if let Some(script_config) = config.scripts.get(&script_to_run) {
        println!("Running script: {}", script_to_run);
        if let Some(desc) = &script_config.description {
            println!("Description: {}", desc);
        }
        let runner = ScriptRunner::new(script_config.clone());
        runner.run()?;
    } else {
        println!("Script '{}' not found in configuration", script_to_run);
    }

    Ok(())
}
