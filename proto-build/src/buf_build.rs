//! Support for [buf](https://buf.build) build tools.

use serde::Deserialize;
use tracing::{error, info};

use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;
use std::process::{self, Command, ExitStatus};
use std::sync::atomic::{self, AtomicBool};

// Suppress log messages
static QUIET: AtomicBool = AtomicBool::new(false);

/// Errors that can occur when working with buf files.
#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
    #[error("unsupported lock file version {}", .encountered)]
    UnsupportedVersion { encountered: String },
}

/// Serialization object for the `buf.lock` file.
#[derive(Debug, Deserialize)]
struct LockFile {
    pub version: String,
    pub deps: Option<Vec<LockedDep>>,
}

/// Serialization object for an entry under the `deps` key in a `buf.lock` file.
#[derive(Debug, Deserialize)]
pub struct LockedDep {
    pub remote: String,
    pub owner: String,
    pub repository: String,
    pub commit: String,
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => {
                panic!(
                    "error running '{:?}': command not found. Is it installed?",
                    cmd.as_ref()
                )
            }
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(proto_path: impl AsRef<Path>) {
    run_cmd(
        "buf",
        ["mod", "update", &proto_path.as_ref().display().to_string()],
    );
}

pub fn read_locked_deps(
    lockfile_path: impl AsRef<Path>,
    proto_path: impl AsRef<Path>,
) -> Result<Vec<LockedDep>, FileError> {
    run_buf(proto_path);
    let file = File::open(lockfile_path)?;
    let lock_config: LockFile = serde_yaml::from_reader(BufReader::new(file))?;
    if lock_config.version != "v1" {
        return Err(FileError::UnsupportedVersion {
            encountered: lock_config.version,
        });
    }
    let deps = lock_config.deps.unwrap_or_default();
    Ok(deps)
}

/// Errors that can occur from invocation of the buf CLI tool.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("failed to execute buf: {}", .0)]
    Spawn(#[from] io::Error),
    #[error("buf exited with {}", .0)]
    Failure(ExitStatus),
}

pub fn export_dep_module(dep: &LockedDep, out_dir: &Path) -> Result<(), ToolError> {
    let module_ref = format!(
        "{}/{}/{}:{}",
        dep.remote, dep.owner, dep.repository, dep.commit
    );
    info!("Exporting `{}` into `{}`", module_ref, out_dir.display());
    let status = Command::new("buf")
        .arg("export")
        .arg(module_ref)
        .arg("-o")
        .arg(out_dir)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(ToolError::Failure(status))
    }
}
