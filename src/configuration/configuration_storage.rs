use std::error::Error;

pub trait ConfigurationProvider<T> {
    fn get_configuration(&self) -> Result<T, Box<dyn Error>>;
}

pub trait ConfigurationStorage<T> {
    fn store_configuration(&self, data: &T) -> Result<(), Box<dyn Error>>;
}
