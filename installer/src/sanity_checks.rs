use crate::settings::Settings;
use std::{
    fmt::Display,
    io::{stdout, Write},
    process::Command,
};

pub(crate) enum SanityCheckError {
    CheckFailure { prog: &'static str },
    GitMissing,
    FlatpakMissing,
    GitRepoFailure { path: String },
}

const MISSING_ERROR: &str = "is not installed, or is not accessible. The \
binary/executable must be visible to `sh`, `bash`, `fish`, etc. via $PATH.";

impl Display for SanityCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SanityCheckError::CheckFailure { prog } => write!(
                f,
                "Failed to sanity check `{}`, please open an issue on GitHub.",
                prog
            ),
            SanityCheckError::GitMissing => write!(f, "`git` {}", MISSING_ERROR),
            SanityCheckError::FlatpakMissing => write!(f, "`flatpak` {}", MISSING_ERROR),
            SanityCheckError::GitRepoFailure { path } => write!(
                f,
                "Encountered a problem when checking the integrity of this \
                `git` repository at {}, please verify the directory has been \
                initialised as a `git` repository and configured correctly.",
                path
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
        Err(_) => Err(SanityCheckError::CheckFailure { prog: "git" }),
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
        Err(_) => Err(SanityCheckError::CheckFailure { prog: "flatpak" }),
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
                    .write_all(&o.stdout)
                    .expect("Failed to write stdout!");
                stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr!");

                return Err(SanityCheckError::GitRepoFailure {
                    path: settings.path.to_string_lossy().to_string(),
                });
            }

            Ok(())
            // assert!(o.status.success())
        }
        Err(_) => Err(SanityCheckError::CheckFailure { prog: "git repo" }),
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
