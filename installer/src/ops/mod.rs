pub(crate) mod fs;
pub(crate) mod git;
pub(crate) mod op;

use self::{
    git::GitOp,
    op::{Operation, OperationError},
};
use crate::output::TerminalLogger;
use fs::CopyOp;
use std::path::{Path, PathBuf};

/// Adds local changes to the `git` repository.
pub(crate) struct AddChanges<'remote> {
    pub(crate) parent_op: Operation<TerminalLogger>,

    /// The `GitOperation` responsible for `add`, `commit` & `push`.
    pub(crate) git_op: Option<GitOp<'remote>>,

    /// The `CopyOperation`'s that need to run to copy local files to local
    /// repository.
    pub(crate) copy_ops: Vec<CopyOp>,
}

impl<'remote> AddChanges<'remote> {
    pub(crate) fn new(op: Operation<TerminalLogger>) -> Self {
        Self {
            parent_op: op,
            git_op: None,
            copy_ops: Vec::new(),
        }
    }

    pub(crate) fn to(mut self, to: PathBuf) -> Result<Self, OperationError> {
        if let Some(_) = &self.git_op {
            self.git_op = Some(self.git_op.unwrap().at_path(&to)?);
        } else {
            self.git_op = Some(GitOp::new().at_path(&to)?);
        }
        Ok(self)
    }

    pub(crate) fn copy_these(mut self, paths: Vec<PathBuf>) -> Result<Self, OperationError> {
        for p in &paths {
            self.copy_ops.push(
                CopyOp::new()
                    .from(&Path::new(env!("HOME")).join(&p))
                    .to(&Path::new(self.git_op.as_ref().unwrap().path()?).join(&p)),
            );
        }

        Ok(self)
    }

    pub(crate) fn to_remote(mut self, remote: &'remote str) -> Result<Self, OperationError> {
        if let Some(_) = &self.git_op {
            self.git_op = Some(self.git_op.unwrap().with_remote(remote));
        } else {
            self.git_op = Some(GitOp::new().with_remote(remote));
        }

        Ok(self)
    }
}

/// Updates local files after pulling latest changes from remote.
struct UpdateLocal<'remote> {
    /// The `GitOperation` responsible for `fetch` & `fast-forward`.
    git_op: Option<GitOp<'remote>>,

    /// The `CopyOperation`'s that need to be run after updating from remote.
    copy_ops: Option<Vec<()>>, // TODO: `Vec` of `CopyOperation`.
}

/// Installs a list of packages.
struct InstallPackages {
    /// `InstallOperation` that will install a list of packages.
    install_op: Option<()>,
}
