use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub(crate) fn copy_file<'a>(
    from: &'a Path,
    to: &'a Path,
) -> io::Result<Result<PathBuf, &'static str>> {
    if let Some(file_name) = from.file_name() {
        if let Err(e) = fs::copy(
            from,
            format!("{}/{}", to.display(), file_name.to_string_lossy()),
        ) {
            return Err(e);
        }

        // The good result.
        return Ok(Ok(from.to_path_buf()));
    }

    // Bad but not the worst result, OS has not failed us, but we didn't
    // encounter a valid path to copy.
    Ok(Err(
        "Path did not contain a file or directory name, and instead ended in '..'",
    ))
}

/// Recursively copy all files found in `from` to `to`. `from` may contain
/// files or directories, as the function will walk directories to discover
/// files.
pub(crate) fn copy_files<'a>(
    from: Vec<&'a Path>,
    to: &'a Path,
) -> io::Result<Vec<Result<PathBuf, &'static str>>> {
    let mut list = Vec::new();
    for pb in from {
        match copy_file(&pb, to) {
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
pub(crate) struct CopyOperation {
    /// Paths that will be copied to some destination (dest is specified when
    /// `copy` is called.)
    pub(crate) paths: Vec<PathBuf>,
    /// Tracks status of each copy operation in a vec containing a tuple - left
    /// side is the status, and right side is the original path.
    pub(crate) results: Vec<Result<PathBuf, &'static str>>,
}

impl CopyOperation {
    /// Create a new `CopyOperation` with `paths`.
    pub(crate) fn new(paths: Vec<PathBuf>) -> Self {
        Self {
            paths,
            results: Vec::new(),
        }
    }

    /// Create a `CopyOperation` from a single `PathBuf`.
    pub(crate) fn only(path: PathBuf) -> Self {
        Self {
            paths: vec![path],
            results: Vec::new(),
        }
    }

    /// Recursively copies all files found in `self.paths`, walking
    /// sub-directories as it goes, to `dest`.
    pub(crate) fn copy_to(&mut self, dest: &Path) -> io::Result<()> {
        match copy_files(self.paths.iter().map(|pb| pb.as_path()).collect(), dest) {
            Ok(v) => {
                self.results = v;
                Ok(())
            }
            Err(e) => Err(e),
        }
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
