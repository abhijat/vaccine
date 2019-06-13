#[macro_use]
extern crate serde_json;

use reqwest::{Client, RequestBuilder};
use serde_json::Value;

mod config_builder;

#[derive(Debug, PartialEq)]
pub enum AuthType {
    Basic,
    Bearer,
}

#[derive(Debug)]
pub struct RestClient {
    root_url: String,
    auth_type: Option<AuthType>,
    basic_auth: Option<(String, String)>,
    token: Option<String>,
}

impl RestClient {
    pub fn new(
        root_url: &str,
        auth_type: Option<AuthType>,
        basic_auth: Option<(&str, &str)>,
        token: Option<&str>,
    ) -> Self {
        RestClient {
            root_url: root_url.to_string(),
            auth_type,
            basic_auth: basic_auth.map(|(a, b)| (a.to_owned(), b.to_owned())),
            token: token.map(|token| token.to_owned()),
        }
    }

    pub fn qualify_url(&self, endpoint: &str) -> String {
        if self.root_url.ends_with("/") || endpoint.starts_with("/") {
            format!("{}{}", self.root_url.clone(), endpoint)
        } else {
            format!("{}/{}", self.root_url.clone(), endpoint)
        }
    }

    fn apply_auth_to_request(&self, auth_type: &AuthType, r: RequestBuilder) -> RequestBuilder {
        match auth_type {
            AuthType::Bearer => {
                let token = self.token
                    .clone()
                    .expect("using bearer auth but token missing from config!");
                r.bearer_auth(token)
            }
            AuthType::Basic => {
                let (username, password) = self.basic_auth
                    .clone()
                    .expect("using basic auth but username and password missing from config!");
                r.basic_auth(username, Some(password))
            }
        }
    }

    fn send(&self, request_builder: RequestBuilder, payload: &Value) -> Option<Value> {
        let request_builder = match &self.auth_type {
            None => request_builder,
            Some(auth_type) => self.apply_auth_to_request(auth_type, request_builder),
        };

        match request_builder.json(payload).send() {
            Ok(mut response) => response.json().expect("response is not JSON formatted!"),
            Err(err) => None,
        }
    }

    pub fn post(&self, url: &str, payload: &Value) -> Option<Value> {
        let request_builder = Client::new().post(&self.qualify_url(url));
        self.send(request_builder, payload)
    }

    pub fn patch(&self, url: &str, payload: &Value) -> Option<Value> {
        let request_builder = reqwest::Client::new().patch(&self.qualify_url(url));
        self.send(request_builder, payload)
    }

    pub fn get(&self, url: &str) -> Option<Value> {
        let get_request_builder = reqwest::Client::new().get(&self.qualify_url(url));
        let get_request_builder = match &self.auth_type {
            None => get_request_builder,
            Some(auth_type) => self.apply_auth_to_request(auth_type, get_request_builder),
        };

        match get_request_builder.send() {
            Ok(mut response) => response.json().expect("response is not JSON formatted!"),
            Err(err) => None,
        }
    }
}

#[cfg(test)]
mod rest_client {
    use mockito;

    use crate::{AuthType, RestClient};
    use crate::config_builder::ClientConfigurationBuilder;

    fn create_config() -> RestClient {
        ClientConfigurationBuilder::new()
            .root_url("http://localhost:8000")
            .build()
    }

    #[test]
    fn test_basic_auth_request() {
        let config = ClientConfigurationBuilder::new()
            .basic_auth("foo", "bar")
            .auth_type(AuthType::Basic)
            .root_url(&mockito::server_url())
            .build();

        // This is the default response. We should not get this!
        let invalid_request = mockito::mock("GET", "/")
            .with_body(r#"{"response": "bad!"}"#)
            .create();

        // We should end up here, because we set up basic auth
        let valid_request = mockito::mock("GET", "/")
            .match_header("authorization", "Basic Zm9vOmJhcg==")
            .with_body(r#"{"response": "hello"}"#)
            .create();

        let response = config.get("/").unwrap();
        assert_eq!("hello", response["response"].as_str().unwrap());
    }

    #[test]
    fn test_bearer_auth_request() {
        let config = ClientConfigurationBuilder::new()
            .token("xyz")
            .auth_type(AuthType::Bearer)
            .root_url(&mockito::server_url())
            .build();

        // This is the default response. We should not get this!
        let invalid_request = mockito::mock("GET", "/")
            .with_body(r#"{"response": "bad!"}"#)
            .create();

        // We should end up here, because we set up basic auth
        let valid_request = mockito::mock("GET", "/")
            .match_header("authorization", "Bearer xyz")
            .with_body(r#"{"response": "hello"}"#)
            .create();

        let response = config.get("/").unwrap();
        assert_eq!("hello", response["response"].as_str().unwrap());
    }
}
