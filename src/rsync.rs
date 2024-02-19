// This file is a part of projsync https://github.com/aminya/projsync licensed under Apache-2.0
// Copyright 2021-2100 Amin Yahyaabadi

use std::process::Command;

use anyhow::{anyhow, Error};
use camino::Utf8Path;
use log::{info, trace, warn};
use wslpath::windows_to_wsl;

use crate::gitignore::gitignore_files;

/// Converts to wsl Path on Windows and keep it as is on Posix
pub fn to_posix_path(path: &str) -> Result<String, Error> {
  if cfg!(windows) && cfg!(target_pointer_width = "64") {
    return windows_to_wsl(path).map_err(|err| anyhow!("Failed to convert path to WSL: {}", err));
  }
  return Ok(path.to_string());
}

/** Sync the given source path on Windows with the given target on WSL
   Params:
       - `source`: the source path as a string. Defaults to current working directory
       - `target`: the target directory as a string. Defaults to `"~/"`
       - `extra_exclude`: an array of paths to ignore when syncing
    Optional Params:
       - `remote`: the target remote. It should be inform of:
            - an alias from `~/.ssh/config`
            - `username@remoteIP_or_name`
            - `localwsl` for syncing with WSL
       - `port`: the port to use for connecting to the remote.
*/
pub fn rsync(
  source: &str,
  target: &str,
  remote: String,
  port: usize,
  exclude: Vec<String>,
) -> Result<(), Error> {
  let mut cmd;

  let source_posix = to_posix_path(source)?;

  match remote.as_str() {
    // localwsl syncinc on windows 64 bit
    "localwsl" if cfg!(windows) && cfg!(target_pointer_width = "64") => {
      cmd = Command::new("wsl");
      cmd.arg("rsync");

      if source_posix != target {
        cmd.args([&source_posix, target]);
      } else {
        return Err(anyhow!(
          "Cannot sync a directory with itself. target and source were: {}",
          target
        ));
      }
    }
    // syncing with a remote machine
    _ => {
      cmd = Command::new("rsync");

      cmd.args([
        "-e",
        &format!("ssh -p {}", port),
        &source_posix,
        &format!("{}:{}", remote, target),
      ]);
    }
  }

  // common rsync options
  cmd.args([
    "--archive",
    "--delete",
    "--human-readable",
    "--update",
    "--progress",
  ]);

  // ignore the excluded files
  let git_excludes = gitignore_files(&Utf8Path::new(source)).unwrap_or_else(|err| {
    warn!(
      "Failed to get gitignored files: {}\nConsidering no git ignored files.",
      err
    );
    Vec::new()
  });

  cmd.args(
    exclude
      .into_iter()
      .chain(git_excludes.into_iter().map(|ex| ex.into_string()))
      .map(|ex| format!("--exclude={}", ex)),
  );

  info!("Syncing {source_posix} to {target} at {remote}:{port}");
  trace!("Running `{cmd:?}`");

  let mut child = cmd.spawn()?;
  let status = child.wait()?;
  if !status.success() {
    return Err(anyhow!(
      "Failed to sync the project via rsync. Exit status: {:?}",
      status.code()
    ));
  }

  return Ok(());
}
