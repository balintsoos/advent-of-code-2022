use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 66306)
}

fn solve() -> u32 {
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

    max
}

fn lines_from_file() -> Vec<String> {
    let file = File::open("inputs/day-01.txt").expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
