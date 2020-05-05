extern crate hashicorp_vault as vault_api;
use crate::config::Provider;
use std::collections::HashMap;
use mocktopus::macros::*;

use crate::environment::get_envs as config_envs;
use crate::environment::set_vars;

pub struct Vault{}

pub struct Client{
    provider: Provider,
    connection: Option<vault_api::Client<vault_api::client::TokenData>>
}

impl Vault {
    pub fn get_envs(client: Client, envs: Option<Vec<String>>) -> String {
        let conn = client.get_client();
        let mut secrets = HashMap::<String, String>::new();
        let config_envs = config_envs(envs);
        for item in config_envs.keys() {
            match &conn.connection {
                Some(connection) => {
                    let secret = connection.get_secret(config_envs.get(item).unwrap()).unwrap();
                    secrets.insert(item.to_string(), secret);
                }
                _ => println!("No connection found")
            }
        }

        set_vars(secrets);

        "vault".to_string()
    }
}

#[mockable]
impl Client {
    pub fn new(p: Provider) -> Client {
        return Client{provider: p, connection: None}
    }

    fn get_client(mut self) -> Client {
        let host : &str = &self.provider.address;
        let token : &str = &self.provider.token;
        self.connection = Some(vault_api::Client::new(host, token).unwrap());
        return self;
    }
}

#[cfg(doc)]


#[cfg(test)]
use mockall::{automock, mock, predicate::*};

#[cfg(test)]
mod tests {
    use super::*;
    use mocktopus::mocking::*;

    #[test]
    fn test_it_matches_vault() {
        let client : Client = Client{connection: None, provider: Provider{name: "vaulty".to_string(), token: "bla".to_string(), address: "blu".to_string()}};
        let environment = Some(vec!["ONE_ENV=fake".to_string(), "TWO_ENV=not_fake".to_string()]);

        Client::get_client.mock_safe(|client| {
            MockResult::Return(client)
        });

        let expected = "vault".to_string();

        let result = Vault::get_envs(client, environment);

        assert_eq!(expected, result);

    }
}
