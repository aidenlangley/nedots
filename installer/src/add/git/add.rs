use super::GitError;
use std::{
    path::Path,
    process::{Command, Output},
};

pub(super) fn add(dest: &Path) -> Result<Output, GitError> {
    let output = Command::new("git")
        .args([
            "-C",
            dest.to_string_lossy().to_string().as_str(),
            "add",
            ".",
        ])
        .output()
        .expect("Failed to add latest changes to `git` tree.");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use crate::settings::Settings;
    use std::{
        io::{stdout, Write},
        process::Command,
    };

    fn git_stash(path: &str) {
        let output = Command::new("git")
            .args(["-C", path, "stash", "push"])
            .output();
        match output {
            Ok(o) => {
                stdout()
                    .write_all(&o.stdout)
                    .expect("Failed to write stdout!");
                stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr!");
            }
            Err(_) => println!("Failed to stash `git` tree at {}!", path),
        }
    }

    fn git_restore(path: &str) {
        let output = Command::new("git")
            .args(["-C", path, "stash", "pop"])
            .output();

        match output {
            Ok(o) => {
                stdout()
                    .write_all(&o.stdout)
                    .expect("Failed to write stdout!");
                stdout()
                    .write_all(&o.stderr)
                    .expect("Failed to write stderr!");
            }
            Err(_) => println!(
                "Failed to restore `git` tree at {} to it's original state!",
                path
            ),
        }
    }

    #[test]
    fn git_add() {
        let settings: Settings = crate::read_settings();
        git_stash(settings.path.to_string_lossy().to_string().as_str());

        let git_add = super::add(&settings.path);
        match git_add {
            Ok(o) => assert!(o.status.success()),
            Err(e) => panic!("{}", e),
        }

        git_restore(settings.path.to_string_lossy().to_string().as_str());
    }
}
