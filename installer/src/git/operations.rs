//! Private module `git::operations`, functions within are exposed by public
//! functions in module `git`.

use super::GitError;
use chrono::Local;
use std::{
    io::Write,
    path::Path,
    process::{Command, Output},
};

/// Wrapper function around the Command: `git add .`
///
/// ### Errors
/// Unsure what errors might be thrown by this operation, but throws
/// `GitError::AddFailure` or `GitError::Unknown` accordingly.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn add(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            path.to_string_lossy().to_string().as_str(),
            "add",
            ".",
        ])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git add`!");
                return Err(GitError::AddFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git add`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git commit -m "Latest ($datetime)"`.
///
/// ### Errors
/// Change conflicts throw `GitError::Conflict`,  or `GitError::Unknown` if the
/// error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn commit(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            path.to_string_lossy().to_string().as_str(),
            "commit",
            "-m",
        ])
        .arg(format!("Latest ({})", Local::now()))
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git commit`!");
                return Err(GitError::Conflict);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git commit`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git push`.
///
/// ### Errors
/// Authentication errors throw `GitError::AuthFailure`,  or `GitError::Unknown`
/// if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn push(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args(["-C", path.to_string_lossy().to_string().as_str(), "push"])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git push`!");
                return Err(GitError::AuthFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git push`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git stash push`. Primarily used when
/// performing operations on this `git` repo as part of the functionality
/// provided by the binary, we only want to add changes from external files,
/// in other words, not the installer code, just the dot file changes.
///
/// ### Errors
/// Authentication errors throw `GitError::StashPushFailure`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn stash_push(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            path.to_string_lossy().to_string().as_str(),
            "stash",
            "push",
        ])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git stash push`!");
                return Err(GitError::StashPushFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git stash push`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git stash pop`. Primarily used when
/// performing operations on this `git` repo as part of the functionality
/// provided by the binary, we only want to add changes from external files,
/// in other words, not the installer code, just the dot file changes.
///
/// ### Errors
/// Authentication errors throw `GitError::StashPopFailure`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn stash_pop(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            path.to_string_lossy().to_string().as_str(),
            "stash",
            "pop",
        ])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git stash pop`!");
                return Err(GitError::StashPopFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git stash pop`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git pull`.
///
/// ### Errors
/// Authentication errors throw `GitError::PullError`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn pull(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args(["-C", path.to_string_lossy().to_string().as_str(), "pull"])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git pull`!");
                return Err(GitError::PullFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git pull`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

/// Wrapper function around the Command: `git rest --hard HEAD^`. Used in
/// conjunction with `commit` to roll back our commit after testing.
///
/// This is a dangerous command and can really fuck your `git` tree. We really
/// do want to nuke the latest commit, since this is a test, so we're happy to
/// use `--hard`, and `HEAD^` refers to the last commit.
///
/// ### Errors
/// Failures throw a `GitError::_RevertFailure`, or `GitError::Unknown` if the
/// error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub(super) fn _reset_hard(path: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            path.to_string_lossy().to_string().as_str(),
            "reset",
            "--hard",
            "HEAD^",
        ])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git reset`!");
                return Err(GitError::_ResetFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git reset`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}
