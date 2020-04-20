use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub config: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub provider: Provider,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    pub name: String,
    pub token: String,
    pub url: String,
}

pub fn load_config(filename: &str) -> Option<Config> {
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let app_config: Config = serde_yaml::from_str(&content).unwrap();
            Some(app_config)
        }
        Err(error) => panic!("There is an error {}: {}", filename, error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_loads_config_file() {
        let config = load_config("config.yaml");

        assert_eq!(config.is_some(), true);
        assert_eq!(config.unwrap().config.environment.unwrap().len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_it_doesnt_load_config_file() {
        load_config("configs.yaml");
    }
}
