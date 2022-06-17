use hyper::{body::Buf, Body, Request, Uri};
use serde::de::DeserializeOwned;

use crate::common::Endpoint;

#[derive(Debug)]
pub struct Client {
    base_url: String,
    auth_method: Option<AuthMethod>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: String::from(base_url),

            auth_method: None,
        }
    }

    pub fn set_auth_method(mut self, auth_method: AuthMethod) -> Self {
        self.auth_method = Some(auth_method);
        self
    }

    pub(crate) async fn send_request<E>(
        &self,
        request: &dyn Endpoint,
    ) -> Result<E, Box<dyn std::error::Error>>
    where
        E: DeserializeOwned,
        E: Default,
    {
        let url = self.build_url(request);
        let req = Request::builder()
            .uri(url)
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .unwrap();

        let client = hyper::Client::new();
        let resp = client.request(req).await?;

        let body = hyper::body::aggregate(resp).await?;
        let res = serde_json::from_reader(body.reader())?;
        Ok(res)
    }

    fn build_url(&self, request: &dyn Endpoint) -> Uri {
        let uri: Uri = format!("{}{}", self.base_url, request.path())
            .parse()
            .expect("invalid url");

        let authority = match &self.auth_method {
            Some(AuthMethod::Basic { username, password }) => {
                format!("{}:{}@{}", username, password, uri.authority().unwrap())
            }
            Some(AuthMethod::Bearer(token)) => {
                format!("api_key:{}@{}", token, uri.authority().unwrap())
            }
            _ => {
                format!("{}", uri.authority().unwrap())
            }
        };

        let path_and_query = format!(
            "{}?{}",
            uri.path_and_query().unwrap(),
            request
                .params()
                .iter()
                .fold(String::new(), |mut acc, (k, v)| {
                    acc.push_str(&format!("{}={}&", k, v));
                    acc
                })
        );

       Uri::builder()
            .scheme(uri.scheme_str().unwrap())
            .authority(authority)
            .path_and_query(path_and_query)
            .build()
            .expect("invalid url")
    }
}

#[derive(Debug)]
pub enum AuthMethod {
    Basic { username: String, password: String },
    Bearer(String),
}
