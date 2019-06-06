#[macro_use]
extern crate serde_json;

use reqwest::RequestBuilder;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum AuthType {
    Basic,
    Bearer,
}

#[derive(Debug)]
pub struct ClientConfigurationBuilder {
    root_url: String,
    auth_type: Option<AuthType>,
    basic_auth: Option<(String, String)>,
    token: Option<String>,
}

impl ClientConfigurationBuilder {
    pub fn new() -> Self {
        ClientConfigurationBuilder {
            root_url: "".to_string(),
            auth_type: None,
            basic_auth: None,
            token: None,
        }
    }

    pub fn root_url(mut self, root_url: &str) -> ClientConfigurationBuilder {
        self.root_url = root_url.to_owned();
        self
    }

    pub fn auth_type(mut self, auth_type: AuthType) -> ClientConfigurationBuilder {
        self.auth_type = Some(auth_type);
        self
    }

    pub fn basic_auth(mut self, username: &str, password: &str) -> ClientConfigurationBuilder {
        self.basic_auth = Some((username.to_owned(), password.to_owned()));
        self
    }

    pub fn token(mut self, token: &str) -> ClientConfigurationBuilder {
        self.token = Some(token.to_owned());
        self
    }

    pub fn build(self) -> ClientConfiguration {
        ClientConfiguration {
            root_url: self.root_url,
            auth_type: self.auth_type,
            basic_auth: self.basic_auth,
            token: self.token,
        }
    }
}

#[derive(Debug)]
pub struct ClientConfiguration {
    root_url: String,
    auth_type: Option<AuthType>,
    basic_auth: Option<(String, String)>,
    token: Option<String>,
}

impl ClientConfiguration {
    pub fn new(
        root_url: &str,
        auth_type: Option<AuthType>,
        basic_auth: Option<(&str, &str)>,
        token: Option<&str>,
    ) -> Self {
        ClientConfiguration {
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

    fn apply_auth_to_request(&self, auth_type: &AuthType, &r: RequestBuilder) -> RequestBuilder {
        match auth_type {
            AuthType::Bearer => {
                let token = self.token
                    .clone()
                    .expect("using bearer auth but token missing from config!");
                request_builder.bearer_auth(token)
            }
            AuthType::Basic => {
                let (username, password) = self.basic_auth
                    .clone()
                    .expect("using basic auth but username and password missing from config!");
                request_builder.basic_auth(username, Some(password))
            }
        }
    }

    pub fn post(&self, client: &reqwest::Client, url: &str) -> reqwest::RequestBuilder {
        let request_builder = client.post(url);
        match &self.auth_type {
            None => request_builder,
            Some(auth_type) => self.apply_auth_to_request(auth_type, request_builder),
        }
    }
}

pub fn post(config: &ClientConfiguration, endpoint: &str, payload: &Value) -> Option<Value> {
    let url = config.qualify_url(endpoint);
    let client = reqwest::Client::new();

    match config.post(&client, url.as_str())
        .json(payload)
        .send() {
        Ok(mut response) => response.json().expect("response is not JSON formatted!"),
        Err(err) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::{AuthType, ClientConfiguration, ClientConfigurationBuilder, post};

    fn create_config() -> ClientConfiguration {
        ClientConfigurationBuilder::new()
            .root_url("http://localhost:8000")
            .build()
    }

    #[test]
    fn test_client_config_builder() {
        let config = ClientConfigurationBuilder::new()
            .basic_auth("foo", "bar")
            .auth_type(AuthType::Basic)
            .root_url("http://localhost:8000")
            .build();
        assert_eq!(&config.root_url, "http://localhost:8000");
        assert_eq!(config.auth_type.unwrap(), AuthType::Basic);
        assert_eq!(config.basic_auth, Some(("foo".to_owned(), "bar".to_owned())));
    }
}
