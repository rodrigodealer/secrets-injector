extern crate hashicorp_vault as vault_api;
use crate::config::Config;
use std::collections::HashMap;


pub fn get_envs(c: Config) -> String {
    let host : &str = &c.config.provider.address;
    let client = vault_api::Client::new(host, c.config.provider.token).unwrap();

    let mut secrets = HashMap::<String, String>::new();

    for item in c.config.environment.unwrap() {
        let vec: Vec<&str> = item.split('=').collect();
        let var_name = vec[0].to_string();
        let secret_name = vec[1];
        let secret = client.get_secret(secret_name).unwrap();
        secrets.insert(var_name, secret);
    }

    for (var, value) in secrets.iter() {
        println!("Calling {}: {}", var, value); 
    }

    "vault".to_string()
}
