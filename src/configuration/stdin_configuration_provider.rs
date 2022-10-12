use std::error::Error;

use super::{configuration_reader::ConfigurationProvider, configuration::Configuration};

pub struct StdInConfigurationProvider;

impl StdInConfigurationProvider{
    pub fn new() -> Self{
        Self
    }
}

impl ConfigurationProvider<Configuration> for StdInConfigurationProvider {
    fn get_configuration(&self) -> Result<Configuration, Box<dyn Error>> {
        use std::io;
        let stdin = io::stdin();

        println!("Please enter Username (leave empty if PAT is used)");
        let mut username = String::new();
        stdin.read_line(&mut username)?;

        println!("Please enter Password or PAT");
        let mut password = String::new();
        stdin.read_line(&mut password)?;

        println!("Please enter base url (format: https://hamiltonreno.visualstudio.com)");
        let mut url = String::new();
        stdin.read_line(&mut url)?;

        Ok(Configuration {
            username,
            password,
            url,
        })
    }
}
