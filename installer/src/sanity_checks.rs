//! Sanity checks are commands that are run to verify whether or not `nedots`
//! will be able to perform its' various operations prior to running them. The
//! output is generally going to be more user friendly than errors returned by
//! problems encountered during an operation, since these errors are few & far
//! between, and they can be concisely reported on & quickly resolved by the
//! user.
//!
//! Functions within this module are only shared with `super`, since other
//! operations do not need to concern themselves with `sanity_checks`.

use crate::settings::Settings;
use std::{fmt::Display, io::Write, process::Command};

/// Error types returned when performing sanity checks.
pub enum SanityCheckError {
    CheckFailure { check: &'static str },
    GitMissing,
    FlatpakMissing,
    GitRepoFailure,
}

/// When `command -v {prog}` fails, this message is returned by the `Display`
/// trait implementation.
const MISSING_ERROR: &str = "is not installed, or is not accessible. The \
binary/executable must be visible to `sh`, `bash`, `fish`, etc. via $PATH.";

impl Display for SanityCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SanityCheckError::CheckFailure { check } => write!(
                f,
                "Failed to sanity check `{}`, please open an issue on GitHub.",
                check
            ),
            SanityCheckError::GitMissing => write!(f, "`git` {}", MISSING_ERROR),
            SanityCheckError::FlatpakMissing => write!(f, "`flatpak` {}", MISSING_ERROR),
            SanityCheckError::GitRepoFailure => write!(
                f,
                "Encountered a problem when checking the integrity of `git` \
                repository, please verify the directory has been initialised \
                as a `git` repository and configured correctly.",
            ),
        }
    }
}

/// Wrapper function around `command -v git`.
///
/// ### Errors
/// Returns `SanityCheckError::GitMissing` when `git` is missing, this is the
/// expected result when failing. If an unexpected error occurs,
/// `SanityCheckError::CheckFailure` is returned.
pub(super) fn check_git() -> Result<(), SanityCheckError> {
    let output = Command::new("command").args(["-v", "git"]).output();
    match output {
        Ok(o) => {
            if !o.status.success() {
                return Err(SanityCheckError::GitMissing);
            }

            Ok(())
        }
        Err(_) => Err(SanityCheckError::CheckFailure { check: "git" }),
    }
}

/// Wrapper function around `command -v flatpak`.
///
/// ### Errors
/// Returns `SanityCheckError::FlatpakMissing` when `flatpak` is missing, this
/// is the expected result when failing. If an unexpected error occurs,
/// `SanityCheckError::CheckFailure` is returned.
pub(super) fn check_flatpak() -> Result<(), SanityCheckError> {
    let output = Command::new("command").args(["-v", "flatpak"]).output();
    match output {
        Ok(o) => {
            if !o.status.success() {
                return Err(SanityCheckError::FlatpakMissing);
            }
            Ok(())
        }
        Err(_) => Err(SanityCheckError::CheckFailure { check: "flatpak" }),
    }
}

/// Wrapper function around `git -C {path} status`.
///
/// ### Errors
/// Returns `SanityCheckError::GitRepoFailure` when the directory is not a `git`
/// repository - it's likely not been initialised. If an unexpected error
/// occurs, `SanityCheckError::CheckFailure` is returned.
pub(super) fn check_repo() -> Result<(), SanityCheckError> {
    let settings: Settings = match Settings::read() {
        Ok(s) => s,
        Err(e) => panic!("{}", e),
    };

    let output = Command::new("git")
        .args([
            "-C",
            settings.path.to_string_lossy().to_string().as_str(),
            "status",
        ])
        .output();

    match output {
        Ok(o) => {
            if !o.status.success() {
                std::io::stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git status`!");
                return Err(SanityCheckError::GitRepoFailure);
            }

            std::io::stdout()
                .write_all(&o.stdout)
                .expect("Failed to write stdout from `git status`!");
            Ok(())
        }
        Err(_) => Err(SanityCheckError::CheckFailure { check: "git repo" }),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_git() {
        if let Err(e) = super::check_git() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn check_flatpak() {
        if let Err(e) = super::check_flatpak() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn check_repo() {
        if let Err(e) = super::check_repo() {
            assert!(false, "{}", e)
        }
    }
}
