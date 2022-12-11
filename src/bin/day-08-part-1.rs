use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 5);
}

fn solve() -> usize {
    let mut m: Vec<Vec<u32>> = Vec::new();

    for line in read_lines() {
        if let Ok(chars) = line {
            let row: Vec<u32> = chars
                .chars()
                .map(|c| c.to_digit(10).expect("Cannot parse number"))
                .collect();
            m.push(row);
        }
    }

    let mut result = 0;
    let column_length = m.len();
    for row_i in 1..column_length - 1 {
        for column_i in 1..m.len() - 1 {
            let item = m[row_i][column_i];

            let left_max = m[row_i][0..column_i].iter().max().expect("No max");
            if *left_max < item {
                result += 1;
                continue;
            }

            let right_max = m[row_i][column_i + 1..m[row_i].len()]
                .iter()
                .max()
                .expect("No max");
            if *right_max < item {
                result += 1;
                continue;
            }

            let top_max = m[0..row_i]
                .iter()
                .map(|row| row[column_i])
                .max()
                .expect("No max");
            if top_max < item {
                result += 1;
                continue;
            }

            let bottom_max = m[row_i + 1..column_length]
                .iter()
                .map(|row| row[column_i])
                .max()
                .expect("No max");
            if bottom_max < item {
                result += 1;
                continue;
            }
        }
    }

    result + column_length * 2 + (m[0].len() - 2) * 2
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-08.txt").expect("No such file");
    io::BufReader::new(file).lines()
}
