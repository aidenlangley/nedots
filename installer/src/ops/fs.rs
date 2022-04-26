use std::path::{Path, PathBuf};
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

#[derive(Debug)]
/// Copies a single file.
struct CopyOperation {
    /// Copy this file.
    from: Option<PathBuf>,

    /// Copy `from` file to this destination.
    to: Option<PathBuf>,
}

impl CopyOperation {
    /// Construct a new `CopyOperation`.
    fn new() -> Self {
        Self {
            from: None,
            to: None,
        }
    }

    /// Assign `from`.
    fn from(mut self, path: &Path) -> Self {
        self.from = Some(path.to_path_buf());
        self
    }

    /// Assign `to`.
    fn to(mut self, path: &Path) -> Self {
        self.to = Some(path.to_path_buf());
        self
    }

    /// Do the copy.
    fn copy(&self) -> Result<(), CopyError> {
        let from = match &self.from {
            Some(f) => {
                // Check the `from` path is a file and not '..' or something odd.
                if let None = f.file_name() {
                    return Err(CopyError::InvalidFileName { path: f.to_owned() });
                }

                f
            }
            None => return Err(CopyError::NoFromPath),
        };

        let mut to = match &self.to {
            Some(f) => {
                if let None = f.file_name() {
                    return Err(CopyError::InvalidFileName { path: f.to_owned() });
                }

                f.to_owned()
            }
            None => return Err(CopyError::NoToPath),
        };

        // When copying from a directory, walk the contents and perform a copy
        // op for each. I pray for you if you've asked me to copy a shit load
        // of subdirectories, I am not multithreaded and don't intend to be.
        if from.is_dir() {
            for e in std::fs::read_dir(from)? {
                let cop = CopyOperation::new().from(&e?.path()).to(&to);
                if let Err(e) = cop.copy() {
                    return Err(e);
                }
            }
        } else {
            if to.is_dir() {
                to = to.join(from.file_name().unwrap());
            }

            if let Err(e) = std::fs::copy(from.canonicalize()?, to) {
                return Err(CopyError::IoError(e));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::CopyOperation;
    use crate::_TESTS_DIR;
    use std::{
        fs::File,
        path::{Path, PathBuf},
    };

    /// Makes the base `copy` directory and returns the `PathBuf`.
    fn setup() -> PathBuf {
        let base_path = Path::new(_TESTS_DIR).join("copy");
        std::fs::create_dir_all(&base_path).expect("Failed to make base_path!");

        base_path
    }

    #[test]
    /// Expects to run a successful `CopyOperation`.
    fn copy_file() {
        let base_path = setup();
        let new_file = Path::new(&base_path).join("COPYING");
        File::create(&new_file).expect("Failed to create new_file!");
        let dest = Path::new(&base_path).join("COPIED");

        if let Err(e) = CopyOperation::new().from(&new_file).to(&dest).copy() {
            assert!(false, "{}", e)
        }

        assert!(dest.is_file());
        std::fs::remove_file(&new_file).expect("Failed to remove COPYING!");
        std::fs::remove_file(&dest).expect("Failed to remove COPIED!");
    }

    #[test]
    /// Checks errors are thrown when a `from` path is not provided, and when a
    /// `to` path is not provided.
    fn no_path() {
        let base_path = setup();
        let cop = CopyOperation::new().from(&Path::new(&base_path).join("COPYING"));
        if let Err(e) = cop.copy() {
            assert_eq!(e.to_string(), "No path to copy `to` provided.")
        } else {
            assert!(false, "Hm? {:#?}", cop)
        }

        let cop = CopyOperation::new().to(&Path::new(&base_path).join("COPIED"));
        if let Err(e) = cop.copy() {
            assert_eq!(e.to_string(), "No path to copy `from` provided.")
        } else {
            assert!(false, "Hm? {:#?}", cop)
        }
    }

    #[test]
    /// Expects a nonsensical file_name to fail.
    fn bad_file_name() {
        let base_path = setup();
        let cop = CopyOperation::new()
            .from(&Path::new(&base_path).join(".."))
            .to(&Path::new(&base_path).join(".."));
        if let Err(e) = cop.copy() {
            assert_eq!(
                e.to_string(),
                format!("Invalid file name: \"{}/..\"", base_path.display())
            )
        } else {
            assert!(
                false,
                "Hm? {} -> {}",
                cop.from.unwrap().display(),
                cop.to.unwrap().display()
            )
        }
    }

    #[test]
    /// Tests providing a directory as opposed to a file to `CopyOperation`.
    /// Expects that `copy` will recurse, by walk the directory and running a
    /// new `CopyOperation` for each.
    fn recurse() {
        let base_path = setup();

        let recurse_dir = &base_path.join("recurse");
        std::fs::create_dir_all(&recurse_dir).expect("Failed to make recurse dir!");

        let recurse_dest_dir = &base_path.join("recurse_dest");
        std::fs::create_dir_all(&recurse_dest_dir).expect("Failed to make recurse_dest dir!");

        for p in [
            Path::new(&recurse_dir).join("COPY0"),
            Path::new(&recurse_dir).join("COPY1"),
        ] {
            File::create(&p).expect(&format!("Failed to create {}", p.display()));
        }

        if let Err(e) = CopyOperation::new()
            .from(&recurse_dir)
            .to(&recurse_dest_dir)
            .copy()
        {
            assert!(false, "{}", e)
        }

        for p in [
            Path::new(&recurse_dest_dir).join("COPY0"),
            Path::new(&recurse_dest_dir).join("COPY1"),
        ] {
            assert!(p.is_file())
        }

        std::fs::remove_dir_all(&base_path.join("recurse")).expect("Failed to remove recurse dir!");
        std::fs::remove_dir_all(&base_path.join("recurse_dest"))
            .expect("Failed to remove recurse_dest dir!");
    }
}
