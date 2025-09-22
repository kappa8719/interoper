use std::{path::Path, process::Command};

use anyhow::Ok;

use crate::package_manager::Backend;

#[derive(Debug)]
pub struct Bun;

impl Backend for Bun {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        let output = Command::new("bun")
            .arg("install")
            .current_dir(dir)
            .output()?;
        println!("{output:?}");

        Ok(())
    }
}
