use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "kebab-case")]
pub struct Config {
    pub package_manager: PackageMangaerType,
    pub packages: HashMap<String, PackageSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum PackageMangaerType {
    #[default]
    Auto,
    Npm,
    Pnpm,
    Yarn,
    Bun,
    #[serde(untagged)]
    LocalExecutable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageSpec {
    // A package from registry with version specified in string
    RegistryVersion(String),
    // A package from registry with name optionally specified
    Registry {
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
        version: Option<PackageGitVersion>,
    },
    // A package from github repository
    Github {
        github: String,
        #[serde(flatten)]
        version: Option<PackageGitVersion>,
    },
    // A package from directory in the local machine
    LocalPath {
        path: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageGitVersion {
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
