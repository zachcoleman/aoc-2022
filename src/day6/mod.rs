use std::fs::File;
use std::io::{BufReader, Read};

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

        let mut idx = 0;
        let (mut m, mut n) = (0, 0);
        let mut tracker = [None; 26];

        // will always keep bytes between [m, n] unique
        for (ix, c) in reader.bytes().enumerate() {
            let c_ix = c.unwrap() as usize - 97;
            if m + 3 <= n {
                idx = n;
                break;
            }
            if let Some(y) = tracker[c_ix] {
                if m <= y {
                    m = y + 1;
                }
            }
            n = ix;
            tracker[c_ix] = Some(ix);
        }

        Part1Solution {
            input: self.input,
            output: idx + 1,
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
        let mut idx = 0;
        let (mut m, mut n) = (0, 0);
        let mut tracker = [None; 26];

        // will always keep bytes between [m, n] unique
        for (ix, c) in reader.bytes().enumerate() {
            let c_ix = c.unwrap() as usize - 97;
            if m + 13 <= n {
                idx = n;
                break;
            }
            if let Some(y) = tracker[c_ix] {
                if m <= y {
                    m = y + 1;
                }
            }
            n = ix;
            tracker[c_ix] = Some(ix);
        }

        Part2Solution {
            input: self.input,
            output: idx + 1,
        }
    }
}
