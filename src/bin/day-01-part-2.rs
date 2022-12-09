use std::fs::File;
use std::io::{self, BufRead};

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open("input.txt")?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut calories: Vec<u32> = Vec::new();
    let mut current: u32 = 0;

    if let Ok(lines) = read_lines() {
        for line in lines {
            if let Ok(calorie) = line {
                if calorie.is_empty() {
                    calories.push(current);
                    current = 0;
                } else {
                    current += calorie.parse::<u32>().unwrap();
                }
            }
        }
    }

    calories.sort_by(|a, b| b.cmp(a));
    println!(
        "Max 3 in total: {}",
        calories[0] + calories[1] + calories[2]
    );
}
