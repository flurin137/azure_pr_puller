use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub username: String,
    pub password: String,
    pub url: String,
}
