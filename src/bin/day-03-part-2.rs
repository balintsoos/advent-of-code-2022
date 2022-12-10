use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 2518)
}

fn solve() -> u16 {
    let mut result: u16 = 0;
    let mut vec: Vec<String> = Vec::new();

    for line in read_lines() {
        if let Ok(chars) = line {
            vec.push(chars);
            if vec.len() == 3 {
                if let Some(char) = shared_char(&vec[0], &vec[1], &vec[2]) {
                    result += get_char_as_integer(char);
                }
                vec.clear();
            }
        }
    }

    result
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-03.txt").expect("No such file");
    io::BufReader::new(file).lines()
}

fn shared_char(first: &String, second: &String, third: &String) -> Option<char> {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();
    third
        .chars()
        .find(|c| first_set.contains(&c) && second_set.contains(&c))
}

fn get_char_as_integer(char: char) -> u16 {
    let char_as_u16 = char as u16;
    if char_as_u16 > 90 {
        return char_as_u16 - 96;
    }
    char_as_u16 - 38
}
