use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day3 input-filename");

    let contents = fs::read_to_string(filename).expect("Failed to read file");
    println!("Input contains {} lines", contents.lines().count());
}
