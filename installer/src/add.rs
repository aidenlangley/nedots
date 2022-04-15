use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use crate::utils::CopyOperation;

pub(crate) enum AddError {
    Conflicts { path: String },
}

impl Display for AddError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AddError::Conflicts { path } => {
                write!(
                    f,
                    "There are conflicting changes for: {}, please fix manually.",
                    &path
                )
            }
        }
    }
}

pub(crate) fn add_changes(
    paths: Vec<PathBuf>,
    dest: &Path,
) -> Result<CopyOperation, std::io::Error> {
    let mut copy_op = CopyOperation::new(paths);
    match copy_op.copy_to(&dest) {
        Ok(_) => Ok(copy_op),
        Err(e) => Err(e),
    }
}

pub(crate) fn git_add(repo_path: &Path) -> Result<(), AddError> {
    todo!()
}

#[cfg(test)]
mod tests {}
