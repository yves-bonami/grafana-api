use hyper::{body::Buf, Body, Request, StatusCode, Uri};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

use crate::{common::Endpoint, error::GrafanaError};

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
    ) -> Result<E, crate::error::Error>
    where
        E: DeserializeOwned,
        E: Default,
    {
        let url = self.build_url(request);
        let token = if let AuthMethod::Bearer(token) = &self.auth_method.as_ref().unwrap() {
            token.clone()
        } else {
            String::new()
        };
        let req = Request::builder()
            .uri(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", token))
            .body(Body::empty())
            .unwrap();

        let https = HttpsConnector::new();
        let client = hyper::Client::builder().build::<_, hyper::Body>(https);
        let resp = client.request(req).await?;

        match resp.status() {
            StatusCode::OK => {
                let body = hyper::body::aggregate(resp).await?;
                let res = serde_json::from_reader(body.reader())?;
                Ok(res)
            }
            _ => {
                let body = hyper::body::aggregate(resp).await?;
                let res: GrafanaError = serde_json::from_reader(body.reader())?;
                Err(crate::error::Error::from(res))
            }
        }
    }

    fn build_url(&self, request: &dyn Endpoint) -> Uri {
        let uri: Uri = format!("{}{}", self.base_url, request.path())
            .parse()
            .expect("invalid url");

        let authority = match &self.auth_method {
            // Some(AuthMethod::Basic { username, password }) => {
            //     format!("{}:{}@{}", username, password, uri.authority().unwrap())
            // }
            // Some(AuthMethod::Bearer(token)) => {
            //     format!("api_key:{}@{}", token, uri.authority().unwrap())
            // }
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
