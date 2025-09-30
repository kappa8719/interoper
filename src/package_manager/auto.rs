use std::path::Path;

use crate::package_manager::{Backend, bun::Bun, npm::Npm, pnpm::Pnpm, yarn::Yarn};

#[derive(Debug)]
pub struct Auto;

impl Backend for Auto {
    fn install(&self, dir: &Path) -> anyhow::Result<()> {
        let backends: [&dyn Backend; 4] = [&Bun, &Pnpm, &Yarn, &Npm];
        for backend in backends {
            if backend.install(dir).is_ok() {
                println!("install succeed");
                return Ok(());
            }
        }

        Err(anyhow::anyhow!(
            "none of backends succeed to install dependencies (not installed)"
        ))
    }
}
