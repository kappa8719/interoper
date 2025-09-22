use std::{path::Path, process::Command};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Pnpm;

impl Backend for Pnpm {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        Command::new("pnpm")
            .arg("install")
            .current_dir(dir)
            .output()?;

        Ok(())
    }
}
