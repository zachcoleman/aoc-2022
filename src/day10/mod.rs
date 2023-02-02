use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Part1Solution {
    pub input: String,
    pub output: i32,
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

        let mut sum = 0;
        let recorded_ticks = HashSet::from([20, 60, 100, 140, 180, 220]);
        let mut tick_counter: usize = 1;
        let mut register: i32 = 1;
        let mut queue: [Option<i32>; 2] = [None, None];
        let mut queue_slice = &mut queue[0..1];

        for line in reader.lines() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();

            match &line_items[..] {
                ["noop"] => {
                    queue = [None, None];
                    queue_slice = &mut queue[0..1];
                }
                ["addx", add_val] => {
                    queue = [None, Some(add_val.parse::<i32>().unwrap())];
                    queue_slice = &mut queue[0..2];
                }
                _ => {
                    println!("Unknown cpu command");
                }
            }

            for op in queue_slice.iter() {
                // start cycle

                // during cycle
                if recorded_ticks.get(&tick_counter).is_some() {
                    sum += (tick_counter as i32) * register;
                }

                // complete cycle
                if op.is_some() {
                    register += op.unwrap();
                }
                tick_counter += 1;
            }
        }

        Part1Solution {
            input: self.input,
            output: sum,
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

        let mut screen_ret = vec![];
        let mut tick_counter: usize = 0;
        let mut register: i32 = 1;
        let mut queue: [Option<i32>; 2] = [None, None];
        let mut queue_slice = &mut queue[0..1];

        for line in reader.lines() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();

            match &line_items[..] {
                ["noop"] => {
                    queue = [None, None];
                    queue_slice = &mut queue[0..1];
                }
                ["addx", add_val] => {
                    queue = [None, Some(add_val.parse::<i32>().unwrap())];
                    queue_slice = &mut queue[0..2];
                }
                _ => {
                    println!("Unknown cpu command");
                }
            }

            for op in queue_slice.iter() {
                // start cycle

                // during cycle
                if (tick_counter as i32 % 40) - 1 <= register
                    && register <= (tick_counter as i32 % 40) + 1
                {
                    screen_ret.push("#");
                } else {
                    screen_ret.push(".");
                }

                // complete cycle
                if op.is_some() {
                    register += op.unwrap();
                }
                tick_counter += 1;
            }
        }

        let ret: String = screen_ret
            .chunks(40)
            .map(|chunk| chunk.join(""))
            .collect::<Vec<String>>()
            .join("\n");

        Part2Solution {
            input: self.input,
            output: ret,
        }
    }
}
