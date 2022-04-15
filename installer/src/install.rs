use clap::Subcommand;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum Distro {
    Fedora,
}

impl Default for Distro {
    fn default() -> Self {
        Self::Fedora
    }
}

impl TryFrom<String> for Distro {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().trim() {
            "fedora" => Ok(Self::Fedora),
            _ => Err("Only 'fedora' is currently supported."),
        }
    }
}

impl FromStr for Distro {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        Distro::try_from(s.to_string())
    }
}

impl From<Distro> for String {
    fn from(distro: Distro) -> Self {
        match distro {
            Distro::Fedora => String::from("fedora"),
        }
    }
}

#[derive(Debug, Subcommand)]
pub(crate) enum InstallCommand {
    Core,
    X11,
    Wayland,
    Flatpaks,
    Dots,
}

pub(crate) struct InstallOperation {
    cmd: InstallCommand,
    distro: Distro,
    results: Vec<Result<&'static str, &'static str>>,
}

#[cfg(test)]
mod tests {
    use super::Distro;

    #[test]
    fn distro_from_valid_string() {
        if let Err(e) = Distro::try_from(String::from("fedora")) {
            panic!("{}", e)
        }
    }

    #[test]
    fn distro_from_invalid_string() {
        Distro::try_from(String::from("some_rando_distro"))
            .expect_err("some_rando_distro is an invalid Distro, try_from should not succeed.");
    }
}