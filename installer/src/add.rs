pub(super) mod git;

use crate::utils::CopyOperation;
use std::path::{Path, PathBuf};

pub(crate) fn add_file_changes(
    paths: Vec<PathBuf>,
    dest: &Path,
) -> Result<CopyOperation, std::io::Error> {
    let mut copy_op = CopyOperation::new(paths);
    match copy_op.copy_to(&dest) {
        Ok(_) => Ok(copy_op),
        Err(e) => Err(e),
    }
}
