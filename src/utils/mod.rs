use log::LevelFilter;
use simplelog::{Config, WriteLogger};
use std::fs::File;
use std::path::Path;
use anyhow::Result;

pub fn setup_logging<P: AsRef<Path>>(log_path: P) -> Result<()> {
    let log_file = File::create(log_path)?;
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        log_file,
    )?;
    Ok(())
}
