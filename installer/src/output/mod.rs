pub mod logger;
pub mod terminal;
pub mod verbosity;

use console::style;

use self::{
    logger::{Logger, Logs, Prints},
    terminal::Terminal,
    verbosity::{MinVerbosity, Verbose, Verbosity},
};
use std::io::Write;

#[derive(Debug, Clone, Copy)]
/// Holds `Verbosity`, and implements `Terminal`.
pub struct TerminalLogger {
    verbosity: Option<Verbosity>,
}

impl TerminalLogger {
    /// Return an empty `Logger`. Builder pattern.
    pub fn new() -> Self {
        Self { verbosity: None }
    }

    /// Add `Verbosity` to `Logger`.
    pub fn with_verbosity(mut self, verbosity: Option<Verbosity>) -> Self {
        self.verbosity = verbosity;
        self
    }
}

impl Default for TerminalLogger {
    fn default() -> Self {
        Self {
            verbosity: Some(Verbosity::Debug),
        }
    }
}

impl Verbose for TerminalLogger {
    fn verbosity(&self) -> Option<Verbosity> {
        self.verbosity
    }
}

impl MinVerbosity for TerminalLogger {
    fn min_verbosity(&self) -> Option<Verbosity> {
        Some(Verbosity::Debug)
    }
}

impl Terminal for TerminalLogger {}

impl Prints<dyn Terminal, &str> for TerminalLogger {
    /// Write str to stdout.
    ///
    /// ### Panics
    /// If writing to stdout fails.
    fn write_line(&self, msg: &str) -> std::io::Result<()> {
        self.term().write_line(msg)
    }

    /// Write str to stderr.
    ///
    /// ### Panics
    /// If writing to stderr fails.
    fn write_error(&self, msg: &str) -> std::io::Result<()> {
        self.err()
            .write_line(&format!("{}", style(msg).red().bold()))
    }
}

impl Prints<dyn Terminal, &[u8]> for TerminalLogger {
    /// Write buffer to stdout.
    ///
    /// ### Panics
    /// If writing to stdout fails.
    fn write_line(&self, buf: &[u8]) -> std::io::Result<()> {
        self.term().write_all(buf)
    }

    /// Write buffer to stderr.
    ///
    /// ### Panics
    /// If writing to stderr fails.
    fn write_error(&self, buf: &[u8]) -> std::io::Result<()> {
        self.err().write_all(buf)
    }
}

impl Logs<TerminalLogger> for TerminalLogger {
    fn logger(&self) -> &TerminalLogger {
        self
    }
}

impl Logger<TerminalLogger, dyn Terminal, &str> for TerminalLogger {
    fn log(&self, msg: &str) -> std::io::Result<()> {
        if self.verbose_enough() {
            return self.write_line(msg);
        }

        Ok(())
    }

    fn log_error(&self, msg: &str) -> std::io::Result<()> {
        self.write_error(msg)
    }
}

/// Convenience function to write `&str` to terminal via `Logger`.
///
/// ### Panics
/// If `write_line` fails to write to stdout.
pub fn term(msg: &str) {
    TerminalLogger::default()
        .write_line(msg)
        .expect("Failed to write `msg: &str` to stdout!")
}

/// Convenience function to write an error `&str` to terminal via `Logger`.
///
/// ### Panics
/// If `write_error` fails to write to stderr.
pub fn error(msg: &str) {
    TerminalLogger::default()
        .write_error(msg)
        .expect("Failed to write `msg: &str` to stderr!")
}

/// Convenience function to write byte buffer to terminal via `Logger`.
///
/// ### Panics
/// If `write_line` fails to write to stdout.
pub fn term_buf(buf: &[u8]) {
    TerminalLogger::default()
        .write_line(buf)
        .expect("Failed to write `buf: &[u8]` to stdout!")
}

/// Convenience function to write an error byte buffer to terminal via `Logger`.
///
/// ### Panics
/// If `write_error` fails to write to stderr.
pub fn error_buf(buf: &[u8]) {
    TerminalLogger::default()
        .write_error(buf)
        .expect("Failed to write `buf: &[u8]` to stderr!")
}

#[cfg(test)]
mod tests {
    use super::{
        logger::Logger,
        verbosity::{MinVerbosity, Verbose, Verbosity},
        TerminalLogger,
    };

    #[test]
    fn term_logger_verbose_enough() {
        let term = TerminalLogger::new().with_verbosity(Some(Verbosity::Debug));
        assert_eq!(term.verbosity(), Some(Verbosity::Debug));
        assert!(term.verbose_enough())
    }

    #[test]
    fn term_logger_default_verbose_enough() {
        let term = TerminalLogger::default();
        assert_eq!(term.verbosity(), Some(Verbosity::Debug));
        assert!(term.verbose_enough())
    }

    #[test]
    fn term_log_ok() {
        let term = TerminalLogger::default();
        if let Err(e) = term.log("Testing term logs!") {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn term_log_err() {
        let term = TerminalLogger::new().with_verbosity(Some(Verbosity::High));
        assert_ne!(term.verbosity(), Some(Verbosity::Debug));
        assert_eq!(term.verbose_enough(), false);
    }
}
