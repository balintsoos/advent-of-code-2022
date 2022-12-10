use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    println!("{}", solve());
}

#[test]
fn test() {
    assert_eq!(solve(), "DCVTCVPCL");
}

fn solve() -> String {
    let mut m: Vec<Vec<char>> = vec![
        vec!['T', 'P', 'Z', 'C', 'S', 'L', 'Q', 'N'],
        vec!['L', 'P', 'T', 'V', 'H', 'C', 'G'],
        vec!['D', 'C', 'Z', 'F'],
        vec!['G', 'W', 'T', 'D', 'L', 'M', 'V', 'C'],
        vec!['P', 'W', 'C'],
        vec!['P', 'F', 'J', 'D', 'C', 'T', 'S', 'Z'],
        vec!['V', 'W', 'G', 'B', 'D'],
        vec!['N', 'J', 'S', 'Q', 'H', 'W'],
        vec!['R', 'C', 'Q', 'F', 'S', 'L', 'V'],
    ];

    for line in read_lines() {
        if let Ok(chars) = line {
            let v: Vec<usize> = chars
                .split(" ")
                .map(|c| c.parse::<usize>().unwrap())
                .collect();
            let how_many = v[0];
            let from = v[1] - 1;
            let to = v[2] - 1;

            let mut stack: Vec<char> = Vec::new();
            for _ in 0..how_many {
                stack.push(m[from].pop().unwrap());
            }
            for _ in 0..how_many {
                m[to].push(stack.pop().unwrap());
            }
        }
    }

    let mut result = String::new();
    for mut v in m {
        result.push(v.pop().unwrap());
    }
    result
}

fn read_lines() -> io::Lines<io::BufReader<File>> {
    let file = File::open("inputs/day-05-steps.txt").expect("No such file");
    io::BufReader::new(file).lines()
}
