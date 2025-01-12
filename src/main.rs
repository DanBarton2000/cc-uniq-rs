use std::path::PathBuf;
use clap::Parser;
use cc_uniq_rs::{build_reader, build_writer, uniq};

fn main() {
    let args = Args::parse();
    let reader = build_reader(args.input_path)
        .expect("Failed to create reader");
    let writer = build_writer(args.output_path)
        .expect("Failed to create writer");
    uniq(reader, writer, args.count).unwrap();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
    #[clap(long, short, action)]
    count: bool,
}