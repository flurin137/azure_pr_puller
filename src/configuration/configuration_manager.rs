use std::error::Error;

use super::{
    configuration_reader_error::ConfigurationReaderError,
    configuration_storage::{ConfigurationProvider, ConfigurationStorage},
};

pub struct ConfigurationManager<T> {
    configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>,
    configuration_storage: Vec<Box<dyn ConfigurationStorage<T>>>,
}

impl<T> ConfigurationManager<T> {
    pub fn new(
        configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>>,
        configuration_storage: Vec<Box<dyn ConfigurationStorage<T>>>,
    ) -> Self {
        ConfigurationManager {
            configuration_providers,
            configuration_storage,
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

    pub fn store_configuration(&self, data: &T) -> Result<(), Vec<Box<dyn Error>>> {
        let mut errors = vec![];

        for store in &self.configuration_storage {
            if let Err(error) = store.store_configuration(data) {
                errors.push(error)
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }
        Ok(())
    }
}
