use std::env;
use std::collections::HashMap;

fn set_var(name: &str, value: &str) {
    env::set_var(name, value);
}

pub fn set_vars(map: HashMap<String, String>) {
    for item in map.iter() {
        set_var(item.0, item.1);
    }

    println!("Setting {} variables to environment", map.len());
}

pub fn get_envs(env_vars: Option<Vec<String>>) -> HashMap<String, String> {
    let mut envs = HashMap::new();
    for item in env_vars.unwrap().iter() {
        let item_split : Vec<&str> = item.split("=").collect();
        envs.insert(item_split[0].to_string(), item_split[1].to_string());
    }
    envs
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn test_it_sets_environment_variable_as_hashmaps() {
        let key = "MY_KEY";
        let value = "MY_VALUE";

        let mut secrets = HashMap::<String, String>::new();
        secrets.insert(key.to_string(), value.to_string());
        set_vars(secrets);

        assert_eq!(env::var(key), Ok(value.to_string()));
    }

    #[test]
    fn test_it_sets_environment_variable() {
        let key = "MY_KEY";
        let value = "MY_VALUE";

        set_var(key, value);

        assert_eq!(env::var(key), Ok(value.to_string()));
    }

    #[test]
    fn test_it_matches_vault() {
        let environment = Some(vec!["ONE_ENV=fake".to_string(), "TWO_ENV=not_fake".to_string()]);

        let envs = get_envs(environment);

        assert_eq!(2, envs.len());
        assert_eq!(&"fake".to_string(), envs.get("ONE_ENV").unwrap());
        assert_eq!(&"not_fake".to_string(), envs.get("TWO_ENV").unwrap());
    }
}
