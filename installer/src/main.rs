use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    action: String,
}

enum Action {
    AddChanges,
}

fn main() {
    let args: Args = Args::parse();
}
