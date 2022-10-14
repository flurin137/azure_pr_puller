use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AzureConfiguration {
    pub username: String,
    pub password: String,
    pub url: String,
}
