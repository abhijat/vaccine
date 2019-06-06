use crate::{AuthType, ClientConfiguration};

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

#[cfg(test)]
mod config_builder {
    use super::*;

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
