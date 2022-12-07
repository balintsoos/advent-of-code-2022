use std::io::{BufRead, BufReader};
use std::fs::File;

fn lines_from_file() -> Vec<String> {
    let file = File::open("input.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn main() {
    let lines = lines_from_file();

    let mut max: u32 = 0;
    let mut current: u32 = 0;
    for line in lines {
        if line.is_empty() {
            if current > max {
                max = current;
            }
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    println!("Max: {}", max);
}
