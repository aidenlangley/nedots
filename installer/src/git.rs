use crate::{
    cli::Verbosity,
    logger::Logger,
    proc::{Process, Run, RunProcess},
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
        Process::run_proc(self)
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
pub fn add(path: &Path, logger: &Logger) -> Result<String, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "add", "."],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        128 => return Ok("No changes".to_string()), // No changes
                        _ => return Err(GitError::AddFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
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
pub fn commit(path: &Path, logger: &Logger) -> Result<String, GitError> {
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
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::CommitFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
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
pub fn push(path: &Path, logger: &Logger) -> Result<String, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "push"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::PushFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
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
pub fn stash(path: &Path, logger: &Logger) -> Result<String, GitError> {
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

            Ok(format!("`{}` successful!", p.prog()))
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
pub fn restore(path: &Path, logger: &Logger) -> Result<String, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "stash", "pop"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::StashPopFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
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
pub fn pull(path: &Path, logger: &Logger) -> Result<String, GitError> {
    let mut p = Process::new(
        "git",
        vec!["-C", &path.to_string_lossy().to_string(), "pull"],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::PullFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
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
fn _reset_hard(path: &Path, logger: &Logger) -> Result<String, GitError> {
    let mut p = Process::new(
        "git",
        vec![
            "-C",
            &path.to_string_lossy().to_string(),
            "reset",
            "--hard",
            "HEAD^0",
        ],
        *logger,
    );

    match Process::run_proc_quietly(&mut p) {
        Ok(o) => {
            let status = o.status;
            if !status.success() {
                if let Some(c) = status.code() {
                    match c {
                        _ => return Err(GitError::_ResetFailure),
                    }
                }
            }

            Ok(format!("`{}` successful!", p.prog()))
        }
        Err(e) => return Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::Verbosity,
        logger::Logger,
        proc::{Process, Run},
    };
    use std::{
        fs::File,
        path::{Path, PathBuf},
        vec,
    };

    fn make_file(test_dir: &Path, file_name: &str, logger: Logger) -> PathBuf {
        let file_path = test_dir.join(file_name);
        if let Err(e) = File::create(&file_path) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Created file: {}", &file_path.display()),
        );

        file_path
    }

    fn remove_file(path: &Path, logger: Logger) {
        if let Err(e) = std::fs::remove_file(path) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Removed file: {}", path.display()),
        );
    }

    fn git_stash(test_dir: &Path, logger: Logger) {
        if let Err(e) = super::stash(test_dir, &logger) {
            assert!(false, "{}", e)
        }
        logger.println(Some(Verbosity::High), &format!("Stashed working tree!"));
    }

    fn git_restore(test_dir: &Path, logger: Logger) {
        if let Err(e) = super::restore(test_dir, &logger) {
            assert!(false, "{}", e)
        }
        logger.println(Some(Verbosity::High), &format!("Restored working tree!"));
    }

    fn git_add(test_dir: &Path, logger: Logger) {
        if let Err(e) = super::add(test_dir, &logger) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Added files to `git` repo!"),
        );
    }

    fn git_commit(test_dir: &Path, logger: Logger) {
        if let Err(e) = super::commit(test_dir, &logger) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Made commit to `git` repo!"),
        );
    }

    // Notice the use of `git reset **hard**`. This is dangerous. See
    // `git::operations::_reset_hard()` for more information.
    fn git_reset(test_dir: &Path, logger: Logger) {
        if let Err(e) = super::_reset_hard(test_dir, &logger) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Reset `git` repo to initial state!"),
        );
    }

    #[test]
    fn stash_add_commit_reset() {
        let logger = Logger::new(true, Some(Verbosity::High));
        let mut test_dir = Path::new(crate::_TESTS_DIR).join("git");

        // Remove directory in case previous tests failed early.
        if let Ok(_) = std::fs::remove_dir_all(&test_dir) {
            logger.println(
                Some(Verbosity::High),
                &format!("Removed dir: {}", &test_dir.display()),
            );
        }

        // Make a new directory that'll be a `git` repository for testing shortly.
        if let Err(e) = std::fs::create_dir_all(&test_dir) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Made dir: {}", &test_dir.display()),
        );

        // Get the full path to the new `git` repository.
        test_dir = match test_dir.canonicalize() {
            Ok(r) => r,
            Err(e) => panic!("{}", e),
        };

        // `git init` the new directory.
        let mut p = Process::new(
            "git",
            vec!["-C", &test_dir.to_string_lossy().to_string(), "init"],
            logger,
        );
        match p.run() {
            Ok(o) => assert!(o.status.success(), "{}", o.status),
            Err(e) => assert!(false, "{}", e),
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Initialised new `git` repository!"),
        );

        // Make a test file for `git` to add & commit.
        let _commit_file = make_file(&test_dir, "committing_me.txt", logger);
        git_add(&test_dir, logger);
        git_commit(&test_dir, logger);

        // Make a file so that the tree is dirty and `git stash` can succeed.
        let dirty_file = make_file(&test_dir, "making_tree_dirty.txt", logger);
        git_add(&test_dir, logger);
        git_stash(&test_dir, logger);
        git_restore(&test_dir, logger);

        // Remove the file that was restored & test.
        remove_file(&dirty_file, logger);
        git_reset(&test_dir, logger);

        // Tidy up after ourselves.
        if let Err(e) = std::fs::remove_dir_all(&test_dir) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Removed dir: {}", &test_dir.display()),
        );
    }
}
