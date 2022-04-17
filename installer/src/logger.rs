use crate::cli::Verbosity;
use std::io::Write;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Logger {
    pub debugging: bool,
    pub verbosity: Option<Verbosity>,
}

impl Logger {
    pub fn new(debugging: bool, verbosity: Option<Verbosity>) -> Self {
        Self {
            debugging,
            verbosity,
        }
    }

    /// Prints `msg` to terminal when `self.verbosity` is greater than
    /// `min_verbosity`, or when `self.debugging`.
    pub fn println(&self, min_verbosity: Option<Verbosity>, msg: &str) {
        if self.debugging {
            println!("{}", msg)
        } else if let Some(v) = self.verbosity {
            if let Some(mv) = min_verbosity {
                if v >= mv {
                    println!("{}", msg)
                }
            }
        }
    }

    /// Writes buffer to `stdout`, the same checks as `println` are performed.
    pub fn write_buf(&self, min_verbosity: Option<Verbosity>, buf: &[u8]) {
        if self.debugging {
            std::io::stdout()
                .write_all(buf)
                .expect("Failed to write to stdout!");
        } else if let Some(v) = self.verbosity {
            if let Some(mv) = min_verbosity {
                if v >= mv {
                    std::io::stdout()
                        .write_all(buf)
                        .expect("Failed to write to stdout!");
                }
            }
        }
    }
}
