use crate::azure_configuration::AzureConfiguration;
use configuration::configuration_storage::ConfigurationProvider;
use std::error::Error;

pub struct StdInConfigurationProvider;

impl StdInConfigurationProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ConfigurationProvider<AzureConfiguration> for StdInConfigurationProvider {
    fn get_configuration(&self) -> Result<AzureConfiguration, Box<dyn Error>> {
        use std::io;
        let stdin = io::stdin();

        println!("Please enter Username (leave empty if PAT is used)");
        let mut username = String::new();
        stdin.read_line(&mut username)?;
        username = username.trim().to_owned();

        println!("Please enter Password or PAT");
        let mut password = String::new();
        stdin.read_line(&mut password)?;
        password = password.trim().to_owned();

        println!("Please enter base url (format: https://hamiltonreno.visualstudio.com)");
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
