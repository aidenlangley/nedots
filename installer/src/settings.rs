use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Settings {
    pub(crate) path: PathBuf,
    pub(crate) root: Vec<PathBuf>,
    pub(crate) user: Vec<PathBuf>,
    #[serde(rename = "packages")]
    pub(crate) pkgs: Packages,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, &'static str> {
        let settings_file: fs::File;
        match fs::File::open("nedots.json") {
            Ok(f) => settings_file = f,
            Err(_) => return Err("Could not open file, file does not exist."),
        }

        let settings: Self;
        match serde_json::from_reader(settings_file) {
            Ok(s) => settings = s,
            Err(_) => return Err("Failed to serialize settings file."),
        }

        Ok(settings)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "core")]
pub(crate) struct CorePackages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "x11")]
pub(crate) struct X11Packages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "wayland")]
pub(crate) struct WaylandPackages {
    #[serde(rename = "fedora")]
    pub(crate) fedora_pkgs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "flatpak")]
pub(crate) struct FlatpakRemote {
    pub(crate) remote: String,
    pub(crate) url: String,
    #[serde(rename = "packages")]
    pub(crate) pkgs: Vec<String>,
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
            panic!("{}", e);
        }
    }

    #[test]
    fn serialize() {
        match deserialize_test_data() {
            Ok(s) => {
                if let Err(e) = serde_json::to_string::<Settings>(&s) {
                    panic!("{}", e);
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
