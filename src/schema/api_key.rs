use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiKey {
    pub id: u64,
    pub name: String,
    pub role: String,
    pub expiration: Option<DateTime<Utc>>,
}
