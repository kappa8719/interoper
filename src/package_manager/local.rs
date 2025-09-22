use std::{path::Path, process::Command};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Local {
    pub executable: String,
}

impl Backend for Local {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        Command::new(self.executable.as_str())
            .arg("install")
            .current_dir(dir)
            .status()?
            .exit_ok()?;

        Ok(())
    }
}
