mod args;
mod git;
mod install;
mod sanity_checks;
mod settings;
mod utils;

use args::{Args, Verbosity};
use clap::{StructOpt, Subcommand};
use install::{Distro, InstallCommand};
use nix::unistd::Uid;
use settings::Settings;
use std::path::PathBuf;
use utils::CopyOperation;

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

/// Checks verbosity and then print to terminal.
fn printv(msg: &str, debug: bool, verbosity: Verbosity) {
    if debug || verbosity == Verbosity::High {
        println!("{}", msg)
    }
}

fn read_settings(debug: bool, verbosity: Verbosity) -> Settings {
    printv("Reading settings", debug, verbosity);

    match Settings::read() {
        Ok(s) => {
            if debug {
                println!("{:#?}", s)
            }

            s
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}

fn sanity_check_git(debug: bool, verbosity: Verbosity, settings: &Settings) {
    printv("Performing a sanity check on `git`", debug, verbosity);
    match sanity_checks::check_git() {
        Ok(r) => {
            if debug {
                println!("{:#?}", r);
            }

            r
        }
        Err(e) => {
            if debug {
                println!("{:#?}", e);
            } else {
                println!("{}", e);
            }

            std::process::exit(1)
        }
    }

    printv(
        "Checking if the `git` repo is initialised and ready",
        debug,
        verbosity,
    );
    if let Err(e) = sanity_checks::check_repo(&settings.path) {
        if debug {
            println!("{:#?}", e);
        } else {
            println!("{}", e);
        }

        std::process::exit(1)
    }
}

fn add(debug: bool, verbosity: Verbosity, settings: Settings) {
    // First, check everything is right with `git` on the system.
    sanity_check_git(debug, verbosity, &settings);

    printv("Stashing working `git` tree", debug, verbosity);
    if let Err(e) = git::stash(&settings.path) {
        if debug {
            println!("{:#?}", e);
        } else {
            println!("{}", e);
        }

        std::process::exit(1)
    }

    if Uid::effective().is_root() {
        printv("Beginning a CopyOperation as root", debug, verbosity);
        let mut copy_op = CopyOperation::new(settings.root);
        match copy_op.copy_to(&settings.path) {
            Ok(_) => println!("{:#?}", copy_op.results),
            Err(e) => {
                if debug {
                    println!("{:#?}", e);
                } else {
                    println!("{}", e);
                }

                std::process::exit(1)
            }
        }
    }

    printv("Beginning a CopyOperation", debug, verbosity);
    let mut copy_op = CopyOperation::new(settings.user);
    match copy_op.copy_to(&settings.path) {
        Ok(_) => println!("{:#?}", copy_op.results),
        Err(e) => {
            if debug {
                println!("{:#?}", e);
            } else {
                println!("{}", e);
            }

            std::process::exit(1)
        }
    }

    printv(
        "Performing `git` operations: `add`, `commit`, `push`, `restore`, in that order",
        debug,
        verbosity,
    );
    for func in [
        git::add(&settings.path),
        git::commit(&settings.path),
        git::push(&settings.path),
        git::restore(&settings.path),
    ] {
        if let Err(e) = func {
            if debug {
                println!("{:#?}", e);
            } else {
                println!("{}", e);
            }

            std::process::exit(1)
        }
    }
}

fn update(debug: bool, verbosity: Verbosity, settings: Settings) {
    // As with `add`, check everything is right with `git` on the system.
    sanity_check_git(debug, verbosity, &settings);

    printv("Stashing working `git` tree", debug, verbosity);
    if let Err(e) = git::stash(&settings.path) {
        if debug {
            println!("{:#?}", e);
        } else {
            println!("{}", e);
        }

        std::process::exit(1)
    }

    printv("Stashing working `git` tree", debug, verbosity);
    if let Err(e) = git::stash(&settings.path) {
        if debug {
            println!("{:#?}", e);
        } else {
            println!("{}", e);
        }

        std::process::exit(1)
    }

    printv("Pulling latest changes from remote", debug, verbosity);
    if let Err(e) = git::pull(&settings.path) {
        if debug {
            println!("{:#?}", e);
        } else {
            println!("{}", e);
        }

        std::process::exit(1)
    }
}

fn install(
    debug: bool,
    verbosity: Verbosity,
    distro: Option<Distro>,
    assume_yes: bool,
    cmd: &InstallCommand,
) {
    match cmd {
        InstallCommand::Core => todo!(),
        InstallCommand::X11 => todo!(),
        InstallCommand::Wayland => todo!(),
        InstallCommand::Flatpaks => {
            if let Err(e) = sanity_checks::check_flatpak() {
                if debug {
                    println!("{:#?}", e);
                } else {
                    println!("{}", e);
                }

                std::process::exit(1)
            }

            todo!()
        }
        InstallCommand::Dots => todo!(),
    }
}

fn main() {
    let args = Args::parse();
    let debug = args.debugging();
    let verbosity = args.get_verbosity();

    if debug {
        println!("Debugging enabled");
        println!("Args: {:#?}", args);

        if let Some(v) = verbosity {
            println!("Verbosity set: {:#?}", v);
        }
    }

    let mut settings = read_settings(debug, verbosity.unwrap());

    // If user has passed us a path, replace the value in settings with the path
    // provided.
    if let Some(p) = args.path {
        printv(
            "A path was provided by caller, replacing settings.path with \
            value provided by the user.",
            debug,
            verbosity.unwrap(),
        );
        settings.path = PathBuf::from(p);
    }

    match args.cmd {
        Command::Add => add(debug, verbosity.unwrap(), settings),
        Command::Update { only, force } => todo!(),
        Command::Check => todo!(),
        Command::Install {
            distro,
            assume_yes,
            cmd,
        } => install(debug, verbosity.unwrap(), distro, assume_yes, &cmd),
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
