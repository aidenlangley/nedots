use crate::{cli::Verbosity, logger::Logger};
use std::process::Output;

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

pub trait Run<E> {
    /// Run a `Command`, returning the `Output` or returning an error of type E.
    fn run(&self, proc: &Process) -> Result<Output, E> {
        match std::process::Command::new(proc.prog())
            .args(proc.args())
            .output()
        {
            Ok(o) => {
                proc.logger().write_buf(self.min_verbosity(), &o.stdout);
                Ok(o)
            }
            Err(_) => panic!("Failed to run `{}`! Is it installed?", proc.prog()),
        }
    }

    /// Runs quietly when passed the flag "-q".
    fn run_quietly(&self, proc: &Process) -> Result<Output, E>;

    /// `run` will print to stdout when `nedots` is run with Verbosity higher
    /// than this.
    fn min_verbosity(&self) -> Option<Verbosity>;
}
