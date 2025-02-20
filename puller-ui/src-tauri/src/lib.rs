mod notification_service;

use anyhow::Result;
use azure_work_lib::{
    azure::Azure,
    azure_configuration::AzureConfiguration,
    models::{PullRequest, Repository},
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
async fn get_pull_requests(state: State<'_, ApplicationState>) -> Result<Vec<PullRequest>, String> {
    let mut pull_requests = vec![];
    let repositories;
    {
        repositories = state
            .repositories
            .lock()
            .map_err(|e| format!("{}", e))?
            .clone();
    }

    for repository in repositories.iter() {
        let repo_pull_requests = state
            .azure
            .get_pull_requests(repository)
            .await
            .map_err(|d| format!("{}", d))?;

        for pull_request in repo_pull_requests {
            pull_requests.push(pull_request);
        }
    }

    Ok(pull_requests)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> Result<()> {
    let file_config_povider = FileConfigurationStorage::new(Path::new("C:\\Dev\\bin\\configuration.json"));
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
        .invoke_handler(tauri::generate_handler![get_pull_requests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
