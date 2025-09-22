mod auto;
mod bun;
mod local;
mod npm;
mod pnpm;
mod yarn;

use std::{fmt::Debug, path::Path};

pub use auto::*;
pub use bun::*;
pub use local::*;
pub use npm::*;
pub use pnpm::*;
pub use yarn::*;

pub trait Backend: Debug {
    fn install(&self, dir: &Path) -> anyhow::Result<()>;
}
