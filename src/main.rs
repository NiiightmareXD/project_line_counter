use std::fs::{self, File};
use std::io::{prelude::*, BufReader};

use crossterm::style::Stylize;
use std::env;
use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();

    let mut count_lines = 0u64;
    let file_paths = get_all_files(current_dir);

    for file_path in file_paths {
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);
        for _ in reader.lines() {
            count_lines += 1;
        }
    }

    println!("found {} lines", count_lines.to_string().bold().magenta());
}

fn get_all_files(dir: PathBuf) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    let paths = fs::read_dir(dir).unwrap();
    for path in paths {
        if !path.as_ref().unwrap().path().is_dir() {
            file_paths.push(path.as_ref().unwrap().path());
        } else {
            let files = get_all_files(path.as_ref().unwrap().path());
            for file in files {
                file_paths.push(file);
            }
        }
    }

    file_paths
}
