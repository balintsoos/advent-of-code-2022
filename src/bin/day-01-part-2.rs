use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 195292)
}

fn solve() -> u32 {
    let mut calories: Vec<u32> = Vec::new();
    let mut current: u32 = 0;

    for line in read_lines() {
        if let Ok(calorie) = line {
            if calorie.is_empty() {
                calories.push(current);
                current = 0;
            } else {
                current += calorie.parse::<u32>().unwrap();
            }
        }
    }

    calories.sort_by(|a, b| b.cmp(a));
    calories[0] + calories[1] + calories[2]
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-01.txt").unwrap();
    io::BufReader::new(file).lines()
}
