use super::{fs, git};
use crate::output::{
    logger::{Logger, Logs, Prints},
    terminal::Terminal,
    verbosity::{MinVerbosity, Verbose, Verbosity},
    TerminalLogger,
};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum OperationError {
    #[error(transparent)]
    Git(#[from] git::GitError),

    #[error(transparent)]
    Copy(#[from] fs::CopyError),
}

pub(crate) trait Operate {
    /// Run the `Operation`.
    fn operate(&self) -> Result<usize, OperationError>;

    /// Return this exit code so the program can terminate correctly.
    fn exit_code(&self) -> usize;
}

/// `Operation` will store information pertaining to the operation runtime.
pub(crate) struct Operation<Logger> {
    /// `Logger` to report errors and/or progress.
    logger: Option<Logger>,

    /// `Vec` of `Result` for this `Operation`, a &str will be used to identify
    /// the key-value pair, e.g. `git_add` or `copy_{path}`.
    results: Option<HashMap<&'static str, Result<(), OperationError>>>,
}

impl<Logger> Operation<Logger> {
    pub(crate) fn new() -> Self {
        Self {
            logger: None,
            results: Some(HashMap::new()),
        }
    }

    pub(crate) fn results(&self) -> &HashMap<&str, Result<(), OperationError>> {
        self.results.as_ref().unwrap()
    }

    pub(crate) fn insert_result(
        &mut self,
        key: &'static str,
        result: Result<(), OperationError>,
    ) -> Option<Result<(), OperationError>> {
        self.results.as_mut().unwrap().insert(key, result)
    }
}

impl Operation<TerminalLogger> {
    pub(crate) fn with_logging(mut self, logger: TerminalLogger) -> Self {
        self.logger = Some(logger);
        self
    }
}

impl Logs<TerminalLogger> for Operation<TerminalLogger> {
    fn logger(&self) -> &TerminalLogger {
        self.logger.as_ref().unwrap()
    }
}

impl Verbose for Operation<TerminalLogger> {
    fn verbosity(&self) -> Option<Verbosity> {
        self.logger().verbosity()
    }
}

impl MinVerbosity for Operation<TerminalLogger> {
    fn min_verbosity(&self) -> Option<Verbosity> {
        Some(Verbosity::Low)
    }
}

impl Logger<TerminalLogger, dyn Terminal, &str> for Operation<TerminalLogger> {
    fn log(&self, msg: &str) -> std::io::Result<()> {
        if self.verbose_enough() {
            return self.logger().write_line(msg);
        }

        Ok(())
    }

    fn log_error(&self, msg: &str) -> std::io::Result<()> {
        self.logger().write_error(msg)
    }
}
