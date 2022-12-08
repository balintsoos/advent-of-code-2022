use std::io::{self, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input.txt")?;
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

fn main() {
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

    println!("Result: {}", result);
}
