use std::process::Output;

use crate::{cli::Verbosity, logger::Logger};

#[derive(Debug, Clone)]
pub struct Process {
    prog: String,
    args: Vec<String>,
    logger: Logger,
}

impl Process {
    pub fn new(prog: &str, args: Vec<&str>, logger: Logger) -> Self {
        Self {
            prog: prog.to_string(),
            args: args.iter().map(|a| a.to_string()).collect(),
            logger,
        }
    }

    pub fn prog(&self) -> &str {
        &self.prog
    }

    pub fn args(&self) -> Vec<&str> {
        self.args.iter().map(|a| a.as_str()).collect()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }
}

pub trait Run<R, E> {
    /// The same as `run_quietly`, but with increased verbosity & interactivity.
    fn run(&mut self) -> Result<R, E>;

    /// Pass a mutable reference to `&self`, and return `R`. We don't want to
    /// restrict what you can do with `&self`, we just want a good `Result`.
    fn run_quietly(&mut self) -> Result<R, E>;

    /// Implementor can return a `min_verbosity` that will determine when `run`
    /// is to give feedback to the user.
    fn min_verbosity(&self) -> Option<Verbosity> {
        None
    }
}

pub trait RunProcess<R, E>: Run<R, E> {
    /// Run a `Command`, returning the `Output` or returning an error of type E.
    fn run_proc(proc: &mut Process) -> Result<Output, E> {
        match std::process::Command::new(proc.prog())
            .args(proc.args())
            .output()
        {
            Ok(o) => {
                proc.logger().write_buf(proc.min_verbosity(), &o.stdout);
                proc.logger().println(
                    Some(Verbosity::High),
                    &format!("{} status: {:#?}", proc.prog(), o.status),
                );
                Ok(o)
            }
            Err(_) => panic!("Failed to run `{}`! Is it installed?", proc.prog()),
        }
    }

    fn run_proc_quietly(proc: &mut Process) -> Result<Output, E> {
        Self::run_proc(proc)
    }
}
