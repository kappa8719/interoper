use std::{
    path::Path,
    process::{Command, Stdio},
};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Npm;

impl Backend for Npm {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        let output = Command::new("npm")
            .arg("install")
            .current_dir(dir)
            .output()?;
        println!("{output:?}");

        Ok(())
    }
}
