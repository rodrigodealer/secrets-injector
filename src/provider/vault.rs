use crate::config::Data;

pub fn init(config: Data) -> String {
    let _envs = config.environment;
    println!("Calling vault {}", config.provider.name);
    config.provider.name
}
