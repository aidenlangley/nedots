use console::Term;
use std::{fmt::Display, io::Write};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
/// Determines the verbosity of the applications output to the terminal.
pub enum Verbosity {
    /// Low leads to the removal of quiet flags being passed to child processes.
    Low = 1,
    /// Medium leads to increased verbosity of child processes.
    Medium = 2,
    /// High leads to increased verbosity of this application in addition to the
    /// above.
    High = 3,
    /// Logs absolutely everything.
    Debug = 4,
}

impl Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verbosity::Low => write!(
                f,
                "Low (leads to the removal of quiet flags being passed to \
                    child processes.)"
            ),
            Verbosity::Medium => write!(
                f,
                "Medium (leads to increased verbosity of child processes where
                    applicable, as well as the verbosity provided by Low.)"
            ),
            Verbosity::High => write!(
                f,
                "High (leads to increased verbosity of this application in \
                    addition to the verbosity provided by Medium.)"
            ),
            Verbosity::Debug => write!(f, "Now debugging."),
        }
    }
}

pub trait Prints<T> {
    fn write_line(&self, min_verbosity: Option<Verbosity>, data: T) -> Result<(), std::io::Error>;
    fn write_error(&self, data: T) -> Result<(), std::io::Error>;
}

pub trait Terminal {
    fn term(&self) -> Term {
        Term::stdout()
    }

    fn err(&self) -> Term {
        Term::stderr()
    }

    fn verbosity(&self) -> Option<Verbosity> {
        None
    }

    fn should_print(&self, min_verbosity: Option<Verbosity>) -> bool {
        if let Some(v) = self.verbosity() {
            if let Some(mv) = min_verbosity {
                return v >= mv;
            }
        }

        false
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Logger {
    verbosity: Option<Verbosity>,
}

impl Logger {
    pub fn new(verbosity: Option<Verbosity>) -> Self {
        Self { verbosity }
    }
}

impl Terminal for Logger {
    fn verbosity(&self) -> Option<Verbosity> {
        self.verbosity
    }
}

impl Prints<&str> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        msg: &str,
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_line(msg)?
        }

        Ok(())
    }

    fn write_error(&self, msg: &str) -> Result<(), std::io::Error> {
        self.err().write_line(msg)
    }
}

impl Prints<&String> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        msg: &String,
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_line(&msg)?
        }

        Ok(())
    }

    fn write_error(&self, msg: &String) -> Result<(), std::io::Error> {
        self.err().write_line(&msg)
    }
}

impl Prints<String> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        msg: String,
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_line(&msg)?
        }

        Ok(())
    }

    fn write_error(&self, msg: String) -> Result<(), std::io::Error> {
        self.err().write_line(&msg)
    }
}

impl Prints<&[u8]> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        buf: &[u8],
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_all(buf)?
        }

        Ok(())
    }

    fn write_error(&self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.err().write_all(&buf)
    }
}

impl Prints<Vec<u8>> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        buf: Vec<u8>,
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_all(&buf)?
        }

        Ok(())
    }

    fn write_error(&self, buf: Vec<u8>) -> Result<(), std::io::Error> {
        self.err().write_all(&buf)
    }
}

impl Prints<&Vec<u8>> for Logger {
    fn write_line(
        &self,
        min_verbosity: Option<Verbosity>,
        buf: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        if self.should_print(min_verbosity) {
            self.term().write_all(&buf)?
        }

        Ok(())
    }

    fn write_error(&self, buf: &Vec<u8>) -> Result<(), std::io::Error> {
        self.err().write_all(&buf)
    }
}
