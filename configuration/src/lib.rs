use std::error::Error;

pub mod config_file;
pub mod configuration_manager;
pub mod configuration_reader_error;
pub mod file_configuration_storage;

pub trait ConfigurationProvider<T> {
    fn get_configuration(&self) -> Result<T, Box<dyn Error>>;
}

pub trait ConfigurationStorage<T> {
    fn store_configuration(&self, data: &T) -> Result<(), Box<dyn Error>>;
}
