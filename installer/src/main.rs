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
use std::{
    io::{stdout, Write},
    process::exit,
};

#[derive(Debug, Parser)]
#[clap(about = "Tool for installing & managing ne/any-dots.")]
#[clap(version, arg_required_else_help(true))]
struct Args {
    #[clap(short, long, parse(from_occurrences))]
    debug: usize,

    #[clap(short, long, parse(from_occurrences))]
    verbose: usize,

    #[clap(short, long, env = "HOME")]
    #[clap(help = "Path of nedots parent data directory")]
    path: String,

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
            exit(1)
        }
    }
}

fn add(settings: Settings) {
    if Uid::effective().is_root() {
        match add::add_file_changes(settings.root, &settings.path) {
            Ok(op) => println!("{:#?}", op.results),
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        }
    }

    match add::add_file_changes(settings.user, &settings.path) {
        Ok(op) => println!("{:?}", op.results),
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }

    match git::add(&settings.path) {
        Ok(o) => stdout()
            .write_all(&o.stdout)
            .expect("Failed to write stdout!"),
        Err(e) => {
            println!("{}", e);
            exit(1)
        }
    }
}

fn install(distro: Option<Distro>, assume_yes: bool, cmd: &InstallCommand) {
    match cmd {
        InstallCommand::Core => todo!(),
        InstallCommand::X11 => todo!(),
        InstallCommand::Wayland => todo!(),
        InstallCommand::Flatpaks => todo!(),
        InstallCommand::Dots => todo!(),
    }
}

fn main() {
    let args = Args::parse();
    let settings: Settings = read_settings();

    match args.cmd {
        cmd => match cmd {
            Some(sub_cmd) => match sub_cmd {
                Command::Add => add(settings),
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
    use crate::{sanity_checks, settings::Settings, Args};
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

    #[test]
    fn run_sanity_checks() {
        if let Err(e) = sanity_checks::check_git() {
            panic!("{}", e);
        }

        if let Err(e) = sanity_checks::check_flatpak() {
            panic!("{}", e);
        }
    }
}
