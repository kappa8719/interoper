use std::{path::Path, process::Command};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Npm;

impl Backend for Npm {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        Command::new("npm")
            .arg("install")
            .current_dir(dir)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}
