use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 8018)
}

fn solve() -> u16 {
    let mut result: u16 = 0;

    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(chars) = line {
                let (left, right) = chars.split_at(chars.len() / 2);
                if let Some(char) = shared_char(left, right) {
                    result += get_char_as_integer(char);
                }
            }
        }
    }

    result
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("inputs/day-03.txt")?;
    Ok(io::BufReader::new(file).lines())
}

fn shared_char(left: &str, right: &str) -> Option<char> {
    let set: HashSet<char> = left.chars().collect();
    right.chars().find(|c| set.contains(&c))
}

fn get_char_as_integer(char: char) -> u16 {
    let char_as_u16 = char as u16;
    if char_as_u16 > 90 {
        return char_as_u16 - 96;
    }
    char_as_u16 - 38
}
