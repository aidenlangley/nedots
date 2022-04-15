use super::GitError;
use chrono::Local;
use std::{
    io::Write,
    path::Path,
    process::{Command, Output},
};

pub(super) fn add(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
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

pub(super) fn commit(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
            "commit",
            "-m",
        ])
        .arg(format!("'Latest ({})'", Local::now()))
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

pub(super) fn push(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args(["-C", dest.to_string_lossy().to_string().as_str(), "push"])
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

pub(super) fn _stash_push(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
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
                return Err(GitError::_StashPushFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git stash push`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

pub(super) fn _stash_pop(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
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
                return Err(GitError::_StashPopFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git stash pop`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}

pub(super) fn _reset_hard(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
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
                    .expect("Failed to write stderr from `git revert`!");
                return Err(GitError::_RevertFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git revert`!");
            Ok(o)
        }
        Err(_) => Err(GitError::Unknown),
    }
}
