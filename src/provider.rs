use crate::config::Config;

mod vault;

pub fn call_agent(config: Option<Config>) -> String {
    match config {
        Some(c) => match c.config.provider.name.as_ref() {
            "vault" => vault::init(c.config),
            _ => panic!("Got something else: {}", c.config.provider.name)
        }
        None => panic!("Unknown provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;
    use crate::config::Config;
    use crate::config::Provider;
    use crate::config::Data;

    #[test]
    fn test_it_matches_vault() {
        let config = load_config("config.yaml");
        let agent = call_agent(config);

        assert_eq!("vault", agent);
    }

    #[test]
    #[should_panic]
    fn test_it_matches_panic() {
        let config = None;
        call_agent(config);
    }

    #[test]
    #[should_panic]
    fn test_it_matches_panic_for_unknown_provider() {
        let config = Config{config: Data{ environment: None, provider: Provider{ name: "Name".to_string(), token: "".to_string(), url: "".to_string() }}};
        call_agent(Some(config));
    }
}
