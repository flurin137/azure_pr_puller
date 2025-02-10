mod notification_service;

use anyhow::Result;
use azure_work_lib::{azure::Azure, azure_configuration::AzureConfiguration};
use configuration::{file_configuration_storage::FileConfigurationStorage, ConfigurationProvider};
use notification_service::NotificationService;
use std::path::Path;
use tauri::State;

struct ApplicationState {
    azure: Azure,
    notification_service: NotificationService,
}

#[tauri::command]
fn greet(name: &str) -> String {
    "".to_owned()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let file_config_povider = FileConfigurationStorage::new(Path::new("./configuration.json"));
    let configuration: AzureConfiguration = file_config_povider.get_configuration()?;

    let azure = Azure::new(configuration);
    let notification_service = NotificationService::new();

    let state = ApplicationState {
        azure,
        notification_service,
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
