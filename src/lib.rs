use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::Ok;

use crate::config::Config;

mod config;
mod node;
mod package_manager;

pub fn build() -> anyhow::Result<()> {
    build_from_config_file("Interoper.toml")
}

pub fn build_from_config_file(path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)?;
    let config = toml::from_str(content.as_str())?;

    build_from_config(&config)
}

pub fn build_from_config(config: &Config) -> anyhow::Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    println!("outdir: {out_dir}");
    let out_dir = PathBuf::from_str(out_dir.as_str())?;
    let workdir = out_dir.join("interoper");
    if !workdir.exists() {
        std::fs::create_dir_all(workdir.as_path())?;
    }

    let package_json = node::build_package_json(config)?;
    std::fs::write(workdir.join("package.json"), package_json)?;

    let package_manager = config.package_manager.as_backend();
    package_manager.install(workdir.as_path())?;

    Ok(())
}
