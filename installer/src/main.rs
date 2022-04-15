mod add;
mod install;
mod sanity_checks;
mod settings;
mod utils;

use crate::add::git;
use clap::{Parser, Subcommand};
use install::{Distro, InstallCommand};
use nix::unistd::Uid;
use settings::Settings;
use std::{path::PathBuf, process};

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

fn read_settings() -> Settings {
    match Settings::new() {
        Ok(s) => s,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    }
}

fn add(settings: Settings) {
    if Uid::effective().is_root() {
        match add::add_file_changes(settings.root, &settings.path) {
            Ok(op) => println!("{:#?}", op.results),
            Err(e) => {
                println!("{}", e);
                process::exit(1)
            }
        }
    }

    match add::add_file_changes(settings.user, &settings.path) {
        Ok(op) => println!("{:#?}", op.results),
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    }

    if let Err(e) = sanity_checks::check_repo() {
        println!("{}", e);
        process::exit(1)
    }

    if let Err(_) = git::add(&settings.path) {
        process::exit(1)
    }

    if let Err(_) = git::commit(&settings.path) {
        process::exit(1)
    }

    if let Err(_) = git::push(&settings.path) {
        process::exit(1)
    }
}

fn install(distro: Option<Distro>, assume_yes: bool, cmd: &InstallCommand) {
    match cmd {
        InstallCommand::Core => todo!(),
        InstallCommand::X11 => todo!(),
        InstallCommand::Wayland => todo!(),
        InstallCommand::Flatpaks => {
            if let Err(e) = sanity_checks::check_flatpak() {
                println!("{}", e);
                process::exit(1)
            }

            todo!()
        }
        InstallCommand::Dots => todo!(),
    }
}

fn main() {
    let args = Args::parse();
    let mut settings: Settings = read_settings();

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
                        process::exit(1)
                    }

                    add(settings)
                }
                Command::Install {
                    distro,
                    assume_yes,
                    cmd,
                } => install(distro, assume_yes, &cmd),
                Command::Update { only, force } => todo!(),
                Command::Check => todo!(),
            },
            // Do nothing.
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
        if let Err(e) = Settings::new() {
            panic!("{}", e);
        }
    }
}
