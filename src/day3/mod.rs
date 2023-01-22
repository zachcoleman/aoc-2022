use std::collections::HashMap;
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

        // map of letters to ascii values
        let mut map = HashMap::new();
        for (p, c) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
            map.insert(c, p + 1);
        }
        for (p, c) in "abcdefghijklmnopqrstuvwxyz"
            .to_uppercase()
            .chars()
            .enumerate()
        {
            map.insert(c, p + 27);
        }

        // array to track priorities of each letter
        let mut total: usize = 0;

        for line in reader.lines() {
            let mut tracker = [0; 52];
            match &line {
                Ok(line) => {
                    for (ix, c) in line.chars().enumerate() {
                        // catalog unique items in first compartment (add to tracker)
                        if ix < line.len() / 2 {
                            if c.is_lowercase() {
                                tracker[(c as u32 - 97) as usize] = 1;
                            } else {
                                tracker[(c as u32 - 65 + 26) as usize] = 1;
                            }
                        }
                        // catalog second compartment (if any non-zero values, add to total)
                        else {
                            // a -> 97 and A -> 65
                            // a..z on tracker is 0..25
                            // A..Z on tracker is 26..51
                            if c.is_lowercase() {
                                if tracker[(c as u32 - 97) as usize] > 0 {
                                    total += (c as u32 - 96) as usize;
                                    break;
                                }
                            } else {
                                if tracker[(c as u32 - 65 + 26) as usize] > 0 {
                                    total += (c as u32 - 65 + 27) as usize;
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
        return Part1Solution {
            input: self.input,
            output: total,
        };
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

        // map of letters to ascii values
        let mut map = HashMap::new();
        for (p, c) in "abcdefghijklmnopqrstuvwxyz".chars().enumerate() {
            map.insert(c, p + 1);
        }
        for (p, c) in "abcdefghijklmnopqrstuvwxyz"
            .to_uppercase()
            .chars()
            .enumerate()
        {
            map.insert(c, p + 27);
        }

        // array to track priorities of each letter
        let mut tracker = [0; 52];
        let mut total: usize = 0;

        for (ix, line) in reader.lines().enumerate() {
            match &line {
                Ok(line) => {
                    // process by group
                    if ix % 3 == 0 {
                        // before resetting tracker, add to total
                        for (ix, val) in tracker.iter().enumerate() {
                            if *val == 3 {
                                total += ix + 1;
                            }
                        }
                        // reset tracker
                        tracker = [0; 52];
                    }
                    for c in line.chars() {
                        if c.is_lowercase() {
                            if tracker[(c as u32 - 97) as usize] == (ix % 3) {
                                tracker[(c as u32 - 97) as usize] += 1;
                            }
                        } else {
                            if tracker[(c as u32 - 65 + 26) as usize] == (ix % 3) {
                                tracker[(c as u32 - 65 + 26) as usize] += 1;
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }

        // add last group
        for (ix, val) in tracker.iter().enumerate() {
            if *val == 3 {
                total += ix + 1;
            }
        }

        Part2Solution {
            input: self.input,
            output: total,
        }
    }
}
