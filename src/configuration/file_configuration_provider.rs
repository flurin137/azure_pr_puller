use crate::configuration::configuration_reader::ConfigurationProvider;
use serde::de;
use std::{error::Error, fs::File, io::BufReader};

pub struct FileConfigurationProvider {
    file_name: String,
}

impl FileConfigurationProvider {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_owned(),
        }
    }
}

impl<T: de::DeserializeOwned> ConfigurationProvider<T> for FileConfigurationProvider {
    fn get_configuration(&self) -> Result<T, Box<dyn Error>>
    where
        T: de::DeserializeOwned,
    {
        let file = File::open(&self.file_name)?;
        let reader = BufReader::new(file);

        let object = serde_json::from_reader(reader)?;

        Ok(object)
    }
}
