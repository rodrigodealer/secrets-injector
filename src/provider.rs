use crate::config::Config;

pub fn call_agent(config: Option<Config>) -> String {
    match config {
        Some(c) => match c.config.provider.name.as_ref() {
            "vault" => {
                println!("Got: {}", c.config.provider.name);
                c.config.provider.name
            },
            _ => panic!("Got something else: {}", c.config.provider.name)
        }
        None => panic!("Unknown provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

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
}
