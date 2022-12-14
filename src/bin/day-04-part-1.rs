use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 567);
}

fn solve() -> u16 {
    let mut result: u16 = 0;

    for line in read_lines() {
        if let Ok(chars) = line {
            let vec: Vec<&str> = chars.split(",").collect();
            let vec1: Vec<&str> = vec[0].split("-").collect();
            let vec2: Vec<&str> = vec[1].split("-").collect();
            if is_fully_contain(vec1, vec2) {
                result += 1;
            }
        }
    }

    result
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-04.txt").expect("No such file");
    io::BufReader::new(file).lines()
}

fn is_fully_contain(assignment1: Vec<&str>, assignment2: Vec<&str>) -> bool {
    assignment1[0].parse::<u8>().unwrap() <= assignment2[0].parse::<u8>().unwrap()
        && assignment1[1].parse::<u8>().unwrap() >= assignment2[1].parse::<u8>().unwrap()
        || assignment1[0].parse::<u8>().unwrap() >= assignment2[0].parse::<u8>().unwrap()
            && assignment1[1].parse::<u8>().unwrap() <= assignment2[1].parse::<u8>().unwrap()
}
