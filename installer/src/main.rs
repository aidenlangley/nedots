mod cli;
mod copy;
mod git;
mod install;
mod settings;

pub mod logger;
pub mod proc;

use clap::StructOpt;
use cli::{Args, Command};
use git::GitError;
use logger::Logger;
use nix::unistd::Uid;
use settings::Settings;
use std::path::PathBuf;

fn run_git_op(result: Result<String, GitError>, logger: &Logger) {
    match result {
        Ok(msg) => logger.println(None, &msg),
        Err(e) => {
            logger.println(None, &format!("{:#?}", e));
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

fn add(settings: &Settings, logger: &Logger) {
    run_git_op(git::stash(&settings.path, logger), logger);

    if Uid::effective().is_root() {
        // Copy from &settings.root to &settings.path
    }

    // Copy from &settings.user to &settings.path

    run_git_op(git::add(&settings.path, logger), logger);
    run_git_op(git::commit(&settings.path, logger), logger);
    run_git_op(git::push(&settings.path, logger), logger);
    run_git_op(git::restore(&settings.path, logger), logger);
}

fn main() {
    let args = Args::parse();
    let debugging = args.debugging();
    let verbosity = args.get_verbosity();
    let logger = Logger::new(debugging, verbosity);

    logger.println(None, &format!("Args: {:#?}", args));
    if let Some(v) = verbosity {
        logger.println(None, &format!("Verbosity: {:#?}", v));
    }

    let mut settings = match Settings::read() {
        Ok(s) => {
            logger.println(None, &format!("{:#?}", s));
            s
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    };
    logger.println(None, &format!("{:#?}", settings));

    // If user has passed us a path, replace `settings.path`.
    if let Some(p) = args.path {
        settings.path = PathBuf::from(p);
        logger.println(None, &format!("{:#?}", settings.path));
    }

    match args.cmd {
        Command::Add => add(&settings, &logger),
        _ => todo!(),
    }
}

const _TESTS_DIR: &'static str = "tests";

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
