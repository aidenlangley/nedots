use crate::{
    config::Config,
    ops::{
        op::{Operate, OperationError},
        AddChanges,
    },
    output::{
        logger::Logger,
        verbosity::{Verbose, Verbosity},
        TerminalLogger,
    },
};
use clap::{Parser, Subcommand};
use indicatif::ProgressBar;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[clap(about = "A tool for installing & managing ne/any-dots.")]
#[clap(version)]
pub(crate) struct Args {
    #[clap(short, long)]
    /// Enable the highest level of verbosity for assistance when debugging.
    debug: bool,

    #[clap(short, long, parse(from_occurrences))]
    /// Enables incremental increases in the level of verbosity, e.g. -vvv
    /// for most verbose.
    verbose: usize,

    #[clap(short, long)]
    /// Silences output.
    quiet: bool,

    #[clap(short, long)]
    /// Instead of sourcing the path from `nedots.json`, it can be passed
    /// as an argument.
    path: Option<String>,

    #[clap(subcommand)]
    /// Operation to perform.
    pub(crate) cmd: Command,
}

impl Args {
    /// Get the `Verbosity` to be used throughout the lifetime of the
    /// program. If `-d/--debug` was passed as an arg, enable the highest
    /// level of `Verbosity`, conversely, disable `Verbosity` when `-q/--quiet`
    /// is passed - otherwise, increase in increments that are determined by
    /// `-v/--verbose`.
    pub(crate) fn verbosity(&self) -> Option<Verbosity> {
        if self.debug {
            Some(Verbosity::Debug)
        } else if self.quiet {
            None
        } else {
            match self.verbose {
                0 => None,
                1 => Some(Verbosity::Low),
                2 => Some(Verbosity::Medium),
                _ => Some(Verbosity::High),
            }
        }
    }

    /// Get the `nedots` repository path if it was passed as an argument,
    /// & canonicalize it.
    pub(crate) fn path(&self) -> Result<Option<PathBuf>, std::io::Error> {
        if let Some(s) = self.path.as_ref() {
            return Ok(Some(Path::new(s).canonicalize()?));
        }

        Ok(None)
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Add changes to remote by commiting & pushing local changes to git
    /// repository. Conflicts are reported on, and it's expected that
    /// you handle them manually.
    AddChanges {
        #[clap(short, long)]
        /// Push changes to remote.
        push: bool,

        #[clap(short, long)]
        #[clap(default_value_t = String::from("origin"))]
        /// Push to this remote instead of origin.
        remote: String,

        #[clap(short, long)]
        /// Use this branch instead of default in .gitconfig.
        branch: Option<String>,
    },

    /// Update config files by pulling changes from remote & applying
    /// them locally.
    ///
    /// This application is not as smart as `git`, and won't try
    /// to be - `git pull` performs a `git fetch`, and makes a decision on
    /// the strategy it will use to integrate upstream changes, `fast forward`
    /// or merge via `rebase` or with a `merge commit`. We'll simply just
    /// `fast forward` since it is the safest option and the least dirty.
    ///
    /// If files have been modified more recently than the latest remote
    /// changes, this operation will stop. Overwrite any local changes with
    /// --force/-f.
    UpdateLocal {
        #[clap(short, long)]
        #[clap(default_value_t = String::from("origin"))]
        /// Pull from this remote instead of origin.
        remote: String,

        #[clap(short, long)]
        /// Use this branch instead of default in .gitconfig.
        branch: Option<String>,

        #[clap(short, long)]
        /// Only update the folders specified.
        only: Option<Vec<String>>,

        #[clap(long)]
        /// Overwrite local files.
        force: bool,
    },

    /// Installs packages from distributions' package manager, Flatpak, and
    /// performs other misc. install operations for supported distributions.
    /// Fedora will configure & install rpmfusion related repositories.
    InstallPackages {
        #[clap(short = 'y', long = "assumeyes")]
        /// Translates to `sudo dnf install -y`.
        assume_yes: bool,
    },
}

impl Operate for AddChanges<'_> {
    fn operate(&self) -> Result<usize, OperationError> {
        let bar =
            ProgressBar::new(self.copy_ops.len().try_into().unwrap()).with_prefix("Copying...");
        for op in &self.copy_ops {
            bar.println(format!(
                "{} -> {}",
                op.from.as_ref().unwrap().display(),
                op.to.as_ref().unwrap().display()
            ));
            op.copy()?;
            bar.inc(1);
        }

        Ok(0)
    }

    fn exit_code(&self) -> usize {
        if let Err(e) = self.operate() {
            match e {
                OperationError::Git(g) => match g {
                    crate::ops::git::GitError::NoPath => return 1,
                    crate::ops::git::GitError::NoRepo => return 1,
                    crate::ops::git::GitError::PushRejected => return 1,
                    crate::ops::git::GitError::Git2(_) => return 1,
                    crate::ops::git::GitError::Unknown => return 1,
                },
                OperationError::Copy(c) => match c {
                    crate::ops::fs::CopyError::NoFromPath => return 1,
                    crate::ops::fs::CopyError::NoToPath => return 1,
                    crate::ops::fs::CopyError::InvalidFileName { path: _ } => return 1,
                    crate::ops::fs::CopyError::IoError(_) => return 1,
                },
            }
        }

        0
    }
}

/// Prints `msg` and exits with `code`.
fn exit(msg: &str, code: usize) -> ! {
    crate::output::error(msg);
    std::process::exit(code.try_into().unwrap())
}

/// Parse args & run operations.
pub(super) fn run() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let logger = TerminalLogger::new().with_verbosity(args.verbosity());

    logger.log(&format!("Args: {:#?}", args))?;
    logger.log(&format!("Verbosity: {:#?}", logger.verbosity()))?;

    let config = match Config::new() {
        Ok(s) => {
            logger.log(format!("Settings: {:#?}", s).as_str())?;
            s
        }
        Err(e) => exit(format!("{}", e).as_str(), 1),
    };

    todo!()
}

#[cfg(test)]
mod tests {
    use super::Args;
    use crate::config::Config;
    use clap::IntoApp;

    #[test]
    fn verify() {
        Args::command().debug_assert()
    }

    #[test]
    fn read_settings() {
        if let Err(e) = Config::new() {
            assert!(false, "{}", e);
        }
    }
}
