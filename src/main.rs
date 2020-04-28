#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;
mod provider;
mod environment;
mod mytest;

fn main() {
    let filename = "config.yaml";
    let config = config::load_config(filename);
    provider::call_agent(config);
}
