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
use logger::{Logger, Prints};
use nix::unistd::Uid;
use settings::Settings;
use std::path::PathBuf;

fn run_git_op(result: Result<String, GitError>, logger: &Logger) -> Result<(), std::io::Error> {
    match result {
        Ok(msg) => logger.write_line(None, &msg),
        Err(e) => {
            logger.write_line(None, &format!("{:#?}", e))?;
            logger.write_error(format!("{}", e))?;
            std::process::exit(1)
        }
    }
}

fn add(push: bool, settings: &Settings, logger: &Logger) -> Result<(), std::io::Error> {
    run_git_op(git::stash(&settings.path, logger), logger)?;

    if Uid::effective().is_root() {
        // Copy from &settings.root to &settings.path
    }

    // Copy from &settings.user to &settings.path

    run_git_op(git::add(&settings.path, logger), logger)?;
    run_git_op(git::commit(&settings.path, logger), logger)?;
    run_git_op(git::restore(&settings.path, logger), logger)?;

    // When the user passes `-p/--push`...
    if push {
        run_git_op(git::push(&settings.path, logger), logger)?;
    }

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let logger = Logger::new(args.verbosity());

    logger.write_line(None, &format!("Args: {:#?}", args))?;
    // logger.vprintln(None, &format!("Verbosity: {:#?}", logger.verbosity()));

    let mut settings = match Settings::read() {
        Ok(s) => {
            logger.write_line(None, &format!("{:#?}", s))?;
            s
        }
        Err(e) => {
            logger.write_error(format!("{}", e))?;
            std::process::exit(1)
        }
    };
    logger.write_line(None, &format!("{:#?}", settings))?;

    // If user has passed us a path, replace `settings.path`.
    if let Some(p) = args.path {
        settings.path = PathBuf::from(p);
        logger.write_line(None, &format!("{:#?}", settings.path))?;
    }

    match args.cmd {
        Command::Add { push } => add(push, &settings, &logger),
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
