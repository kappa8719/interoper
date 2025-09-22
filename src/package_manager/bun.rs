use std::{path::Path, process::Command};

use anyhow::{Ok, anyhow};

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Bun;

impl Backend for Bun {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        Command::new("bun")
            .arg("install")
            .current_dir(dir)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}
