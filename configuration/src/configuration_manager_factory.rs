use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config_file, configuration_manager::ConfigurationManager,
    file_configuration_storage::FileConfigurationStorage, ConfigurationProvider,
};

pub fn get_configuration_manager<T>(
    fallback_provider: Box<dyn ConfigurationProvider<T>>,
) -> Result<ConfigurationManager<T>, Box<dyn Error>>
where
    T: DeserializeOwned,
    T: Serialize,
{
    let config_file = config_file::get_config_path()?;

    let configuration_providers: Vec<Box<dyn ConfigurationProvider<T>>> = vec![
        Box::new(FileConfigurationStorage::new(&config_file)),
        fallback_provider,
    ];

    let configuration_storages =
        Box::new(FileConfigurationStorage::new(&config_file));

    let config_reader =
        ConfigurationManager::<T>::new(configuration_providers, configuration_storages);

    Ok(config_reader)
}
