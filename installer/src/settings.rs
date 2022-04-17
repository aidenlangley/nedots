use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, path::PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub path: PathBuf,
    pub root: Vec<PathBuf>,
    pub user: Vec<PathBuf>,
    #[serde(rename = "packages")]
    pub pkgs: Packages,
}

#[derive(Debug)]
pub enum SettingsError {
    FileOpenError(std::io::Error),
    DeserializeError(String),
}

impl Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::FileOpenError(e) => write!(f, "{}", e),
            SettingsError::DeserializeError(s) => write!(
                f,
                "Failed to deserialize `nedots.json` into `Settings` struct: {}",
                s
            ),
        }
    }
}

impl Settings {
    pub(super) fn read() -> Result<Self, SettingsError> {
        let settings_file = match File::open("nedots.json") {
            Ok(f) => f,
            Err(e) => return Err(SettingsError::FileOpenError(e)),
        };

        match serde_json::from_reader(settings_file) {
            Ok(s) => Ok(s),
            Err(e) => Err(SettingsError::DeserializeError(e.to_string())),
        }
    }

    pub(super) fn _write(&self) -> std::io::Result<Self> {
        todo!() // Serialize struct to JSON -> write JSON to file.
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Packages {
    #[serde(rename = "core")]
    pub core_pkgs: CorePackages,
    #[serde(rename = "x11")]
    pub x11_pkgs: X11Packages,
    #[serde(rename = "wayland")]
    pub wayland_pkgs: WaylandPackages,
    #[serde(rename = "flatpak")]
    pub flatpaks: Vec<FlatpakRemote>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "core")]
pub struct CorePackages {
    #[serde(rename = "fedora")]
    pub fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "x11")]
pub struct X11Packages {
    #[serde(rename = "fedora")]
    pub fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "wayland")]
pub struct WaylandPackages {
    #[serde(rename = "fedora")]
    pub fedora_pkgs: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "flatpak")]
pub struct FlatpakRemote {
    pub remote: String,
    pub url: String,
    #[serde(rename = "packages")]
    pub pkgs: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::Settings;
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

    fn deserialize_test_data() -> Result<Settings, serde_json::Error> {
        serde_json::from_value::<Settings>(make_test_data())
    }

    #[test]
    fn deserialize() {
        if let Err(e) = deserialize_test_data() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn serialize() {
        match deserialize_test_data() {
            Ok(s) => {
                if let Err(e) = serde_json::to_string::<Settings>(&s) {
                    assert!(false, "{}", e)
                }
            }
            Err(e) => assert!(false, "{}", e),
        }
    }
}
