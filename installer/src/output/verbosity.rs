use std::fmt::Display;

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
            Verbosity::Low => write!(f, "Low"),
            Verbosity::Medium => write!(f, "Medium"),
            Verbosity::High => write!(f, "High"),
            Verbosity::Debug => write!(f, "Debug"),
        }
    }
}

/// Implementors of `Verbose` will be associated with a level of `Verbosity`.
pub trait Verbose {
    /// Returns level of `Verbosity`.
    fn verbosity(&self) -> Option<Verbosity>;
}

/// The `Logs` trait, in combination with `MinVerbosity`, provides a way to
/// check if a process is `verbose_enough` to print messages to the terminal.
/// Implementor must first know how verbose it wants to be by implementing
/// `Verbose`.
pub trait MinVerbosity: Verbose {
    /// The minimum `Verbosity` at which the implementor should print
    /// output. None means no output, but is ignored when
    /// `Verbose::Verbosity` is `Verbosity::Debug`.
    fn min_verbosity(&self) -> Option<Verbosity>;

    /// Check if implementor is set to a `Verbosity` greater than
    /// `min_verbosity`, or is set to `Verbosity::Debug`. Typically callers
    /// of this function will print some info to the terminal afterwards.
    fn verbose_enough(&self) -> bool {
        if let Some(v) = self.verbosity() {
            if v == Verbosity::Debug {
                return true;
            }

            if let Some(mv) = self.min_verbosity() {
                return v >= mv;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::{MinVerbosity, Verbose, Verbosity};

    #[test]
    fn display() {
        assert_eq!(Verbosity::Low.to_string(), "Low");
        assert_eq!(Verbosity::Medium.to_string(), "Medium");
        assert_eq!(Verbosity::High.to_string(), "High");
        assert_eq!(Verbosity::Debug.to_string(), "Debug");
    }

    struct VerboseEnough;

    impl Verbose for VerboseEnough {
        fn verbosity(&self) -> Option<Verbosity> {
            Some(Verbosity::Debug)
        }
    }

    impl MinVerbosity for VerboseEnough {
        fn min_verbosity(&self) -> Option<Verbosity> {
            Some(Verbosity::Debug)
        }
    }

    #[test]
    fn verbose_enough() {
        let v = VerboseEnough {};
        assert_eq!(v.verbosity(), Some(Verbosity::Debug));
        assert!(v.verbose_enough());
    }

    struct NotVerboseEnough;

    impl Verbose for NotVerboseEnough {
        fn verbosity(&self) -> Option<Verbosity> {
            Some(Verbosity::High)
        }
    }

    impl MinVerbosity for NotVerboseEnough {
        fn min_verbosity(&self) -> Option<Verbosity> {
            Some(Verbosity::Debug)
        }
    }

    #[test]
    fn not_verbose_enough() {
        let nv = NotVerboseEnough {};
        assert_eq!(nv.verbosity(), Some(Verbosity::High));
        assert_eq!(nv.verbose_enough(), false);
    }
}
