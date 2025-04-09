use anyhow::Result;

pub mod config_file;
pub mod configuration_manager;
pub mod configuration_manager_factory;
pub mod configuration_reader_error;
pub mod file_configuration_storage;

pub trait ConfigurationProvider<T> {
    fn get_configuration(&self) -> Result<T>;
}

pub trait ConfigurationStorage<T> {
    fn store_configuration(&self, data: &T) -> Result<()>;
}
