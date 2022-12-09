use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 12156)
}

fn solve() -> u16 {
    let mut result: u16 = 0;

    for line in read_lines() {
        if let Ok(chars) = line {
            let vec: Vec<&str> = chars.split(" ").collect();
            result += get_point(vec[0], vec[1]);
        }
    }

    result
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-02.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn get_point(opponent_pick: &str, your_pick: &str) -> u16 {
    match (opponent_pick, your_pick) {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,
        (&_, _) => todo!(),
    }
}
