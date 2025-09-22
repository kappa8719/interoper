use std::{path::Path, process::Command};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Yarn;

impl Backend for Yarn {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        Command::new("yarn").current_dir(dir).output()?;

        Ok(())
    }
}
