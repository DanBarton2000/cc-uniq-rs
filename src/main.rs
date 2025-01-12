use std::io::stdout;
use std::path::PathBuf;
use clap::Parser;
use cc_uniq_rs::{build_reader, uniq};

fn main() {
    let args = Args::parse();
    let reader = build_reader(args.input_path)
        .expect("Failed to create reader");
    uniq(reader, stdout(), args.count).unwrap();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input_path: Option<PathBuf>,
    #[clap(long, short, action)]
    count: bool,
}