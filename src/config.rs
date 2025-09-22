use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::package_manager;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub package_manager: PackageManagerType,
    pub dependencies: HashMap<String, DependencySpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PackageManagerType {
    /// Automatically use package manager on system
    #[default]
    Auto,
    Npm,
    Pnpm,
    Yarn,
    Bun,
    #[serde(untagged)]
    LocalExecutable(String),
}

impl PackageManagerType {
    pub fn as_backend(&self) -> Box<dyn package_manager::Backend> {
        match self {
            PackageManagerType::Auto => Box::new(package_manager::Auto),
            PackageManagerType::Npm => Box::new(package_manager::Npm),
            PackageManagerType::Pnpm => Box::new(package_manager::Pnpm),
            PackageManagerType::Yarn => Box::new(package_manager::Yarn),
            PackageManagerType::Bun => Box::new(package_manager::Bun),
            PackageManagerType::LocalExecutable(executable) => Box::new(package_manager::Local {
                executable: executable.clone(),
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencySpec {
    // A package from registry with version specified in string
    RegistryVersion(String),
    // A package from registry with name optionally specified
    Registry {
        #[serde(default = "DependencySpec::default_registry")]
        registry: String,
        name: Option<String>,
        version: String,
    },
    // A package from tarball located at the url
    Url {
        url: String,
    },
    // A package from git repository
    Git {
        git: String,
        #[serde(flatten)]
        version: Option<GitDependencyVersion>,
    },
    // A package from github repository
    Github {
        github: String,
        #[serde(flatten)]
        version: Option<GitDependencyVersion>,
    },
    // A package from directory in the local machine
    LocalPath {
        path: String,
    },
}

impl DependencySpec {
    fn default_registry() -> String {
        String::from("npm")
    }

    pub fn as_package_json_dependency_version(&self, key: &str) -> String {
        match self {
            DependencySpec::RegistryVersion(version) => version.clone(),
            DependencySpec::Registry {
                registry,
                name,
                version,
            } => {
                if let Some(name) = name {
                    format!("{key}@{registry}:{name}@{version}")
                } else {
                    format!("{key}@{registry}:{key}@{version}")
                }
            }
            DependencySpec::Url { url } => url.clone(),
            DependencySpec::Git { git, version } => {
                if let Some(version) = version {
                    format!("{git}#{version}")
                } else {
                    git.clone()
                }
            }
            DependencySpec::Github { github, version } => {
                if let Some(version) = version {
                    format!("{github}#{version}")
                } else {
                    github.clone()
                }
            }
            DependencySpec::LocalPath { path } => path.clone(),
        }
    }
}

/// A version of git dependency(Git, Github)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GitDependencyVersion {
    Tag {
        tag: String,
    },
    Reference {
        #[serde(rename = "ref")]
        reference: String,
    },
    Branch {
        branch: String,
    },
}

impl Display for GitDependencyVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitDependencyVersion::Tag { tag } => f.write_str(tag.as_str()),
            GitDependencyVersion::Reference { reference } => f.write_str(reference.as_str()),
            GitDependencyVersion::Branch { branch } => f.write_str(branch.as_str()),
        }
    }
}
