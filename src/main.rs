use std::env;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;

fn main() {
    let filename = "config.yaml";
    config::load_config(filename);
}

fn set_var(name: &str, value: &str) {
    env::set_var(name, value);
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_it_sets_environment_variable() {
        let key = "MY_KEY";
        let value = "MY_VALUE";

        set_var(key, value);

        assert_eq!(env::var(key), Ok(value.to_string()));
    }
}