use crate::utils::CopyOperation;
use std::{
    fmt::Display,
    path::{Path, PathBuf},
    process::{Command, Output},
};

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

pub(crate) fn git_add(parent_path: &Path) -> Result<Output, AddError> {
    let output = if cfg!(target_os = "windows") {
        todo!()
    } else {
        Command::new("git")
            .args([
                "-C",
                parent_path
                    .join(".nedots")
                    .as_path()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
                "add",
                ".",
            ])
            .output()
            .expect("failed to execute process")
    };

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;
    use std::process::{Command, Output};

    fn git_stash(path: &str) -> Output {
        Command::new("git")
            .args(["-C", path, "stash"])
            .output()
            .expect(&format!("Failed to stash `git` tree at {}!", path))
    }

    fn git_restore(path: &str) -> Output {
        Command::new("git")
            .args(["-C", path, "stash", "pop"])
            .output()
            .expect(&format!(
                "Failed to restore `git` tree at {} to it's original state!",
                path
            ))
    }

    #[test]
    fn git_add() {
        let settings: Settings = crate::read_settings();
        let repo_path = settings.path.join(".nedots");

        git_stash(repo_path.to_string_lossy().to_string().as_str());

        let git_add = super::git_add(&settings.path);
        match git_add {
            Ok(o) => assert!(o.status.success()),
            Err(e) => panic!("{}", e),
        }

        git_restore(repo_path.to_string_lossy().to_string().as_str());
    }
}
