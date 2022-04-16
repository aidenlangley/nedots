use crate::Command;
use clap::Parser;
use std::fmt::Display;

#[derive(Debug, Parser)]
#[clap(about = "Tool for installing & managing ne/any-dots.")]
#[clap(version)]
pub(super) struct Args {
    #[clap(short, long, parse(from_occurrences))]
    pub(super) debug: usize,

    #[clap(short, long, parse(from_occurrences))]
    pub(super) verbose: usize,

    #[clap(short, long)]
    #[clap(help = "Path of nedots data directory")]
    pub(super) path: Option<String>,

    #[clap(subcommand)]
    pub(super) cmd: Command,
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
                "Medium (leads to increased verbosity of child processes, as \
                well as the verbosity provided by Low.)"
            ),
            Verbosity::High => write!(
                f,
                "High (leads to increased verbosity of this application in \
                addition to the verbosity provided by Medium.)"
            ),
        }
    }
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
