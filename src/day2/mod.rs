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
    pub fn solution(&mut self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let mut score = 0;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let round = line.split(" ").into_iter().collect::<Vec<&str>>();
                    // round result points
                    match round.as_slice() {
                        // wins
                        ["A", "Y"] | ["B", "Z"] | ["C", "X"] => {
                            score += 6;
                        }
                        // draws
                        ["A", "X"] | ["B", "Y"] | ["C", "Z"] => {
                            score += 3;
                        }
                        //losses
                        ["A", "Z"] | ["B", "X"] | ["C", "Y"] => {}
                        // ahhhhhh
                        _ => {
                            panic!("Invalid input");
                        }
                    }
                    // round shape points
                    match round.as_slice() {
                        // rock
                        [_, "X"] => {
                            score += 1;
                        }
                        [_, "Y"] => {
                            score += 2;
                        }
                        [_, "Z"] => {
                            score += 3;
                        }
                        _ => {
                            panic!("Invalid input")
                        }
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }

        return Part1Solution {
            input: self.input.clone(),
            output: score,
        };
    }
}
