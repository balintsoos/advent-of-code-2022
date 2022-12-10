use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 1965);
}

fn solve() -> usize {
    let mut set: HashSet<u8> = HashSet::new();

    for line in read_lines() {
        if let Ok(chars) = line {
            let bytes = chars.as_bytes();
            for i in 3..bytes.len() {
                for j in i - 3..=i {
                    if set.contains(&bytes[j]) {
                        break;
                    }
                    if j == i {
                        return i + 1;
                    }
                    set.insert(bytes[j]);
                }
                set.clear();
            }
        }
    }
    0
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-06.txt").unwrap();
    io::BufReader::new(file).lines()
}
