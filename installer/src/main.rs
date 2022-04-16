mod git;
mod install;
mod sanity_checks;
mod settings;
mod utils;

use clap::{Parser, Subcommand};
use install::{Distro, InstallCommand};
use nix::unistd::Uid;
use settings::Settings;
use std::path::PathBuf;
use utils::CopyOperation;

#[derive(Debug, Parser)]
#[clap(about = "Tool for installing & managing ne/any-dots.")]
#[clap(version, arg_required_else_help(true))]
struct Args {
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(short, long)]
    #[clap(help = "Path of nedots data directory")]
    path: Option<String>,

    #[clap(subcommand)]
    cmd: Option<Command>,
}

enum Verbosity {
    Low,
    Medium,
    High,
}

impl Args {
    pub fn debugging(&self) -> bool {
        self.debug > 0
    }

    pub fn verbosity(&self) -> Verbosity {
        match self.verbose {
            1 => Verbosity::Low,
            2 => Verbosity::Medium,
            _ => Verbosity::High,
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
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

    #[clap(about = "Fancy way to git fetch to check for remote changes.")]
    Check,

    #[clap(about = "Install packages, configs & perform misc. install \
    operations.")]
    Install {
        #[clap(short, long)]
        distro: Option<Distro>,

        #[clap(short = 'y', long = "assumeyes")]
        assume_yes: bool,

        #[clap(subcommand)]
        cmd: install::InstallCommand,
    },
}

fn read_settings(args: Args) -> Settings {
    match Settings::read() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

fn add(args: Args, settings: Settings) {
    if let Err(e) = git::stash(&settings.path) {
        println!("{}", e);
        std::process::exit(1)
    }

    if Uid::effective().is_root() {
        let mut copy_op = CopyOperation::new(settings.root);
        match copy_op.copy_to(&settings.path) {
            Ok(_) => println!("{:#?}", copy_op.results),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1)
            }
        }
    }

    let mut copy_op = CopyOperation::new(settings.user);
    match copy_op.copy_to(&settings.path) {
        Ok(_) => println!("{:#?}", copy_op.results),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }

    if let Err(e) = sanity_checks::check_repo() {
        println!("{}", e);
        std::process::exit(1)
    }

    for func in [
        git::add(&settings.path),
        git::commit(&settings.path),
        git::push(&settings.path),
        git::restore(&settings.path),
    ] {
        if let Err(e) = func {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

fn install(args: Args, distro: Option<Distro>, assume_yes: bool, cmd: &InstallCommand) {
    match cmd {
        InstallCommand::Core => todo!(),
        InstallCommand::X11 => todo!(),
        InstallCommand::Wayland => todo!(),
        InstallCommand::Flatpaks => {
            if let Err(e) = sanity_checks::check_flatpak() {
                println!("{}", e);
                std::process::exit(1)
            }

            todo!()
        }
        InstallCommand::Dots => todo!(),
    }
}

fn main() {
    let args = Args::parse();
    let mut settings: Settings = read_settings(args);

    // If user has passed us a path, replace the value in settings with the path
    // provided.
    if let Some(p) = args.path {
        settings.path = PathBuf::from(p);
    }

    match args.cmd {
        cmd => match cmd {
            Some(sub_cmd) => match sub_cmd {
                Command::Add => {
                    if let Err(e) = sanity_checks::check_git() {
                        println!("{}", e);
                        std::process::exit(1)
                    }

                    add(args, settings)
                }
                Command::Install {
                    distro,
                    assume_yes,
                    cmd,
                } => install(args, distro, assume_yes, &cmd),
                Command::Update { only, force } => todo!(),
                Command::Check => todo!(),
            },
            None => (),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{settings::Settings, Args};
    use clap::IntoApp;

    #[test]
    fn verify() {
        Args::command().debug_assert()
    }

    #[test]
    fn read_settings() {
        if let Err(e) = Settings::read() {
            assert!(false, "{}", e);
        }
    }
}
