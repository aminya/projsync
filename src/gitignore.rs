// This file is a part of projsync https://github.com/aminya/projsync licensed under Apache-2.0
// Copyright 2021-2100 Amin Yahyaabadi

use std::process::Command;

use anyhow::Error;
use camino::{Utf8Path, Utf8PathBuf};

pub fn gitignore_files(root: &Utf8Path) -> Result<Vec<Utf8PathBuf>, Error> {
  let excluded_files = Command::new("git")
    .args([
      "-C",
      root.as_str(),
      "ls-files",
      "--exclude-standard",
      "-oi",
      "--directory",
    ])
    .output()
    .map_err(|err| anyhow::anyhow!("Failed to run git ls-files: {}", err))
    .map(|output| {
      String::from_utf8(output.stdout)
        .unwrap()
        .split('\n')
        .filter_map(|file| {
          if !file.is_empty() && root.join(file).exists() {
            return Some(Utf8PathBuf::from(file));
          }
          return None;
        })
        .collect()
    });

  return excluded_files;
}
