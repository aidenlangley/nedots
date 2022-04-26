use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors thrown during a `CopyOperation`.
pub enum CopyError {
    #[error("No path to copy `from` provided.")]
    /// Need a path to copy data `from`.
    NoFromPath,

    #[error("No path to copy `to` provided.")]
    /// Need a path to copy data `to`.
    NoToPath,

    #[error("Invalid file name: {path:?}")]
    /// Path was probably a '..' or something other than a valid file name.
    InvalidFileName { path: PathBuf },

    #[error(transparent)]
    /// A wrapper around IO errors.
    IoError(#[from] std::io::Error),
}

/// Copies a single file.
struct CopyOperation {
    /// Copy this file.
    from: Option<PathBuf>,

    /// Copy `from` file to this destination.
    to: Option<PathBuf>,
}

impl CopyOperation {
    fn new() -> Self {
        Self {
            from: None,
            to: None,
        }
    }

    fn from(mut self, from: PathBuf) -> Self {
        self.from = Some(from);
        self
    }

    fn to(mut self, from: PathBuf) -> Self {
        self.to = Some(from);
        self
    }

    fn copy(&self) -> Result<(), CopyError> {
        // Check the `from` path is a file and not '..' or something odd.
        let from = match &self.from {
            Some(f) => {
                if let None = f.file_name() {
                    return Err(CopyError::InvalidFileName { path: f.to_owned() });
                }

                f
            }
            None => return Err(CopyError::NoFromPath),
        };

        let to = match &self.to {
            Some(f) => {
                if let None = f.file_name() {
                    return Err(CopyError::InvalidFileName { path: f.to_owned() });
                }

                f
            }
            None => return Err(CopyError::NoToPath),
        };

        if from.is_dir() {
            let cop = CopyOperation::new().from(from.to_owned()).to(to.to_owned());
            if let Err(e) = cop.copy() {
                return Err(e);
            }
        } else {
            let mut to = to.clone();
            if to.is_dir() {
                to = to.join(from.file_name().unwrap());
            }

            if let Err(e) = std::fs::copy(from.canonicalize()?, to.canonicalize()?) {
                return Err(CopyError::IoError(e));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
