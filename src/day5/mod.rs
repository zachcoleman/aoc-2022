use std::fs::File;
use std::io::{BufRead, BufReader};

struct Stack {
    stack: Vec<char>,
}

pub struct Part1Solution {
    pub input: String,
    pub output: String,
}

impl Part1Solution {
    pub fn new(input: String) -> Part1Solution {
        Part1Solution {
            input: input,
            output: String::new(),
        }
    }
    pub fn solution(self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // vec of stacks
        let mut stacks = Vec::<Stack>::new();
        let mut skip_lines = 0;

        // populate stacks w/ initial state
        for (ix, line) in reader.lines().enumerate() {
            match line.unwrap().as_str() {
                "" => {
                    skip_lines = ix;
                    break;
                }
                line => {
                    for (stack_idx, line_idx) in (1..line.len()).filter(|i| i % 4 == 1).enumerate()
                    {
                        if stack_idx >= stacks.len() {
                            stacks.push(Stack { stack: Vec::new() });
                        }
                        if line.chars().nth(line_idx).unwrap().is_alphabetic() {
                            let c = line.chars().nth(line_idx).unwrap();
                            if stacks[stack_idx].stack.len() == 0 {
                                stacks[stack_idx].stack.push(c);
                            } else {
                                stacks[stack_idx].stack.insert(0, c);
                            }
                        }
                    }
                }
            }
        }

        // iterate over moves
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        for (ix, line) in reader.lines().enumerate() {
            if ix <= skip_lines {
                continue;
            }
            let line = line.unwrap().clone();
            let vals: [&str; 6] = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let [_, amt, _, send, _, dest] = vals;
            let (amt, source, dest) = (
                amt.parse::<usize>().unwrap(),
                send.parse::<usize>().unwrap() - 1,
                dest.parse::<usize>().unwrap() - 1,
            );
            for _ in 0..amt {
                let c = stacks[source].stack.pop().unwrap();
                stacks[dest].stack.push(c);
            }
        }

        Part1Solution {
            input: self.input,
            output: stacks
                .iter()
                .map(|s| s.stack[s.stack.len() - 1])
                .collect::<String>(),
        }
    }
}

pub struct Part2Solution {
    pub input: String,
    pub output: String,
}

impl Part2Solution {
    pub fn new(input: String) -> Part2Solution {
        Part2Solution {
            input: input,
            output: String::new(),
        }
    }
    pub fn solution(self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // vec of stacks
        let mut stacks = Vec::<Stack>::new();
        let mut skip_lines = 0;

        // populate stacks w/ initial state
        for (ix, line) in reader.lines().enumerate() {
            match line.unwrap().as_str() {
                "" => {
                    skip_lines = ix;
                    break;
                }
                line => {
                    for (stack_idx, line_idx) in (1..line.len()).filter(|i| i % 4 == 1).enumerate()
                    {
                        if stack_idx >= stacks.len() {
                            stacks.push(Stack { stack: Vec::new() });
                        }
                        if line.chars().nth(line_idx).unwrap().is_alphabetic() {
                            let c = line.chars().nth(line_idx).unwrap();
                            if stacks[stack_idx].stack.len() == 0 {
                                stacks[stack_idx].stack.push(c);
                            } else {
                                stacks[stack_idx].stack.insert(0, c);
                            }
                        }
                    }
                }
            }
        }

        // iterate over moves
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        for (ix, line) in reader.lines().enumerate() {
            if ix <= skip_lines {
                continue;
            }
            let line = line.unwrap().clone();
            let vals: [&str; 6] = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let [_, amt, _, send, _, dest] = vals;
            let (amt, source, dest) = (
                amt.parse::<usize>().unwrap(),
                send.parse::<usize>().unwrap() - 1,
                dest.parse::<usize>().unwrap() - 1,
            );

            // to maintain order this time we must use an intermediate stack
            // classic two stack => fifo queue
            let mut intermediate = Vec::new();
            for _ in 0..amt {
                let c = stacks[source].stack.pop().unwrap();
                intermediate.push(c);
            }
            for _ in 0..amt {
                let c = intermediate.pop().unwrap();
                stacks[dest].stack.push(c);
            }
        }

        Part2Solution {
            input: self.input,
            output: stacks
                .iter()
                .map(|s| s.stack[s.stack.len() - 1])
                .collect::<String>(),
        }
    }
}
