use std::{error::Error, fs::File, io::BufReader};

use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub url: String,
}

pub struct ConfigurationReader {
    file_name: String,
}

impl ConfigurationReader {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_owned(),
        }
    }

    pub fn read_configuration<T>(&self) -> Result<T, Box<dyn Error>>
    where
        T: de::DeserializeOwned,
    {
        let file = File::open(&self.file_name)?;
        let reader = BufReader::new(file);

        let object = serde_json::from_reader(reader)?;

        Ok(object)
    }
}
