use crate::settings::Settings;
use std::{
    fmt::Display,
    io::{stdout, Write},
    process::Command,
};

pub(crate) enum SanityCheckError {
    CheckFailure { check: &'static str },
    GitMissing,
    FlatpakMissing,
    GitRepoFailure,
}

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

pub(super) fn check_repo() -> Result<(), SanityCheckError> {
    let settings: Settings = crate::read_settings();
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
                stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr from `git status`!");
                return Err(SanityCheckError::GitRepoFailure);
            }

            stdout()
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
            panic!("{}", e)
        }
    }

    #[test]
    fn check_flatpak() {
        if let Err(e) = super::check_flatpak() {
            panic!("{}", e)
        }
    }

    #[test]
    fn check_repo() {
        if let Err(e) = super::check_repo() {
            panic!("{}", e)
        }
    }
}
