mod notification_service;

use anyhow::Result;
use azure_work_lib::{
    azure::{Azure, PullRequestInformation},
    azure_configuration::AzureConfiguration,
    models::Repository,
};
use configuration::{file_configuration_storage::FileConfigurationStorage, ConfigurationProvider};
use notification_service::NotificationService;
use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use tauri::State;

struct ApplicationState {
    azure: Azure,
    repositories: Arc<Mutex<Vec<Repository>>>,
    notification_service: NotificationService,
}

#[tauri::command]
async fn get_pull_requests(
    state: State<'_, ApplicationState>,
) -> Result<PullRequestInformation, String> {
    println!("Called get_pull_requests");

    let repositories = state
        .repositories
        .lock()
        .map_err(|e| format!("{}", e))?
        .clone();

    let pull_request_information = state
        .azure
        .get_my_pull_requests(&repositories)
        .await
        .map_err(|d| format!("{}", d))?;

    Ok(pull_request_information)
}

#[tauri::command]
async fn load_repositories(state: State<'_, ApplicationState>) -> Result<(), String> {
    println!("Called load_repositories");

    let repositories = state
        .azure
        .get_repositories()
        .await
        .map_err(|e| format!("{}", e))?;
    println!("{:?}", repositories);

    let mut repositories_guard = state.repositories.lock().map_err(|e| format!("{}", e))?;

    *repositories_guard = repositories;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let file_config_povider =
        FileConfigurationStorage::new(Path::new("C:\\Dev\\bin\\configuration.json"));
    let configuration: AzureConfiguration = file_config_povider.get_configuration()?;

    let azure = Azure::new(configuration);
    let notification_service = NotificationService::new();

    let state = ApplicationState {
        azure,
        notification_service,
        repositories: Arc::new(Mutex::new(vec![])),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_pull_requests,
            load_repositories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
