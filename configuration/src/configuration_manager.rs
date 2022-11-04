use std::error::Error;

use super::{
    configuration_reader_error::ConfigurationReaderError, ConfigurationProvider,
    ConfigurationStorage,
};

pub struct ConfigurationManager<T> {
    configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>,
    configuration_storage: Box<dyn ConfigurationStorage<T>>,
}

impl<T> ConfigurationManager<T> {
    pub fn new(
        configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>,
        configuration_storage: Box<dyn ConfigurationStorage<T>>,
    ) -> Self {
        ConfigurationManager {
            configuration_providers,
            configuration_storage,
        }
    }

    pub fn upsert_configuration(&self) -> Result<T, Box<dyn Error>> {
        for provider in self.configuration_providers.iter() {
            if let Ok(configuration) = provider.get_configuration() {
                self.configuration_storage
                    .store_configuration(&configuration)?;
                return Ok(configuration);
            }
        }

        let error_message = "Unable to retrieve configuration".to_owned();
        Err(Box::new(ConfigurationReaderError { error_message }))
    }
}
