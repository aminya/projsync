// This file is a part of projsync https://github.com/aminya/projsync licensed under Apache-2.0
// Copyright 2021-2100 Amin Yahyaabadi

use std::process::Command;

use anyhow::Error;
use camino::{Utf8Path, Utf8PathBuf};
use log::trace;

pub fn gitignore_files(root: &Utf8Path) -> Result<Vec<Utf8PathBuf>, Error> {
  let mut cmd = Command::new("git");
  cmd.args([
    "-C",
    root.as_str(),
    "ls-files",
    "--exclude-standard",
    "-oi",
    "--directory",
  ]);

  trace!("Running `{cmd:?}` at {root}");

  let excluded_files = cmd
    .output()
    .map_err(|err| anyhow::anyhow!("Failed to run git ls-files at {root}: {err}"))
    .map(|output| {
      if !output.status.success() {
        return Err(anyhow::anyhow!(
          "Failed to run git ls-files at {root}: {}",
          String::from_utf8(output.stderr).unwrap_or_else(|_| "Unknown error".to_string())
        ));
      }

      let paths = String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .filter_map(|file| {
          if !file.is_empty() && root.join(file).exists() {
            return Some(Utf8PathBuf::from(file));
          }
          return None;
        })
        .collect();

      return Ok(paths);
    })?;

  return excluded_files;
}
