use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 907);
}

fn solve() -> u16 {
    let mut result: u16 = 0;

    for line in read_lines() {
        if let Ok(chars) = line {
            let vec: Vec<&str> = chars.split(",").collect();
            let vec1: Vec<&str> = vec[0].split("-").collect();
            let vec2: Vec<&str> = vec[1].split("-").collect();
            if is_overlap(vec1, vec2) {
                result += 1;
            }
        }
    }

    result
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-04.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn is_overlap(a: Vec<&str>, b: Vec<&str>) -> bool {
    to_u8(a[1]) >= to_u8(b[0]) && to_u8(a[0]) <= to_u8(b[1])
}

fn to_u8(s: &str) -> u8 {
    s.parse::<u8>().unwrap()
}
