

pub mod vault {
    extern crate hashicorp_vault as vault_api;
    use crate::config::Config;
    use std::collections::HashMap;

    use crate::environment::get_envs as config_envs;

    pub fn get_envs(c: Config) -> String {
        let host : &str = &c.config.provider.address;
        let envs = c.config.environment;
        #[cfg(not(test))]
        let client = vault_api::Client::new(host, c.config.provider.token).unwrap();
        let mut secrets = HashMap::<String, String>::new();
        let config_envs = config_envs(envs);
        for item in config_envs.keys() {
            let secret = client.get_secret(config_envs.get(item).unwrap()).unwrap();
            secrets.insert(item.to_string(), secret);
        }

        for (var, value) in secrets.iter() {
            println!("Calling {}: {}", var, value);
        }

        "vault".to_string()
    }
}
