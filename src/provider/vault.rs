extern crate hashicorp_vault as vault_api;
use crate::config::Config;
use std::collections::HashMap;
use mockall::*;
use mockall::predicate::*;

use crate::environment::get_envs as config_envs;

pub struct Vault{}

pub struct Client{}

#[automock]
impl Vault {
    pub fn get_envs(c: Config) -> String {
        let host : &str = &c.config.provider.address;
        let envs = c.config.environment;
        #[cfg(not(test))]
        let client = Client::get_client(host, &c.config.provider.token);
        let mut secrets = HashMap::<String, String>::new();
        let config_envs = config_envs(envs);
        for item in config_envs.keys() {
            #[cfg(not(test))]
            let secret = client.get_secret(config_envs.get(item).unwrap()).unwrap();
            #[cfg(not(test))]
            secrets.insert(item.to_string(), secret);
        }

        for (var, value) in secrets.iter() {
            println!("Calling {}: {}", var, value);
        }

        "vault".to_string()
    }
}

impl Client {
    pub fn get_client(host: &str, token: &str) -> vault_api::Client<vault_api::client::TokenData> {
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
//         let mut mock = MockVault::new();
//         mock.expect_foo()
//             .with(eq("bla".to_string()))
//             .times(1)
//             .returning(|x| "bla".to_string());
//         assert_eq!("bla".to_string(), mock.foo("bla".to_string()));
//     }
// }
