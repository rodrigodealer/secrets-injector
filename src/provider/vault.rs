extern crate hashicorp_vault as vault_api;
use crate::config::Provider;
use std::collections::HashMap;
use mockall::*;
use mockall::predicate::*;

use crate::environment::get_envs as config_envs;

pub struct Vault{}

pub struct Client{
    provider: Provider
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

#[automock]
impl Client {
    pub fn new(p: Provider) -> Client {
        return Client{provider: p}
    }

    fn get_client(&self) -> vault_api::Client<vault_api::client::TokenData> {
        let host : &str = &self.provider.address;
        let token : &str = &self.provider.token;
        return vault_api::Client::new(host, token).unwrap();
    }
}

// #[cfg(test)]
// use mockall::{automock, mock, predicate::*};

// #[cfg_attr(test, automock)]
// trait Vault {
//     fn foo(&self, x: String) -> String {
//         "bla".to_string()
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn mytest() {
//         let mut mock = MockClient::new();
//         mock.expect_get_client()
//             .times(1)
//             .returning(|x| None );
//         assert_eq!("bla".to_string(), mock.foo("bla".to_string()));
//     }
// }
