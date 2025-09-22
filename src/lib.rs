#![feature(exit_status_error)]

use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::config::Config;

mod config;
mod node;
mod package_manager;

/// A project which is built from given [Config]
#[derive(Debug)]
pub struct Project {
    /// Paths to installed dependencies by their key
    pub dependencies: HashMap<String, PathBuf>,
}

pub fn build() -> anyhow::Result<Project> {
    build_from_config_file("Interoper.toml")
}

pub fn build_from_config_file(path: impl AsRef<Path>) -> anyhow::Result<Project> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(content.as_str())?;

    build_from_config(&config)
}

pub fn build_from_config(config: &Config) -> anyhow::Result<Project> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = PathBuf::from_str(out_dir.as_str())?;
    let workdir = out_dir.join("interoper");
    if !workdir.exists() {
        std::fs::create_dir_all(workdir.as_path())?;
    }

    let package_json = node::build_package_json(config)?;
    std::fs::write(workdir.join("package.json"), package_json)?;

    let package_manager = config.package_manager.as_backend();
    package_manager.install(workdir.as_path())?;

    let node_modules = workdir.join("node_modules");
    let dependencies = std::fs::read_dir(node_modules)?
        .filter_map(|v| v.ok())
        .filter_map(|v| {
            let path = v.path();
            let key = path.file_name()?.to_str()?;
            let key = key.to_string();

            if !config.dependencies.contains_key(&key) || !v.metadata().ok()?.is_dir() {
                return None;
            }

            Some((key.to_string(), path))
        })
        .collect::<HashMap<_, _>>();

    Ok(Project { dependencies })
}
