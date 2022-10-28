use std::{
    error::Error,
    path::{Path, PathBuf},
};

use super::configuration_reader_error::ConfigurationReaderError;

const CONFIG_FILE_NAME: &str = "configuration.json";

pub fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let current_exe = std::env::current_exe()?;
    let execution_folder = current_exe.parent().ok_or(ConfigurationReaderError {
        error_message: "Unable to find Execution Folder".to_owned(),
    })?;

    let config_file = Path::new(execution_folder).join(CONFIG_FILE_NAME);

    Ok(config_file)
}
