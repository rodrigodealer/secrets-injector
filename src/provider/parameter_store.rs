use crate::config::Provider;
use mocktopus::macros::*;

pub struct ParameterStore {
}

pub struct Client{
    provider: Provider,
}

impl ParameterStore {
    pub fn get_envs(envs: Option<Vec<String>>) -> String {
        "parameter_store".to_string()
    }
}

#[mockable]
impl Client {
    pub fn new(p: Provider) -> Client {
        return Client{provider: p}
    }
}
