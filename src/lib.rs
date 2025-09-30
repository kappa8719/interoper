#![feature(exit_status_error)]

use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::anyhow;

use crate::config::Config;

mod config;
mod node;
mod package_manager;

fn walk_dir(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buffer = vec![];
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let walk = walk_dir(path)?;
            buffer.extend(walk);
        } else {
            buffer.push(path);
        }
    }

    Ok(buffer)
}

/// A project which is built from given [Config]
#[derive(Debug)]
pub struct Project {
    /// Paths to installed dependencies by their key
    pub dependencies: HashMap<String, PathBuf>,
}

impl Project {
    pub fn build_templates(
        &self,
        source: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let source = source.as_ref();
        let destination = destination.as_ref();

        if !source.exists() || !source.is_dir() {
            return Err(anyhow!(
                "the source directory does not exist or not a directory"
            ));
        }
        std::fs::create_dir_all(destination)?;

        for path in walk_dir(source)? {
            // the path relative to source directory
            let path_relative = path.strip_prefix(source)?;
            let output = destination.join(path_relative);

            self.build_template(path, output)?;
        }

        Ok(())
    }

    pub fn build_template(
        &self,
        source: impl AsRef<Path>,
        destination: impl AsRef<Path>,
    ) -> anyhow::Result<()> {
        let source = source.as_ref();
        let destination = destination.as_ref();

        let mut compiled = std::fs::read_to_string(source)?;
        for (key, path) in self.dependencies.iter() {
            let pattern = format!("{{{{ interoper:{key} }}}}");
            let path = path.canonicalize()?;
            let Some(path) = path.as_path().to_str() else {
                return Err(anyhow!("failed to canonicalize path of dependency {key}"));
            };

            compiled = compiled.replace(pattern.as_str(), path);
        }

        std::fs::write(destination, compiled)?;

        Ok(())
    }
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
    // let dependencies = std::fs::read_dir(node_modules)?
    //     .filter_map(|v| v.ok())
    //     .filter_map(|v| {
    //         let path = v.path();
    //         let key = path.file_name()?.to_str()?;
    //         let key = key.to_string();
    //
    //         if !config.dependencies.contains_key(&key) || !v.metadata().ok()?.is_dir() {
    //             return None;
    //         }
    //
    //         Some((key.to_string(), path))
    //     })
    //     .collect::<HashMap<_, _>>();
    let dependencies = config
        .dependencies
        .iter()
        .filter_map(|(name, _)| {
            // the dependency is scoped package
            if name.starts_with("@") && name.contains("/") {
                let (scope, package) = name.split_once("/")?;
                let path = node_modules.join(scope).join(package);
                if path.exists() {
                    Some((name.clone(), path))
                } else {
                    None
                }
            } else {
                let path = node_modules.join(name);
                if path.exists() {
                    Some((name.clone(), path))
                } else {
                    None
                }
            }
        })
        .collect::<HashMap<_, _>>();

    Ok(Project { dependencies })
}
