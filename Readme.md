# projsync

Sync projects to different remote machines over SSH or WSL

`projsync` can be used to sync a project to a different remote machine over SSH or a local WSL machine. This makes it easy to test the project on different machines while developing on a different machine.

It uses `rsync` to sync the files and automatically excludes the files in the `.gitignore` file.

# Installation

```shell
cargo install --git https://github.com/aminya/projsync.git --rev main
```

# Examples

- Sync the current directory with `~/folder_name` on the remote machine using a SSH alias

  ```shell
  projsync --remote mac2014
  ```

- Sync the current directory with `~/folder_name` on the remote machine using a username and IP address

  ```shell
  projsync --remote username@remoteIP_or_name --port 22
  ```

- Sync the current directory with `~/folder_name` on the local WSL machine (`localwsl`)

  ```shell
  projsync --remote localwsl
  ```

- Sync the given source directory to the given directory

  ```shell
  projsync --source ./ --target ~/folder_name --remote localwsl
  ```

- Exclude some files or folders in addition to gitignored files from syncing

  ```shell
  projsync --exclude some_file_or_folder --remote localwsl
  ```


# Command Line Options

```shell
Usage: projsync [--source <source>] [--target <target>] --remote <remote> [--port <port>] [--exclude <exclude...>]

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

Options:
  --source          the source path as a string. Defaults to the current
                    directory
  --target          the target directory as a string. Defaults to ~
  --remote          the target remote. It should be inform of (1) an alias from
                    `~/.ssh/config` (2) `username@remoteIP_or_name` (3)
                    `localwsl` for syncing with WSL
  --port            the ssh port to use for syncing the folder with. Defaults to
                    22
  --exclude         an array of paths to ignore when syncing
  --help, help      display usage information
```
