use console::Term;

/// Implementor would like access to the `Terminal`, provided by the `Console`
/// crate.
pub trait Terminal {
    /// Provides access to stdout.
    fn term(&self) -> Term {
        Term::stdout()
    }

    /// Provides access to stderr.
    fn err(&self) -> Term {
        Term::stderr()
    }
}

#[cfg(test)]
mod tests {
    use super::Terminal;

    struct TestTerminal;
    impl Terminal for TestTerminal {}

    #[test]
    fn term_writes() {
        let test_term = TestTerminal {};
        if let Err(e) = test_term.term().write_line("Testing term!") {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn err_writes() {
        let test_term = TestTerminal {};
        if let Err(e) = test_term.err().write_line("Testing err!") {
            assert!(false, "{}", e)
        }
    }
}
