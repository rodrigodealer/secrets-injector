extern crate hashicorp_vault as vault_api;
use crate::config::Provider;
use std::collections::HashMap;
use mockall::*;
use mockall::predicate::*;
#[cfg(test)]
use mocktopus::macros::*;

use crate::environment::get_envs as config_envs;

pub struct Vault{}

pub struct Client{
    provider: Provider,
    connection: Option<vault_api::Client<vault_api::client::TokenData>>
}

#[automock]
impl Vault {
    pub fn get_envs(client: Client, envs: Option<Vec<String>>) -> String {
        #[cfg(not(test))]
        let conn = client.get_client();
        let mut secrets = HashMap::<String, String>::new();
        let config_envs = config_envs(envs);
        for item in config_envs.keys() {
            #[cfg(not(test))]
            let secret = conn.get_secret(config_envs.get(item).unwrap()).unwrap();
            #[cfg(not(test))]
            secrets.insert(item.to_string(), secret);
        }

        for (var, value) in secrets.iter() {
            println!("Calling {}: {}", var, value);
        }

        "vault".to_string()
    }
}

#[mockable]
impl Client {
    pub fn new(p: Provider) -> Client {
        return Client{provider: p, connection: None}
    }

    fn get_client(&mut self) -> &Client {
        let host : &str = &self.provider.address;
        let token : &str = &self.provider.token;
        self.connection = Some(vault_api::Client::new(host, token).unwrap());
        return self;
    }

    fn foo(&self, bla: String) -> String {
        "bla".to_string()
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
    fn my_test() {
        let client : &Client = &Client{connection: None, provider: Provider{name: "vault".to_string(), token: "bla".to_string(), address: "blu".to_string()}};

        Client::get_client.mock_safe(|client| {
            MockResult::Return(client)
        });

    }

    // #[test]
    // fn mytest() {
    //     let provider = Provider{name: "vault".to_string(), token: "bla".to_string(), address: "blu".to_string()};
    //     let mut mock = MockClient::new(provider);
    //     mock.expect_foo()
    //         .times(1)
    //         .returning(|x| None );
    //     assert_eq!("bla".to_string(), mock.foo("bla".to_string()));
    // }

    // #[test]
    // fn mytest() {
    //     let provider = Provider{name: "vault".to_string(), token: "bla".to_string(), address: "blu".to_string()};
    //     let mut mock = MockClient::new(provider);
    //     mock.expect_foo()
    //         .with(eq("bla".to_string()))
    //         .times(1)
    //         .returning(|x| None );
    //     assert_eq!("bla".to_string(), mock.foo("bla".to_string()));
    // }
}
