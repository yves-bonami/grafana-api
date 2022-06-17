use std::collections::HashMap;

use hyper::Method;
use serde::Serialize;

use crate::{client::Client, common::Endpoint, schema::api_key::ApiKey};

#[derive(Debug, Default, Serialize)]
pub struct ListApiKeysRequest {
    /// Enable listing of expired keys.
    pub include_expired: Option<bool>,
}

impl Endpoint for ListApiKeysRequest {
    fn path(&self) -> String {
        String::from("/api/auth/keys")
    }

    fn method(&self) -> hyper::Method {
        Method::GET
    }

    fn params(&self) -> std::collections::HashMap<&str, String> {
        let mut params = HashMap::<&str, String>::new();

        self.include_expired
            .as_ref()
            .map(|v| params.insert("include_expired", v.to_string()));

        params
    }
}

impl ListApiKeysRequest {
    pub async fn send(&self, client: &Client) -> Result<Vec<ApiKey>, Box<dyn std::error::Error>> {
        let res = client.send_request::<Vec<ApiKey>>(self).await?;
        Ok(res)
    }
}
