use crate::config::Config;

mod vault;

pub fn call_agent(config: Option<Config>) -> String {
    match config {
        Some(c) => match c.config.provider.name.as_ref() {
            "vault" => {
                println!("Got: {}", c.config.provider.name);
                let client = vault::Client::new(c.config.provider);
                vault::Vault::get_envs(client, c.config.environment)
            },
            _ => panic!("Got something else: {}", c.config.provider.name)
        }
        None => panic!("Unknown provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::config::Provider;
    use crate::config::Data;

    #[test]
    #[should_panic]
    fn test_it_matches_panic() {
        call_agent(None);
    }

    #[test]
    #[should_panic]
    fn test_it_matches_panic_for_unknown_provider() {
        let config = Config{config: Data{ environment: None, provider: Provider{ name: "Name".to_string(), token: "".to_string(), address: "".to_string(), access_key: None, secret_key: None}}};
        call_agent(Some(config));
    }
}
