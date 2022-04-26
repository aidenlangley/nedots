use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    path::{Path, PathBuf},
};
use thiserror::Error;

#[derive(Debug, Error)]
/// Errors thrown during `Config` creation, involving fs operations,
/// serialization and path resolution.
pub(crate) enum ConfigError {
    #[error("Failed to deserialize `nedots.json`, is it badly formatted?")]
    /// Serde error.
    DeserializeError,

    #[error("Could not resolve path: {path:?}")]
    /// Failed to resolve `nedots` path in settings, which is required for
    /// nedots operation.
    BadPath { path: PathBuf },

    #[error("Could not resolve paths: {paths:#?}")]
    /// Failed to resolve paths in settings, required for managing dotfiles.
    BadPaths { paths: Vec<PathBuf> },

    #[error(transparent)]
    /// A wrapper around IO errors.
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    /// The location of `nedots` directory.
    pub(crate) path: PathBuf,

    /// Paths owned by root.
    pub(crate) root: Vec<PathBuf>,

    /// Paths owned by user. $HOME is prepended to these paths during discovery.
    pub(crate) user: Vec<PathBuf>,

    #[serde(rename = "packages")]
    /// Packages in categories.
    pub(crate) pkgs: Packages,
}

impl Config {
    /// Create a new `Config` and resolve paths.
    pub(crate) fn new() -> Result<Self, ConfigError> {
        Self::read(None)?
            .resolve_path()?
            .resolve_root_paths()?
            .resolve_user_paths()
    }

    /// Read & deserialize `nedots.json`.
    ///
    /// ### Errors
    /// Returns `std::io::Error` when the file does not exist, or
    /// `SettingsError::DeserializeError` if `serde` fails to deserialize.
    pub(crate) fn read(path: Option<&Path>) -> Result<Self, ConfigError> {
        let path = path.or(Some(Path::new("nedots.json"))).unwrap();
        serde_json::from_reader::<File, Config>(File::open(path)?)
            .or(Err(ConfigError::DeserializeError))
    }

    /// Resolve `settings.path`, if it doesn't exist, prepend $HOME.
    ///
    /// ### Errors
    /// Returns `SettingsError::BadPath` if `canonicalize` fails.
    pub(crate) fn resolve_path(mut self) -> Result<Self, ConfigError> {
        if !self.path.exists() {
            self.path = match Path::new(env!("HOME")).join(&self.path).canonicalize() {
                Ok(pb) => pb,
                Err(_) => return Err(ConfigError::BadPath { path: self.path }),
            };
        }

        Ok(self)
    }

    /// Resolve paths in `settings.root`.
    ///
    /// ### Errors
    /// Returns `SettingsError::BadPaths` containing all paths that do not
    /// exist.
    pub(crate) fn resolve_root_paths(self) -> Result<Self, ConfigError> {
        let mut bad_paths = Vec::new();
        for pb in &self.root {
            if !pb.exists() {
                bad_paths.push(pb.to_owned());
            }
        }

        if bad_paths.len() > 0 {
            return Err(ConfigError::BadPaths { paths: bad_paths });
        }

        Ok(self)
    }

    /// Resolve paths in `settings.user` by prepending `settings.path`.
    ///
    /// ### Errors
    /// Returns `SettingsError::BadPaths` containing all paths that fail
    /// `canonicalize`.
    pub(crate) fn resolve_user_paths(mut self) -> Result<Self, ConfigError> {
        let mut bad_paths = Vec::new();
        self.user = self
            .user
            .into_iter()
            .map(
                |pb| match Path::new(env!("HOME")).join(&pb).canonicalize() {
                    Ok(pb) => pb,
                    Err(_) => {
                        bad_paths.push(pb.to_owned());
                        pb
                    }
                },
            )
            .collect();

        if bad_paths.len() > 0 {
            return Err(ConfigError::BadPaths { paths: bad_paths });
        }

        Ok(self)
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(crate) struct Packages {
    #[serde(rename = "core")]
    pub(crate) core_pkgs: CorePackages,
    #[serde(rename = "x11")]
    pub(crate) x11_pkgs: X11Packages,
    #[serde(rename = "wayland")]
    pub(crate) wayland_pkgs: WaylandPackages,
    #[serde(rename = "flatpak")]
    pub(crate) flatpaks: Vec<FlatpakRemote>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "core")]
pub(crate) struct CorePackages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "x11")]
pub(crate) struct X11Packages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "wayland")]
pub(crate) struct WaylandPackages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename = "flatpak")]
pub(crate) struct FlatpakRemote {
    pub(crate) remote: String,
    pub(crate) url: String,
    #[serde(rename = "packages")]
    pub(crate) pkgs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::Config;
    use crate::_TESTS_DIR;
    use serde_json::{json, Value};

    fn make_test_data() -> Value {
        json!({
            "path": "/a/fake/path",
            "root": [
                "/a/fake/root/path",
                "/another/fake/root/path"
            ],
            "user": [
                "a/fake/home/path",
                "another/fake/home/path",
                "notice/there/is/no/prefix"
            ],
            "packages": json!({
                "core": json!({
                    "fedora": [
                        "core_pkg0",
                        "core_pkg1",
                        "core_pkg2",
                        "core_pkg3",
                    ]
                }),
                "x11": json!({
                    "fedora": [
                        "x11_pkg0",
                        "x11_pkg1",
                        "x11_pkg2",
                        "x11_pkg3",
                    ]
                }),
                "wayland": json!({
                    "fedora": [
                        "wayland_pkg0",
                        "wayland_pkg1",
                        "wayland_pkg2",
                        "wayland_pkg3",
                    ]
                }),
                "flatpak": [
                    json!({
                        "remote": "remote_name",
                        "url": "https://some.fake.url/",
                        "packages": [
                            "flatpak_pkg0",
                            "flatpak_pkg1",
                            "flatpak_pkg2",
                            "flatpak_pkg3"
                        ]
                    })
                ],
            })
        })
    }

    fn deserialize_test_data() -> Result<Config, serde_json::Error> {
        serde_json::from_value::<Config>(make_test_data())
    }

    #[test]
    /// Simple deserialization test.
    fn deserialize() {
        if let Err(e) = deserialize_test_data() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    /// Simple serialization test.
    fn serialize() {
        match deserialize_test_data() {
            Ok(s) => {
                if let Err(e) = serde_json::to_string::<Config>(&s) {
                    assert!(false, "{}", e)
                }
            }
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    /// Expects that `path` found in `nedots.test.json` will resolve. Also
    /// expects that the file will deserialize into a valid `Config`.
    fn path_resolve_err() {
        let path = Path::new(_TESTS_DIR).join("nedots.test.json");
        match Config::read(Some(&path)) {
            Ok(c) => {
                if let Err(e) = c.resolve_path() {
                    assert_eq!(
                        e.to_string(),
                        format!("Could not resolve path: \"/not_here/.nedots\"")
                    )
                }
            }
            Err(e) => assert!(false, "{}", e),
        }
    }

    #[test]
    /// Expects paths in `user` to not resolve. Also expects `nedots.test.json`
    /// to deserialize.
    fn user_paths_resolve_err() {
        let path = Path::new(_TESTS_DIR).join("nedots.test.json");
        match Config::read(Some(&path)) {
            Ok(c) => {
                if let Err(e) = c.resolve_user_paths() {
                    assert_eq!(
                        e.to_string(),
                        format!("Could not resolve paths: [\n    \"/not_here/.nedots\",\n    \"/not_here/.nedots/installer/tests\",\n]")
                    )
                }
            }
            Err(e) => assert!(false, "{}", e),
        }
    }
}
