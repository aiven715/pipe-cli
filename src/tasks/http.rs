use crate::tasks::{Task, TaskExecutionError};
use http::{header::USER_AGENT, HeaderMap, HeaderValue};
use reqwest::{Method, Url};
use serde_json::Value;

pub struct Http<'a> {
    name: &'a str,
    method: &'a str,
    url: &'a str,
}

impl<'a> Http<'a> {
    pub fn new(name: &'a str, method: &'a str, url: &'a str) -> Self {
        // TODO: create http client here so there could be a distinction between
        // init and execution errors?

        Self { name, method, url }
    }

    fn set_headers(headers: &mut HeaderMap) -> &mut HeaderMap {
        let pkg_name = env!("CARGO_PKG_NAME");
        let pkg_version = env!("CARGO_PKG_VERSION");

        let user_agent = HeaderValue::from_str(&format!("{}/{}", pkg_name, pkg_version)).unwrap();

        headers.insert(USER_AGENT, user_agent);

        headers
    }
}

impl<'a> Task for Http<'a> {
    fn name(&self) -> &str {
        self.name
    }

    // TODO: what of 4xx/5xx is returned?
    fn execute(&self) -> Result<Value, TaskExecutionError> {
        let parse_url_err = Err(TaskExecutionError(format!(
            "failed to parse {} url",
            self.url
        )));
        let execution_err = |err| {
            let msg = format!(
                "failed to send {} request of {} task. {}",
                self.url, self.name, err
            );

            Err(TaskExecutionError(msg))
        };
        let json_parse_err = |err| {
            Err(TaskExecutionError(format!(
                "failed to parse json response for {} task, {}",
                self.name, err
            )))
        };

        let method = match_method(self.method);
        let url = Url::parse(self.url).or(parse_url_err)?;

        let mut request = reqwest::blocking::Request::new(method, url);

        Self::set_headers(request.headers_mut());

        // TODO: avoid creating a new client at every request
        let client = reqwest::blocking::Client::builder().build().unwrap();
        let response = client.execute(request).or_else(execution_err)?;

        let json = response
            .json::<serde_json::Value>()
            .or_else(json_parse_err)?;

        Ok(json)
    }
}

fn match_method(method: &str) -> Method {
    match method.to_lowercase().as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "patch" => Method::PATCH,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        _ => panic!("unsupported http method"),
    }
}
