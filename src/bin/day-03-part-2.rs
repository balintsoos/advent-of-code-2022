use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}

fn shared_char(first: &String, second: &String, third: &String) -> Option<char> {
    let first_set: HashSet<char> = first.chars().collect();
    let second_set: HashSet<char> = second.chars().collect();
    third.chars().find(|c| first_set.contains(&c) && second_set.contains(&c))
}

fn get_char_as_integer(char: char) -> u16 {
    let char_as_u16 = char as u16;
    if char_as_u16 > 90 {
        return char_as_u16 - 96;
    }
    char_as_u16 - 38
}

fn main() {
    let mut result: u16 = 0;
    let mut vec: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines() {
        for line in lines {
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
    }

    println!("Result: {}", result);
}
