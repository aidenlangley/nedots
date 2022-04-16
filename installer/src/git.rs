//! Public module that acts as a proxy between `operations` and it's caller.
//! This module provides `git` functions, errors & tests.

mod operations;

use std::{fmt::Display, path::Path, process::Output};

#[derive(Debug)]
pub enum GitError {
    AddFailure,
    Conflict,
    AuthFailure,
    StashPushFailure,
    StashPopFailure,
    PullFailure,
    _ResetFailure,
    Unknown,
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::StashPushFailure => write!(
                f,
                "Failed `git stash push` to push latest changes to stash."
            ),
            GitError::StashPopFailure => write!(
                f,
                "Failed `git stash pop` to restore latest working tree \
                from stash."
            ),
            GitError::AddFailure => write!(
                f,
                "Failed `git add` to add latest \
                changes to tree."
            ),
            GitError::Conflict => write!(
                f,
                "Failed `git commit`, there are conflicting changes, please \
                fix manually."
            ),
            GitError::AuthFailure => write!(
                f,
                "Failed `git push`, authentication failed when pushing to \
                remote."
            ),
            GitError::PullFailure => write!(
                f,
                "Failed `git pull`, latest changes have not been pulled from \
                remote."
            ),
            GitError::_ResetFailure => write!(
                f,
                "Failed `git reset`, the latest commit in the working tree is \
                likely not supposed to be there."
            ),

            GitError::Unknown => write!(
                f,
                "An unknown error occured when attempting to perform a `git` \
                operation, please open an issue on GitHub."
            ),
        }
    }
}

pub fn add(path: &Path) -> Result<Output, GitError> {
    operations::add(path)
}

pub fn commit(path: &Path) -> Result<Output, GitError> {
    operations::commit(path)
}

pub fn push(path: &Path) -> Result<Output, GitError> {
    operations::push(path)
}

pub fn stash(path: &Path) -> Result<Output, GitError> {
    operations::stash_push(path)
}

pub fn restore(path: &Path) -> Result<Output, GitError> {
    operations::stash_pop(path)
}

pub fn pull(path: &Path) -> Result<Output, GitError> {
    operations::pull(path)
}

fn _reset_hard(path: &Path) -> Result<Output, GitError> {
    operations::_reset_hard(path)
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;
    use std::{fs::File, path::Path};

    fn git_stash(path: &Path) {
        let git_stash = super::stash(path);
        match git_stash {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_restore(path: &Path) {
        let git_restore = super::restore(path);
        match git_restore {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_add(path: &Path) {
        let git_add = super::add(path);
        match git_add {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_commit(path: &Path) {
        let git_commit = super::commit(path);
        match git_commit {
            Ok(o) => assert!(o.status.success()),
            Err(e) => assert!(false, "{}", e),
        }
    }

    fn git_reset_hard(path: &Path) {
        let git_revert = super::_reset_hard(path);
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
        let settings = match Settings::read() {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };

        git_stash(&settings.path);

        // Since we've stashed our work, our working tree would be empty (clean)
        // so failing here is okay, since we'd fail at `git_add` anyway.
        File::create("git_commit_test.txt").expect("Failed to create file: `git_commit_test.txt`!");

        git_add(&settings.path);
        git_commit(&settings.path);

        // Notice the use of `git reset **hard**`. This is dangerous. See
        // `git::operations::_reset_hard()` for more information.
        git_reset_hard(&settings.path);

        git_restore(&settings.path);
    }
}
