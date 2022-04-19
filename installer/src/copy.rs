use dialoguer::Confirm;

use crate::{cli::Verbosity, logger::Logger, proc::Run};
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum CopyError {
    InvalidFileName { path: PathBuf },
    FoundDirectory,
    Error(std::io::Error),
}

impl Display for CopyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CopyError::InvalidFileName { path } => {
                write!(
                    f,
                    "{} is invalid. Is it relative, or malformed?",
                    path.display()
                )
            }
            CopyError::FoundDirectory => {
                write!(
                    f,
                    "Encountered a directory & decided not to recursively copy \
                        its' contents."
                )
            }
            CopyError::Error(e) => write!(f, "{}", e),
        }
    }
}

/// A helper struct for performing copy operations.
pub struct CopyOperation {
    paths: Vec<PathBuf>,
    dest: PathBuf,
    logger: Logger,
}

impl CopyOperation {
    pub fn new(paths: &[PathBuf], dest: &Path, logger: &Logger) -> Self {
        Self {
            paths: paths.to_vec(),
            dest: dest.to_path_buf(),
            logger: *logger,
        }
    }

    pub fn only(path: &Path, dest: &Path, logger: &Logger) -> Self {
        Self {
            paths: vec![path.to_path_buf()],
            dest: dest.to_path_buf(),
            logger: *logger,
        }
    }
}

impl Run<Vec<PathBuf>, CopyError> for CopyOperation {
    /// See `run_quietly` for documentation, this function operates the same,
    /// but is littered with logging or interactivity.
    fn run(&mut self) -> Result<Vec<PathBuf>, CopyError> {
        let mut results = Vec::new();
        for pb in &self.paths {
            if let Some(fin) = pb.file_name() {
                if pb.is_dir() {
                    if let Ok(o) = Confirm::new()
                        .with_prompt(format!("Recursing into {}. Continue?", pb.display()))
                        .default(true)
                        .interact_opt()
                    {
                        if let Some(_) = o {
                            let mut cop = CopyOperation::only(pb, &self.dest, &self.logger);
                            match cop.run_quietly() {
                                Ok(mut r) => results.append(&mut r),
                                Err(e) => return Err(e),
                            }
                        }
                    }

                    eprintln!("{}", CopyError::FoundDirectory);
                    std::process::exit(1)
                }

                let mut dest_path = self.dest.clone();
                if dest_path.is_dir() {
                    dest_path = dest_path.join(fin);
                }

                match std::fs::copy(pb, &dest_path) {
                    Ok(_) => {
                        self.logger.println(
                            Some(Verbosity::Medium),
                            &format!("Copied {} to {}", pb.display(), &dest_path.display()),
                        );
                        results.push(dest_path.to_path_buf());
                    }
                    Err(e) => return Err(CopyError::Error(e)),
                }
            } else {
                return Err(CopyError::InvalidFileName {
                    path: pb.to_path_buf(),
                });
            }
        }

        Ok(results)
    }

    fn run_quietly(&mut self) -> Result<Vec<PathBuf>, CopyError> {
        // Store our results here - especially since this function can recurse.
        // Mutable because it will change when we recurse.
        let mut results = Vec::new();

        // Go through all of the files that belong to our `CopyOperation`.
        for pb in &self.paths {
            // See `file_name` description for more info.
            if let Some(fin) = pb.file_name() {
                // Recurse when we've encountered a directory.
                if pb.is_dir() {
                    let mut cop = CopyOperation::only(&pb, &self.dest, &self.logger);
                    match cop.run_quietly() {
                        Ok(mut r) => results.append(&mut r),
                        Err(e) => return Err(e),
                    }
                }

                // When the destination is a directory, we have to append the
                // filename to the end of the path.
                let mut dest_path = self.dest.clone();
                if dest_path.is_dir() {
                    dest_path = dest_path.join(fin);
                }

                // Copy the file, throw errors.
                match std::fs::copy(pb, &dest_path) {
                    Ok(_) => results.push(dest_path.to_path_buf()),
                    Err(e) => return Err(CopyError::Error(e)),
                }
            } else {
                // Intentionally return, the design is to stop on failure, and
                // let the user fix the problems, before we perform a clean run.
                return Err(CopyError::InvalidFileName {
                    path: pb.to_path_buf(),
                });
            }
        }

        Ok(results)
    }

    fn min_verbosity(&self) -> Option<Verbosity> {
        Some(Verbosity::Low)
    }
}

#[cfg(test)]
mod tests {
    use super::CopyOperation;
    use crate::{cli::Verbosity, logger::Logger, proc::Run};
    use std::{fs::File, path::Path};

    const FROM_DIR: &'static str = "copy";

    #[test]
    fn copy_many() {
        let logger = Logger::new(true, Some(Verbosity::High));

        let from_dir = Path::new(crate::_TESTS_DIR)
            .join(FROM_DIR)
            .join("copy_many");
        let mut from_paths = vec![];

        let dest_dir = from_dir.join("out");
        let mut dest_paths = vec![];

        if let Err(e) = std::fs::create_dir_all(&dest_dir) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Made dir: {}", &dest_dir.display()),
        );

        for fin in ["test0.txt", "test1.txt", "test2.txt"] {
            let from_path = from_dir.join(fin);
            if let Err(e) = File::create(&from_path) {
                assert!(false, "{}", e)
            }
            logger.println(
                Some(Verbosity::High),
                &format!("Created file: {}", &from_path.display()),
            );

            from_paths.push(from_path);
            dest_paths.push(dest_dir.join(fin));
        }

        let mut cop = CopyOperation::new(&from_paths, &dest_dir, &logger);
        if let Err(e) = cop.run() {
            assert!(false, "{}", e)
        }

        dest_paths.append(&mut from_paths);
        for pb in dest_paths {
            if let Err(e) = std::fs::remove_file(&pb) {
                assert!(false, "{}", e)
            }
            logger.println(
                Some(Verbosity::High),
                &format!("Removed file: {}", &pb.display()),
            );
        }

        for d in [dest_dir, from_dir] {
            if let Err(e) = std::fs::remove_dir(&d) {
                assert!(false, "{}", e)
            }
            logger.println(
                Some(Verbosity::High),
                &format!("Removed dir: {}", &d.display()),
            );
        }
    }

    #[test]
    fn copy_only() {
        let logger = Logger::new(true, Some(Verbosity::High));

        let from_dir = Path::new(crate::_TESTS_DIR)
            .join(FROM_DIR)
            .join("copy_only");
        let dest_dir = from_dir.join("out");

        if let Err(e) = std::fs::create_dir_all(&dest_dir) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Made dir: {}", &dest_dir.display()),
        );

        let file_name = "test.txt";
        let from_path = from_dir.join(file_name);
        if let Err(e) = File::create(&from_path) {
            assert!(false, "{}", e)
        }
        logger.println(
            Some(Verbosity::High),
            &format!("Created file: {}", &from_path.display()),
        );

        let mut cop = CopyOperation::only(&from_path, &dest_dir, &logger);
        if let Err(e) = cop.run() {
            assert!(false, "{}", e)
        }

        for p in [from_path, dest_dir.join(file_name)] {
            if let Err(e) = std::fs::remove_file(&p) {
                assert!(false, "{}", e)
            }
            logger.println(
                Some(Verbosity::High),
                &format!("Removed file: {}", &p.display()),
            );
        }

        for d in [dest_dir, from_dir] {
            if let Err(e) = std::fs::remove_dir(&d) {
                assert!(false, "{}", e)
            }
            logger.println(
                Some(Verbosity::High),
                &format!("Removed dir: {}", &d.display()),
            );
        }
    }
}
