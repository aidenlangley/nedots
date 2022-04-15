pub(crate) fn check_git() {}
pub(crate) fn check_flatpak() {}
pub(crate) fn check_all() {}

#[cfg(test)]
mod tests {
    use crate::{read_settings, settings::Settings};
    use std::process::Command;

    #[test]
    fn git_status() {
        let settings: Settings = read_settings();
        let git_status = Command::new("git")
            .args([
                "-C",
                settings
                    .path
                    .join(".nedots")
                    .as_path()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
                "status",
            ])
            .output();

        match git_status {
            Ok(o) => assert!(o.status.success()),
            Err(e) => panic!("{}", e),
        }
    }
}
