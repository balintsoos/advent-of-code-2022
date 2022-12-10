use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 1348005);
}

fn solve() -> u64 {
    let mut size_map: HashMap<String, u64> = HashMap::new();
    let mut path: Vec<String> = Vec::new();

    for line in read_lines() {
        if let Ok(chars) = line {
            let v: Vec<&str> = chars.split(" ").collect();
            if v.len() == 3 && v[0] == "$" && v[1] == "cd" && v[2] != ".." {
                path.push(v[2].to_string());
                continue;
            }
            if v.len() == 3 && v[0] == "$" && v[1] == "cd" && v[2] == ".." {
                path.pop();
                continue;
            }
            if v.len() == 2 && v[0] != "$" && v[0] != "dir" {
                let size = v[0].parse::<u64>().unwrap();
                for i in 0..path.len() {
                    let key = path[0..=i].join("/");
                    *size_map.entry(key).or_insert(0) += size;
                }
            }
        }
    }

    size_map.values().filter(|v| **v <= 100000).sum()
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-07.txt").expect("No such file");
    io::BufReader::new(file).lines()
}
