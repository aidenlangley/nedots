mod cli;
mod copy;
mod git;
mod install;
mod settings;

pub mod logger;
pub mod proc;

use clap::StructOpt;
use cli::{Args, Command, Verbosity};
use copy::CopyOperation;
use logger::Logger;
use nix::unistd::Uid;
use settings::Settings;
use std::path::PathBuf;

fn add(settings: Settings, logger: &Logger) {
    if let Err(e) = crate::git::stash(&settings.path, logger) {
        logger.println(None, &format!("{:#?}", e));
        eprintln!("{}", e);
        std::process::exit(1)
    }

    if Uid::effective().is_root() {
        let mut copy_op = CopyOperation::new(settings.root);
        match copy_op.copy_to(&settings.path) {
            Ok(_) => logger.println(Some(Verbosity::Medium), &format!("{:#?}", copy_op.results)),
            Err(e) => {
                logger.println(None, &format!("{:#?}", e));
                eprintln!("{}", e);
                std::process::exit(1)
            }
        }
    }

    let mut copy_op = CopyOperation::new(settings.user);
    match copy_op.copy_to(&settings.path) {
        Ok(_) => println!("{:#?}", copy_op.results),
        Err(e) => {
            logger.println(None, &format!("{:#?}", e));
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }

    for func in [
        crate::git::add(&settings.path, logger),
        crate::git::commit(&settings.path, logger),
        crate::git::push(&settings.path, logger),
        crate::git::restore(&settings.path, logger),
    ] {
        if let Err(e) = func {
            logger.println(None, &format!("{:#?}", e));
            eprintln!("{}", e);
            std::process::exit(1)
        }
    }
}

fn update(settings: Settings, logger: &Logger, only: Option<Vec<String>>, force: bool) {}

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
        Command::Add => add(settings, &logger),
        Command::Update { only, force } => update(settings, &logger, only, force),
        _ => todo!(),
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
