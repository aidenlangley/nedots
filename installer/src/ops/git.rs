use chrono::Local;
use git2::{AnnotatedCommit, IndexAddOption, Oid, Reference, Repository};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors returned during `GitOperation` runtime.
pub enum GitError {
    #[error("No path to a `git` repository was provided.")]
    /// Thrown by a poorly constructed `GitOperation`.
    NoPath,

    #[error("`git` repository has not been opened yet.")]
    /// Thrown if a function is called prematurely, before opening a
    /// `Repository`.
    NoRepo,

    #[error("Push was rejected.")]
    PushRejected,

    #[error(transparent)]
    /// Errors thrown by the `git2` library.
    Git2(#[from] git2::Error),

    #[error("An unknown error ocurred")]
    Unknown,
}

/// A `GitOperation` is constructed and passed a `path`, then a `Repository` is
/// opened and referenced throughout its' lifetime.
pub struct GitOperation {
    /// Path of git repository.
    path: Option<PathBuf>,

    /// `git2::Repository` struct.
    repo: Option<Repository>,
}

impl GitOperation {
    /// Construct a new `GitOperation`.
    pub fn new(remote: Option<String>) -> Self {
        Self {
            path: None,
            repo: None,
        }
    }

    /// Assign `path` and open `Repository`.
    pub fn at_path(mut self, path: &Path) -> Result<Self, GitError> {
        self.path = Some(path.to_path_buf());

        // Open the repo at the given path.
        self.repo = Some(Repository::open(path)?);

        Ok(self)
    }

    /// Get the `path` of git `Repository`.
    fn path(&self) -> Result<&Path, GitError> {
        self.path
            .as_ref()
            .map(|pb| pb.as_path())
            .ok_or(GitError::NoPath)
    }

    /// Borrow the `Repository` - takes a mutable borrow of self so that we can
    /// open it if necessary.
    fn repo(&mut self) -> Result<&Repository, GitError> {
        Ok(self.repo.get_or_insert(Repository::open(self.path()?)?))
    }

    /// Adds latest changes from a dirty tree.
    pub fn add_changes(mut self) -> Result<(Self, Oid), GitError> {
        let mut index = self.repo()?.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;

        Ok((self, index.write_tree()?))
    }

    /// Commit changes - `tree_id` comes from `add_changes`.
    pub fn commit(mut self, tree_id: Oid) -> Result<(Self, Oid), GitError> {
        let repo = self.repo()?;
        let sig = repo.signature()?;
        let oid = repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("Latest {}", Local::now()),
            &repo.find_tree(tree_id)?,
            &[&repo.find_commit(repo.head()?.target().unwrap())?],
        )?;

        Ok((self, oid))
    }

    /// Push changes to `remote`.
    pub fn push(&mut self, remote: &str) -> Result<(), GitError> {
        let repo = self.repo()?;
        repo.find_remote(remote)?.push::<&str>(&[], None)?;

        Ok(())
    }

    /// Fetch changes from `remote` for `branch`.
    pub fn fetch(&mut self, branch: &str, remote: &str) -> Result<AnnotatedCommit, GitError> {
        let repo = self.repo()?;
        repo.find_remote(remote)?.fetch(&[branch], None, None)?;

        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        Ok(repo.reference_to_annotated_commit(&fetch_head)?)
    }

    /// NOTE: This code is pretty much just copied & pasted from:
    /// https://github.com/rust-lang/git2-rs/blob/ae256db3b27dd7dedab02fa5c051bd7adedf7de7/examples/pull.rs
    /// It might need some work!
    pub fn fast_forward(
        &mut self,
        branch: &mut Reference,
        commit: &AnnotatedCommit,
    ) -> Result<(), GitError> {
        let name = branch.name().map_or_else(
            || String::from_utf8_lossy(branch.name_bytes()).to_string(),
            |s| s.to_string(),
        );

        branch.set_target(
            commit.id(),
            &format!("Fast-Forward: {} -> {}", name, commit.id()),
        )?;

        let repo = self.repo()?;
        repo.set_head(&name)?;

        // This line in particular is not a good one to keep as is. The force is
        // apparently overkill, and instead there could be logic to handle dirty
        // working tree state.
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;

        Ok(())
    }

    pub fn rebase(&mut self) -> Result<(), GitError> {
        todo!()
    }

    pub fn merge(&mut self) -> Result<(), GitError> {
        todo!()
    }

    pub fn stash(&mut self) -> Result<Oid, GitError> {
        todo!()
    }

    pub fn restore(mut self) -> Result<Self, GitError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::GitOperation;
    use crate::_TESTS_DIR;
    use chrono::Local;
    use git2::{RemoteCallbacks, Repository};
    use std::{fs::File, path::Path};

    #[test]
    /// Expects to fail because a `GitOperation` was made without a `path` to a
    /// `Repository`.
    fn no_path() {
        if let Err(e) = GitOperation::new(Some("origin".to_string())).path() {
            assert_eq!(e.to_string(), "No path to a `git` repository was provided.")
        }
    }

    #[test]
    /// Expects to fail because a `GitOperation` was made with a `path` to a
    /// directory that isn't a git `Repository`.
    fn bad_path() {
        if let Err(e) = GitOperation::new(Some("origin".to_string())).at_path(Path::new(_TESTS_DIR))
        {
            assert!(e
                .to_string()
                .contains("could not find repository from 'tests'"))
        }
    }

    #[test]
    /// Sets up a `Repository`, performs an `add_changes`, then commits manually
    /// (without leaning on `GitOperation`) to make the initial commit.
    ///
    /// Once the initial commit is made, dirty the tree again via `add_changes`,
    /// the runs `commit` via `GitOperation` to test its impl.
    ///
    /// Then asserts the commit message is as expected, then cleans up by
    /// removing the git directory.
    fn add_commit() {
        let path = Path::new(_TESTS_DIR).join("git");
        Repository::init(&path).expect("Failed to init repository!"); // init will make directories.

        // Make GitOperation & open repo.
        let go = GitOperation::new(Some("origin".to_string()))
            .at_path(&path)
            .expect("Failed to create `GitOperation`!");

        // Dirty the tree with a new file.
        File::create(&path.join("README")).expect("Failed to create README file!");

        // Add changes - mutates `go` so is re-assigned.
        let (mut go, tree_id) = go.add_changes().expect("Failed add_changes README!");

        // Commit the changes.
        let repo = go.repo().expect("Failed to get repo!");
        let sig = repo.signature().expect("Failed to get signature!");
        let dt = Local::now();
        let oid0 = repo
            .commit(
                Some("HEAD"),
                &sig,
                &sig,
                &format!("Latest {}", dt),
                &repo.find_tree(tree_id).expect("Failed to find tree!"),
                &[],
            )
            .expect("Failed commit README!");

        // Dirty the tree with another new file.
        File::create(&path.join("CONTRIBUTING")).expect("Failed to create CONTRIBUTING file!");

        // Add changes once again. Again, mutated so re-assigned.
        let (go, tree_id) = go.add_changes().expect("Failed add_changes CONTRIBUTING!");

        // Commit the changes, this time via `GitOperation::commit`. Again, mutated so re-assigned.
        let (mut go, _) = go.commit(tree_id).expect("Failed commit CONTRIBUTING!");

        // Assert first commit was all goods.
        let commit = go
            .repo()
            .expect("Failed to get repo!")
            .find_commit(oid0)
            .expect("Failed to get README commit!");
        assert_eq!(commit.message().unwrap(), format!("Latest {}", dt));

        // Tidy up.
        std::fs::remove_dir_all(&path).expect("Failed to remove git dir!");
    }

    // #[test]
    /// Tests a `fetch` - don't expect `fetch` to pull in any changes, but we
    /// expect that this won't fail.
    ///
    /// TODO: auth.
    fn fetch() {
        let mut cb = RemoteCallbacks::new();
        cb.credentials(|_user, username_from_url, _allowed_types| todo!());

        let mut go = GitOperation::new(Some("origin".to_string()))
            .at_path(
                std::env::current_dir()
                    .expect("Failed to get current_dir!")
                    .parent()
                    .expect("Failed to get parent of current_dir!"),
            )
            .expect("Failed to create `GitOperation`!");

        go.fetch("main", "origin").expect("Failed to fetch!");
    }
}
