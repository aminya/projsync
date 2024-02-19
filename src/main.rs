// This file is a part of projsync https://github.com/aminya/projsync licensed under Apache-2.0
// Copyright 2021-2100 Amin Yahyaabadi

#![allow(
  clippy::needless_return,
  clippy::too_many_arguments,
  clippy::type_complexity
)]

use std::env::current_dir;

use anyhow::{anyhow, Error};
use argh::FromArgs;
use log::LevelFilter;
use untildify::untildify;

mod gitignore;
mod rsync;
use rsync::*;

/**
  Sync projects to different remote machines over SSH or WSL
  https://github.com/aminya/projsync

  Example:
    - Sync the current directory with ~/folder_name

      sshsync --remote localwsl
      sshsync --remote ssh_alias
      sshsync --remote username@remoteIP_or_name --port 22

    - Sync the given source to the given directory

      sshsync --source ./ --target ~/folder_name --remote localwsl

    - Exclude some files or folders in addition to gitignored files from syncing

      sshsync --exclude some_file_or_folder --remote localwsl

*/
#[derive(FromArgs, PartialEq, Debug)]
pub struct Opts {
  #[argh(option)]
  /** the source path as a string. Defaults to the current directory */
  pub source: Option<String>,

  #[argh(option)]
  /** the target directory as a string. Defaults to ~ */
  pub target: Option<String>,

  #[argh(option)]
  /** the target remote. It should be inform of (1) an alias from `~/.ssh/config` (2) `username@remoteIP_or_name` (3) `localwsl` for syncing with WSL */
  pub remote: String,

  #[argh(option, default = "22")]
  /** the ssh port to use for syncing the folder with. Defaults to 22 */
  pub port: usize,

  #[argh(option)]
  /** an array of paths to ignore when syncing */
  exclude: Vec<String>,
}

fn main() -> Result<(), Error> {
  env_logger::builder()
    .filter_level(LevelFilter::Info)
    .parse_default_env()
    .format_target(true)
    .format_timestamp(None)
    .init();

  let opts: Opts = argh::from_env();

  let source = match opts.source {
    Some(source) => untildify(&source),
    None => current_dir()?
      .into_os_string()
      .into_string()
      .map_err(|err| {
        anyhow!(
          "Failed to convert path to string: {}",
          err.to_string_lossy()
        )
      })?,
  };

  let target = &opts
        .target
        // if not specified, the folder will be created at ~ with the same name
        .unwrap_or_else(|| "~".to_owned());

  return rsync(&source, &target, opts.remote, opts.port, opts.exclude);
}
