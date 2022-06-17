use std::collections::HashMap;

use hyper::Method;

pub trait Endpoint {
    fn path(&self) -> String;
    fn method(&self) -> Method;
    fn params(&self) -> HashMap<&str, String>;
}
