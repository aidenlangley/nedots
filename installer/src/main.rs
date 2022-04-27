mod cli;
mod config;
mod ops;
mod output;

fn main() -> Result<(), std::io::Error> {
    cli::run()
}

pub const _TESTS_DIR: &'static str = "tests";
