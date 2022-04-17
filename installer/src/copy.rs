use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum CopyError {
    InvalidFileName,
    Error(std::io::Error),
}

impl Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::InvalidFileName => write!(
                f,
                "Path did not contain a file or directory name, and instead ended in '..'"
            ),
            CopyError::Error(e) => write!(f, "{}", e),
        }
    }
}

/// Copy file at `from` path to destination at `to` path.
///
/// ### Errors
/// When the file found at `from` does not have a file name, and instead is
/// something like '..', an appropriate error is returned. When there is an
/// error during the copy operation, an appropriate error is returned.
pub fn copy_file<'a>(from: &'a Path, to: &'a Path) -> std::io::Result<Result<PathBuf, CopyError>> {
    if let Some(file_name) = from.file_name() {
        if let Err(e) = std::fs::copy(
            from,
            format!("{}/{}", to.display(), file_name.to_string_lossy()),
        ) {
            return Ok(Err(CopyError::Error(e)));
        }

        // The good result.
        return Ok(Ok(from.to_path_buf()));
    }

    // Bad but not the worst result, OS has not failed us, but we didn't
    // encounter a valid path to copy.
    Ok(Err(CopyError::InvalidFileName))
}

/// Iterates over a vec of tuples, and copies the file from 1st index to the
/// path at the 2nd index.
pub fn copy_each(
    path_pairs: Vec<(&Path, &Path)>,
) -> std::io::Result<Vec<Result<PathBuf, CopyError>>> {
    let mut list = Vec::new();
    for p in path_pairs {
        match copy_file(p.0, p.1) {
            Ok(r) => match r {
                Ok(p) => list.push(Ok(p)),
                Err(e) => list.push(Err(e)),
            },
            Err(e) => return Err(e),
        }
    }

    Ok(list)
}

/// A helper struct for performing copy operations.
pub struct CopyOperation {
    /// Paths that will be copied to some destination (dest is specified when
    /// `copy` is called.)
    pub paths: Vec<PathBuf>,
    /// Tracks status of each copy operation in a vec containing a tuple - left
    /// side is the status, and right side is the original path.
    pub results: Vec<Result<PathBuf, CopyError>>,
}

impl CopyOperation {
    /// Create a new `CopyOperation` with `paths`.
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            results: Vec::new(),
        }
    }

    /// Create a `CopyOperation` from a single `PathBuf`.
    pub fn only(path: PathBuf) -> Self {
        Self {
            paths: vec![path],
            results: Vec::new(),
        }
    }

    /// Recursively copies all files found in `self.paths`, walking
    /// sub-directories as it goes, to `dest`.
    pub fn copy_to(&mut self, dest: &Path) -> std::io::Result<()> {
        for pb in &self.paths {
            match copy_file(&pb, dest) {
                Ok(r) => match r {
                    Ok(p) => self.results.push(Ok(p)),
                    Err(e) => self.results.push(Err(e)),
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}

impl From<Vec<&Path>> for CopyOperation {
    fn from(vec: Vec<&Path>) -> Self {
        Self {
            paths: vec.iter().map(|p| p.into()).collect(),
            results: Vec::new(),
        }
    }
}

impl From<&Path> for CopyOperation {
    fn from(path: &Path) -> Self {
        CopyOperation::only(path.into())
    }
}

#[cfg(test)]
mod tests {}
