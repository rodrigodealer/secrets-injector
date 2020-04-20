use std::env;

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

