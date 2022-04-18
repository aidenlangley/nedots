use crate::{
    cli::Verbosity,
    logger::Logger,
    proc::{self, Process, Run, RunProcess},
};
use chrono::Local;
use std::{fmt::Display, path::Path, process::Output};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GitError {
    AddFailure,
    CommitFailure,
    Conflicts,
    NothingToCommit,
    PushFailure,
    AuthFailure,
    StashPushFailure,
    StashPopFailure,
    PullFailure,
    _ResetFailure,
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::AddFailure => {
                write!(f, "`git add` failed! Could not add latest changes to tree.")
            }
            GitError::CommitFailure => {
                write!(f, "`git commit` failed! Could not commit latest changes.")
            }
            GitError::Conflicts => {
                write!(
                    f,
                    "Found conflicting changes during commit, please fix \
                        manually."
                )
            }
            GitError::NothingToCommit => {
                write!(f, "There were no new changes to commit.")
            }
            GitError::PushFailure => {
                write!(
                    f,
                    "`git push` failed! Latest changes were not pushed to remote."
                )
            }
            GitError::AuthFailure => {
                write!(f, "Authentication failed when pushing to remote.")
            }
            GitError::StashPushFailure => {
                write!(
                    f,
                    "`git stash push` failed! Latest changes were not pushed \
                        to stash."
                )
            }
            GitError::StashPopFailure => {
                write!(
                    f,
                    "`git stash pop` failed! Unable to restore latest working \
                        tree from stash."
                )
            }
            GitError::PullFailure => {
                write!(
                    f,
                    "`git pull` failed! Latest changes have not been pulled \
                        from remote."
                )
            }
            GitError::_ResetFailure => {
                write!(
                    f,
                    "`git reset` failed! The latest commit in the working tree \
                        is likely not supposed to be there."
                )
            }
        }
    }
}

impl Run<Output, GitError> for Process {
    fn run(&mut self) -> Result<Output, GitError> {
        panic!("Run me via `run_proc`, not `run")
    }

    fn run_quietly(&mut self) -> Result<Output, GitError> {
        panic!("Run me via `run_proc_quietly`, not `run_quietly")
    }

    fn min_verbosity(&self) -> Option<Verbosity> {
        Some(Verbosity::Low)
    }
}

impl RunProcess<Output, GitError> for Process {
    /// Adds "-q" (quiet) flag to `Command` args when `Verbosity` is greater
    /// than `Verbosity::Low`.
    fn run_proc_quietly(proc: &mut Process) -> Result<Output, GitError> {
        if let Some(v) = proc.logger().verbosity {
            if let Some(mv) = proc.min_verbosity() {
                if !proc.logger().debugging && v > mv {
                    proc.args().push("-q");
                    proc.logger()
                        .println(Some(Verbosity::High), &format!("Args: {:#?}", proc.args()));
                }
            }
        }

        Self::run_proc(proc)
    }
}

/// Wrapper function around `std::process::Command`: `git add .`
///
/// ### Errors
/// Unsure what errors might be thrown by this operation, but throws
/// `GitError::AddFailure` or `GitError::Unknown` accordingly.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn add(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "add", "."],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::AddFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git commit -m "Latest
/// ($datetime)"`.
///
/// ### Errors
/// Change conflicts throw `GitError::Conflict`,  or `GitError::Unknown` if the
/// error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn commit(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec![
            "-C",
            &path.to_string_lossy().to_string(),
            "commit",
            "-m",
            &format!("Latest ({})", Local::now()),
        ],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::CommitFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git push`.
///
/// ### Errors
/// Authentication errors throw `GitError::AuthFailure`,  or `GitError::Unknown`
/// if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn push(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "push"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::PushFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git stash push`. Primarily
/// used when performing operations on this `git` repo as part of the
/// functionality provided by the binary, we only want to add changes from
/// external files, in other words, not the installer code, just the dot file
/// changes.
///
/// ### Errors
/// Authentication errors throw `GitError::StashPushFailure`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn stash(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "stash", "push"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::StashPushFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git stash pop`. Primarily
/// used when performing operations on this `git` repo as part of the
/// functionality provided by the binary, we only want to add changes from
/// external files, in other words, not the installer code, just the dot file
/// changes.
///
/// ### Errors
/// Authentication errors throw `GitError::StashPopFailure`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn restore(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "stash", "pop"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::StashPopFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git pull`.
///
/// ### Errors
/// Authentication errors throw `GitError::PullError`, or
/// `GitError::Unknown` if the error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
pub fn pull(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "pull"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::PullFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

/// Wrapper function around `std::process::Command`: `git rest --hard HEAD^`. Used in
/// conjunction with `commit` to roll back our commit after testing.
///
/// This is a dangerous command and can really fuck your `git` tree. We really
/// do want to nuke the latest commit, since this is a test, so we're happy to
/// use `--hard`, and `HEAD^` refers to the last commit.
///
/// ### Errors
/// Failures throw a `GitError::_RevertFailure`, or `GitError::Unknown` if the
/// error is unrecognised.
///
/// ### Panics
/// Panics when `std::io::stdout().write_all(buf)` fails to write to `stdout`.
fn _reset_hard(path: &Path, logger: &Logger) -> Result<Output, GitError> {
    let mut p = Process::new(
        "git",
        vec![
            "-C",
            &path.to_string_lossy().to_string(),
            "reset",
            "--hard",
            "HEAD^",
        ],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::_ResetFailure),
                    }
                }
            }

            Ok(o)
        }
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{cli::Verbosity, logger::Logger, settings::Settings};
    use std::{fs::File, path::Path};

    fn git_stash(path: &Path, logger: &Logger) {
        let git_stash = super::stash(path, &logger);
        match git_stash {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_restore(path: &Path, logger: &Logger) {
        let git_restore = super::restore(path, &logger);
        match git_restore {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_add(path: &Path, logger: &Logger) {
        let git_add = super::add(path, &logger);
        match git_add {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_commit(path: &Path, logger: &Logger) {
        let git_commit = super::commit(path, &logger);
        match git_commit {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_reset_hard(path: &Path, logger: &Logger) {
        let git_revert = super::_reset_hard(path, &logger);
        match git_revert {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    /// In a nut shell, writes a file, runs `git_add`, followed by `git_commit`,
    /// and then rolls back. Stashes and restores working tree to avoid messing
    /// with dev work.
    ///
    /// ### Asserts
    /// Each & every git operation must pass, the creation, and subsequent
    /// deletion of the file necessary for the operations to run, must also
    /// pass and/or not panic.
    ///
    /// ### Panics
    /// When the test fails to create necessary files for testing the `git`
    /// operations.
    fn all_ops() {
        let logger = Logger::new(true, Some(Verbosity::High));
        let settings = match Settings::read() {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };

        git_stash(&settings.path, &logger);

        // Since we've stashed our work, our working tree would be empty (clean)
        // so failing here is okay, since we'd fail at `git_add` anyway.
        File::create("git_commit_test.txt").expect("Failed to create file!");

        git_add(&settings.path, &logger);
        git_commit(&settings.path, &logger);

        // Notice the use of `git reset **hard**`. This is dangerous. See
        // `git::operations::_reset_hard()` for more information.
        git_reset_hard(&settings.path, &logger);

        git_restore(&settings.path, &logger);
    }
}
