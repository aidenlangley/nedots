//! Public module that acts as a proxy between `operations` and it's caller.
//! This module provides `git` functions, errors & tests.

mod operations;

use std::{fmt::Display, path::Path, process::Output};

pub enum GitError {
    AddFailure,
    Conflict,
    AuthFailure,
    StashPushFailure,
    StashPopFailure,
    _RevertFailure,
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
            GitError::_RevertFailure => write!(
                f,
                "Failed `git revert`, the latest commit in the working tree is \
                likely not supposed to be there."
            ),
            GitError::AuthFailure => write!(
                f,
                "Failed `git push`, authentication failed when pushing to \
                remote."
            ),
            GitError::Unknown => write!(
                f,
                "An unknown error occured when attempting to perform a `git` \
                operation, please open an issue on GitHub."
            ),
        }
    }
}

pub fn add(dest: &Path) -> Result<Output, GitError> {
    operations::add(dest)
}

pub fn commit(dest: &Path) -> Result<Output, GitError> {
    operations::commit(dest)
}

pub fn push(dest: &Path) -> Result<Output, GitError> {
    operations::push(dest)
}

pub fn stash(dest: &Path) -> Result<Output, GitError> {
    operations::stash_push(dest)
}

pub fn restore(dest: &Path) -> Result<Output, GitError> {
    operations::stash_pop(dest)
}

fn _reset_hard(dest: &Path) -> Result<Output, GitError> {
    operations::_reset_hard(dest)
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
        let settings: Settings = crate::read_settings();
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
