mod console_writer;
mod stdin_configuration_provider;

use crate::stdin_configuration_provider::StdInConfigurationProvider;
use azure_work_lib::azure::Azure;
use configuration::configuration_manager_factory::get_configuration_manager;
use console_writer::ConsoleWriter;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let fallback_configuration_provider = StdInConfigurationProvider::new_boxed();
    let configuration_manager = get_configuration_manager(fallback_configuration_provider)?;

    let config = configuration_manager.upsert_configuration()?;

    let azure = Azure::new(&config);
    let repositories = azure.get_repositories().await?;

    let pull_request_information = azure.get_my_pull_requests(&repositories).await?;

    let console_writer = ConsoleWriter::new(&azure);

    console_writer
        .print_pull_request_information(&pull_request_information)
        .await;

    Ok(())
}
