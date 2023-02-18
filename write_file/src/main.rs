use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("my_file.txt").expect("Could not create file!");

    file.write_all(b"Welcome to rust!")
        .expect("Cannot write to the file!")
}
