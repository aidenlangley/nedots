use clap::{Parser, Subcommand};
use std::fmt::Display;

use crate::install::Distro;

#[derive(Debug, Parser)]
#[clap(about = "Tool for installing & managing ne/any-dots.")]
#[clap(version)]
pub(super) struct Args {
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(short, long)]
    #[clap(help = "Path of nedots data directory")]
    pub(super) path: Option<String>,

    #[clap(subcommand)]
    pub(super) cmd: Command,
}

impl Args {
    pub fn debugging(&self) -> bool {
        self.debug > 0
    }

    pub fn get_verbosity(&self) -> Option<Verbosity> {
        match self.verbose {
            0 => None,
            1 => Some(Verbosity::Low),
            2 => Some(Verbosity::Medium),
            _ => Some(Verbosity::High),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Determines the verbosity of the applications output to the terminal.
pub enum Verbosity {
    /// Low leads to the removal of quiet flags being passed to child processes.
    Low = 1,
    /// Medium leads to increased verbosity of child processes.
    Medium = 2,
    /// High leads to increased verbosity of this application in addition to the
    /// above.
    High = 3,
}

impl Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::Low => write!(
                f,
                "Low (leads to the removal of quiet flags being passed to \
                    child processes.)"
            ),
            Verbosity::Medium => write!(
                f,
                "Medium (leads to increased verbosity of child processes where
                    applicable, as well as the verbosity provided by Low.)"
            ),
            Verbosity::High => write!(
                f,
                "High (leads to increased verbosity of this application in \
                    addition to the verbosity provided by Medium.)"
            ),
        }
    }
}

#[derive(Debug, Subcommand)]
pub(super) enum Command {
    #[clap(about = "Add changes to remote by commiting & pushing local changes \
        to git repository. Conflicts are reported on, and it's expected that \
        you handle them manually.")]
    Add,

    #[clap(about = "Update config files by pulling changes from remote & \
        applying them locally. If files have been modified more recently than \
        the latest remote changes, this operation will stop. Overwrite any \
        local changes with --force/-f.")]
    Update {
        #[clap(short, long)]
        #[clap(help = "Only update the folders specified.")]
        only: Option<Vec<String>>,

        #[clap(long, parse(try_from_str))]
        #[clap(help = "Overwrite local files.")]
        force: bool,
    },

    #[clap(about = "Install packages, configs & perform misc. install \
        operations.")]
    Install {
        #[clap(short, long)]
        distro: Option<Distro>,

        #[clap(short = 'y', long = "assumeyes")]
        assume_yes: bool,
        // #[clap(subcommand)]
        // cmd: install::InstallCommand,
    },
}
