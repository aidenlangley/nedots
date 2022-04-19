use crate::{install::Distro, logger::Verbosity};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(about = "Tool for installing & managing ne/any-dots.")]
#[clap(version)]
pub(super) struct Args {
    #[clap(short, long)]
    debug: bool,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(short, long)]
    #[clap(help = "Path of nedots data directory")]
    pub(super) path: Option<String>,

    #[clap(subcommand)]
    pub(super) cmd: Command,
}

impl Args {
    pub fn verbosity(&self) -> Option<Verbosity> {
        if self.debug {
            Some(Verbosity::Debug)
        } else {
            match self.verbose {
                0 => None,
                1 => Some(Verbosity::Low),
                2 => Some(Verbosity::Medium),
                _ => Some(Verbosity::High),
            }
        }
    }
}

#[derive(Debug, Subcommand)]
pub(super) enum Command {
    #[clap(about = "Add changes to remote by commiting & pushing local changes \
        to git repository. Conflicts are reported on, and it's expected that \
        you handle them manually.")]
    Add {
        #[clap(short, long)]
        #[clap(help = "Push changes to remote.")]
        push: bool,
    },

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
