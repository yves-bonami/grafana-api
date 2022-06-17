use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    Grafana(GrafanaError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Hyper(e) => write!(f, "Hyper error: {}", e),
            Error::Serde(e) => write!(f, "Serde error: {}", e),
            Error::Io(e) => write!(f, "Io error: {}", e),
            Error::Grafana(e) => write!(f, "Grafana error: {}", e.message),
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::Hyper(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<GrafanaError> for Error {
    fn from(e: GrafanaError) -> Self {
        Error::Grafana(e)
    }
}

#[derive(Debug, Deserialize)]
pub struct GrafanaError {
    pub message: String,
}
