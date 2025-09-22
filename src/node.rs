use std::collections::HashMap;

use serde_json::json;

use crate::config::Config;

pub fn build_package_json(config: &Config) -> Result<String, serde_json::Error> {
    let dependencies = config
        .dependencies
        .iter()
        .map(|v| {
            (
                v.0.clone(),
                v.1.as_package_json_dependency_version(v.0.as_str()),
            )
        })
        .collect::<HashMap<_, _>>();

    let json = json!({
        "dependencies": dependencies
    });

    serde_json::to_string_pretty(&json)
}
