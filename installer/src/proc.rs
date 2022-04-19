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
    fn run_quietly(&mut self) -> Result<R, E> {
        self.run()
    }

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
                // Print the prog name and args when debugging or when
                // `Verbosity::High`.
                proc.logger().println(
                    Some(Verbosity::High),
                    &format!("{:#?}: {:#?}", proc.prog(), proc.args()),
                );

                // Print the output of `stdout` when `Ok` and `Verbosity` is high enough.
                proc.logger().write_buf(Some(Verbosity::Medium), &o.stdout);

                Ok(o)
            }
            Err(_) => {
                eprint!("Failed to run `{}`! Is it installed?", proc.prog());
                std::process::exit(1)
            }
        }
    }

    fn run_proc_quietly(proc: &mut Process) -> Result<Output, E> {
        Self::run_proc(proc)
    }
}
