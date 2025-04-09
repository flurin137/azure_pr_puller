use crate::{ConfigurationProvider, ConfigurationStorage};
use anyhow::Result;
use serde::{de, Serialize};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

pub struct FileConfigurationStorage {
    file_name: PathBuf,
}

impl FileConfigurationStorage {
    pub fn new(file_name: &Path) -> Self {
        Self {
            file_name: file_name.to_owned(),
        }
    }
}

impl<T: Serialize> ConfigurationStorage<T> for FileConfigurationStorage {
    fn store_configuration(&self, data: &T) -> Result<()> {
        let contents = serde_json::to_string_pretty(&data)?;

        std::fs::write(&self.file_name, contents)?;

        Ok(())
    }
}

impl<T: de::DeserializeOwned> ConfigurationProvider<T> for FileConfigurationStorage {
    fn get_configuration(&self) -> Result<T>
    where
        T: de::DeserializeOwned,
    {
        let file = File::open(&self.file_name)?;
        let reader = BufReader::new(file);

        let object = serde_json::from_reader(reader)?;

        Ok(object)
    }
}
