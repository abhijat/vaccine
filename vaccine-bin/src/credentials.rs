use schema_parser::configuration::Session;
use rest_client::AuthType;
use std::env;

pub enum Credentials {
    Token(String),
    UsernamePassword {
        username: String,
        password: String,
    },
}

pub fn resolve_credentials(session: &Session) -> Option<Credentials> {
    let auth_type = session.auth_type;
    if let Some(auth_type) = auth_type {
        let c = match auth_type {
            AuthType::Basic => {
                let username_password = env::var("VACCINE_CREDENTIALS")
                    .expect("could not find vaccine token in env!");
                let (username, password) = username_password.split_at(
                    username_password.find(':').expect("Invalid username:password - no seperator ':'")
                );
                Credentials::UsernamePassword {
                    username: username.to_owned(),
                    password: password.to_owned(),
                }
            },
            AuthType::Bearer | AuthType::JWT => {
                let token = env::var("VACCINE_TOKEN")
                    .expect("could not find vaccine token in env!");
                Credentials::Token(token)
            },
        };
        Some(c)
    } else {
        None
    }
}
