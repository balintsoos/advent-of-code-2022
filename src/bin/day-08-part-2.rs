use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), 385112);
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

    let mut max = 0;
    let column_length = m.len();
    for row_i in 1..column_length - 1 {
        for column_i in 1..m.len() - 1 {
            let current = m[row_i][column_i];

            let left_iter = m[row_i][0..column_i].iter().rev();
            let mut left = 0;
            for item in left_iter {
                left += 1;
                if *item >= current {
                    break;
                }
            }

            let right_iter = m[row_i][column_i + 1..m[row_i].len()].iter();
            let mut right = 0;
            for item in right_iter {
                right += 1;
                if *item >= current {
                    break;
                }
            }

            let top_iter = m[0..row_i].iter().rev().map(|row| row[column_i]);
            let mut top = 0;
            for item in top_iter {
                top += 1;
                if item >= current {
                    break;
                }
            }

            let bottom_iter = m[row_i + 1..column_length].iter().map(|row| row[column_i]);
            let mut bottom = 0;
            for item in bottom_iter {
                bottom += 1;
                if item >= current {
                    break;
                }
            }

            let count = left * right * top * bottom;
            if count > max {
                max = count;
            }
        }
    }

    max
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-08.txt").expect("No such file");
    io::BufReader::new(file).lines()
}
