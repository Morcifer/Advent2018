use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn get_file_path(is_test: bool, day: u32, suffix: Option<String>) -> String {
    let sub_folder = if is_test { "test" } else { "real" };
    let suffix = if let Some(suffix) = suffix { format!("_{suffix}") } else { String::new() };
    format!("./data/{sub_folder}/day_{day}{suffix}.txt")
}

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect()
}