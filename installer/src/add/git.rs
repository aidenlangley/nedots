pub(crate) mod add;

use std::{fmt::Display, path::Path, process::Output};

pub(crate) enum GitError {
    Conflict { path: String },
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conflict { path } => {
                write!(
                    f,
                    "There are conflicting changes at {}, please fix manually.",
                    &path
                )
            }
        }
    }
}

pub(crate) fn add(dest: &Path) -> Result<Output, GitError> {
    add::add(dest)
}
