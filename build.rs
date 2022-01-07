use anyhow::Result;
use vergen::{Config, SemverKind, vergen};

fn main() -> Result<()> {
  // Generate the default 'cargo:' instruction output
  let mut config = Config::default();
  *config.git_mut().semver_kind_mut() = SemverKind::Lightweight;
  vergen(config)
}
