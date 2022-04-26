/// Implementor `Prints` data of `Type` to output `Via`, for example:
///
/// ```
/// impl Prints<dyn Terminal, &str> for TerminalLogger {
///     ...
/// }
/// ```
pub trait Prints<Via: ?Sized, Type> {
    fn write_line(&self, data: Type) -> std::io::Result<()>;
    fn write_error(&self, data: Type) -> std::io::Result<()>;
}

/// Something that `Logs` will have constructed something of type `T` that
/// `Prints`.
pub trait Logs<T> {
    /// Borrows a logger.
    fn logger(&self) -> &T;
}

/// A `Logger` must use something that `Logs` in order to write `Type` data
/// `Via` some method, for example a `TerminalLogger`.
///
/// ```
/// impl Logger<TerminalLogger, dyn Terminal, &str> for SomeOperation {
///     ...
/// }
/// ```
///
/// These functions must be implemented manually since this uses dynamic
/// dispath, and the compiler must know about any size constraints of the
/// implementor.
pub trait Logger<Logger: Prints<Via, Type>, Via: ?Sized, Type>: Logs<Logger> {
    /// Should typically check if `verbose_enough` to `write_line` by also
    /// implementing `MinVerbosity`, otherwise just print some data `Via`.
    ///
    /// ### A Good Default
    /// ```
    /// fn log(&self, msg: &str) -> std::io::Result<()> {
    ///     if self.verbose_enough() {
    ///         return self.logger().write_line(msg);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// This must be implemented manually since this uses dynamic dispath, and
    /// the compiler must know about any size constraints of the implementor.
    ///
    /// ### Panics
    /// If `write_line` fails to write to stdout.
    fn log(&self, data: Type) -> std::io::Result<()>;

    /// Shortcut to `write_error`.
    ///
    /// ### A Good Default
    /// ```
    /// fn log_error(&self, msg: &str) -> std::io::Result<()> {
    ///    self.logger().write_error(msg)
    /// }
    /// ```
    ///
    /// This must be implemented manually since this uses dynamic dispath, and
    /// the compiler must know about any size constraints of the implementor.
    ///
    /// ### Panics
    /// If `write_error` fails to write to stderr.
    fn log_error(&self, data: Type) -> std::io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::{Logs, Prints};
    use crate::output::terminal::Terminal;
    use std::io::Write;

    #[derive(Debug, PartialEq, Eq)]
    struct Logger;

    impl Terminal for Logger {}

    impl Logs<Logger> for Logger {
        fn logger(&self) -> &Logger {
            self
        }
    }

    impl Prints<Logger, &str> for Logger {
        fn write_line(&self, msg: &str) -> std::io::Result<()> {
            self.term().write_line(msg)
        }

        fn write_error(&self, msg: &str) -> std::io::Result<()> {
            self.err().write_line(msg)
        }
    }

    impl Prints<Logger, &[u8]> for Logger {
        fn write_line(&self, buf: &[u8]) -> std::io::Result<()> {
            self.term().write_all(buf)
        }

        fn write_error(&self, buf: &[u8]) -> std::io::Result<()> {
            self.err().write_all(buf)
        }
    }

    #[test]
    fn logs() {
        let logger = Logger {};
        assert_eq!(logger.logger(), &logger);
    }

    #[test]
    fn prints_str_stdout() {
        let logger = Logger {};
        if let Err(e) = logger.write_line("Testing prints &str to stdout!") {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn prints_str_stderr() {
        let logger = Logger {};
        if let Err(e) = logger.write_line("Testing prints &str to stderr!") {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn prints_buf_stdout() {
        let logger = Logger {};
        if let Err(e) = logger.write_line(b"Testing prints &[u8] to stdout!".as_slice()) {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn prints_buf_stderr() {
        let logger = Logger {};
        if let Err(e) = logger.write_line(b"Testing prints &[u8] to stderr!".as_slice()) {
            assert!(false, "{}", e)
        }
    }
}
