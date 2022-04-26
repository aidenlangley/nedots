mod fs;
mod git;

use self::git::GitOperation;
use crate::output::TerminalLogger;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
enum OperationError {
    #[error(transparent)]
    Git(#[from] git::GitError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

trait Operate {
    /// Run the `Operation`.
    fn run(&self) -> Result<usize, OperationError>;

    /// Return this exit code so the program can terminate correctly.
    fn exit_code(&self) -> usize;
}

/// `Operation` will store information pertaining to the program runtime.
pub struct Operation {
    /// Id of the `Operation`.
    id: usize,

    /// `Logger` to report errors and/or progress.
    logger: TerminalLogger,

    /// `Vec` of `Result` for this `Operation`, a &str will be used to identify
    /// the key-value pair, e.g. `git_add` or `copy_{path}`.
    results: HashMap<&'static str, Result<(), OperationError>>,
}

/// Adds local changes to the `git` repository.
pub struct AddChanges {
    /// The `Operation` that holds the `id`, `logger` and a `Vec` of `Result`.
    op: Option<Operation>,

    /// The `GitOperation` responsible for `add`, `commit` & `push`.
    git_op: Option<GitOperation>,

    /// The `CopyOperation`'s that need to run to copy local files to local
    /// repository.
    copy_ops: Option<Vec<()>>, // TODO: `Vec` of `CopyOperation`.
}

/// Updates local files after pulling latest changes from remote.
pub struct UpdateLocal {
    /// The `Operation` that holds the `id`, `logger` and a `Vec` of `Result`.
    op: Operation,

    /// The `GitOperation` responsible for `fetch` & `fast-forward`.
    git_op: Option<GitOperation>,

    /// The `CopyOperation`'s that need to be run after updating from remote.
    copy_ops: Option<Vec<()>>, // TODO: `Vec` of `CopyOperation`.
}

/// Installs a list of packages.
pub struct InstallPackages {
    /// The `Operation` that holds the `id`, `logger` and a `Vec` of `Result`.
    op: Operation,

    /// `InstallOperation` that will install a list of packages.
    install_op: Option<()>,
}
