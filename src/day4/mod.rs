use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Part1Solution {
    pub input: String,
    pub output: usize,
}

impl Part1Solution {
    pub fn new(input: String) -> Part1Solution {
        Part1Solution {
            input: input,
            output: 0,
        }
    }
    pub fn solution(self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        let mut total = 0;
        for line in reader.lines() {
            let lines: [usize; 4] = line
                .unwrap()
                .trim()
                .split(",")
                .map(|s| s.split("-"))
                .flatten()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            let [x1, x2, y1, y2] = lines;
            // only two conditions to check
            // x contains y
            if x1 <= y1 && y2 <= x2 {
                total += 1;
            }
            // y contains x
            else if y1 <= x1 && x2 <= y2 {
                total += 1;
            }
        }

        Part1Solution {
            input: self.input,
            output: total,
        }
    }
}

pub struct Part2Solution {
    pub input: String,
    pub output: usize,
}

impl Part2Solution {
    pub fn new(input: String) -> Part2Solution {
        Part2Solution {
            input: input,
            output: 0,
        }
    }
    pub fn solution(self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        let mut total = 0;
        for line in reader.lines() {
            let lines: [usize; 4] = line
                .unwrap()
                .trim()
                .split(",")
                .map(|s| s.split("-"))
                .flatten()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap();
            let [x1, x2, y1, y2] = lines;
            // TODO: find more elegant solution
            // checks all overlapping conditions
            // x contains y
            if x1 <= y1 && y2 <= x2 {
                total += 1;
            }
            // y contains x
            else if y1 <= x1 && x2 <= y2 {
                total += 1;
            }
            // x is "left" of y
            else if x1 <= y1 && y1 <= x2 && x2 <= y2 {
                total += 1;
            }
            // y is "left" of x
            else if y1 <= x1 && x1 <= y2 && y2 <= x2 {
                total += 1;
            }
        }

        Part2Solution {
            input: self.input,
            output: total,
        }
    }
}
