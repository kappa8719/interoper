use std::path::Path;

use crate::config::Config;

mod config;
mod node;

pub fn build() {
    build_from_config_file("Interoper.toml");
}

pub fn build_from_config_file(path: impl AsRef<Path>) {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).unwrap();
    let config = toml::from_str(content.as_str()).unwrap();

    build_from_config(&config)
}

pub fn build_from_config(config: &Config) {
    println!("{config:#?}");
}

