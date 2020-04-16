use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub config: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Data {
    provider: Provider,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<String>>,
}

pub fn load_config() -> Option<Config> {
    let filename = "config.yaml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let application_data: Config = serde_yaml::from_str(&content).unwrap();
            Some(application_data)
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
            None
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Provider {
    name: String,
    token: String,
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it_loads_config_file() {
        let config = load_config();

        assert_eq!(config.is_some(), true);
        assert_eq!(config.unwrap().config.environment.unwrap().len(), 2);
    }
}