use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 66306)
}

fn solve() -> u32 {
    let mut max: u32 = 0;
    let mut current: u32 = 0;

    for line in read_lines() {
        if let Ok(chars) = line {
            if chars.is_empty() {
                if current > max {
                    max = current;
                }
                current = 0;
            } else {
                current += chars.parse::<u32>().unwrap();
            }
        }
    }

    max
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-01.txt").expect("No such file");
    io::BufReader::new(file).lines()
}
