use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let mut reader = build_reader(args.input_path)
        .expect("Failed to create reader.");

    let mut line = String::new();
    let mut last_hash = None;

    while reader.read_line(&mut line).unwrap() > 0 {
        let current_hash = calculate_hash(&line);
        if let Some(last_hash) = last_hash {
            if last_hash != current_hash {
                print!("{line}");
            }
        } else {
            print!("{line}")
        }

        last_hash = Some(current_hash);
        line.clear();
    }
}

fn build_reader(path: Option<PathBuf>) -> std::io::Result<Box<dyn BufRead>> {
    if let Some(path) = path {
        if path.to_str() == Some("-") {
            return Ok(Box::new(BufReader::new(stdin())));
        }
        File::open(&path)
            .map(|file| Box::new(BufReader::new(file)) as Box<dyn BufRead>)
    } else {
        Ok(Box::new(BufReader::new(stdin())))
    }
}

// Taken from https://doc.rust-lang.org/std/hash/index.html
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    input_path: Option<PathBuf>
}