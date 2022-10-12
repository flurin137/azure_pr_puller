use std::error::Error;

use super::configuration_reader_error::ConfigurationReaderError;

pub trait ConfigurationProvider<T> {
    fn get_configuration(&self) -> Result<T, Box<dyn Error>>;
}

pub struct ConfigurationReader<T> {
    configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>,
}

impl<T> ConfigurationReader<T> {
    pub fn new(configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>) -> Self {
        ConfigurationReader {
            configuration_providers,
        }
    }

    pub fn get_configuration(&self) -> Result<T, Box<dyn Error>> {
        for provider in self.configuration_providers.iter() {
            if let Ok(configuration) = provider.get_configuration() {
                return Ok(configuration);
            }
        }

        let error_message = "".to_owned();
        Err(Box::new(ConfigurationReaderError { error_message }))
    }
}
