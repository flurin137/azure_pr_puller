use anyhow::Result;
use azure_work_lib::azure_configuration::AzureConfiguration;
use configuration::ConfigurationProvider;

pub struct StdInConfigurationProvider;

impl StdInConfigurationProvider {
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl ConfigurationProvider<AzureConfiguration> for StdInConfigurationProvider {
    fn get_configuration(&self) -> Result<AzureConfiguration> {
        use std::io;
        let stdin = io::stdin();

        println!("Please enter Username (Use the name as shown in the Azure Profile)");
        let mut username = String::new();
        stdin.read_line(&mut username)?;
        username = username.trim().to_owned();

        println!("Please enter PAT (Azure Devops User Settings -> Personal Access Tokens)");
        let mut password = String::new();
        stdin.read_line(&mut password)?;
        password = password.trim().to_owned();

        println!("Please enter base url (format: https://dev.azure.com/your-company)");
        let mut url = String::new();
        stdin.read_line(&mut url)?;
        url = url.trim().to_owned();

        Ok(AzureConfiguration {
            password,
            username,
            url,
        })
    }
}
