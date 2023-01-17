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
        let file = File::open(&self.input).expect("Unable to open file");
        let reader = BufReader::new(file);
        let mut max_calories = 0;
        let mut curr_calories = 0;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    match line.as_str() {
                        "" => {
                            // check the maximal calories
                            if curr_calories > max_calories {
                                max_calories = curr_calories;
                            }
                            // reset the current elf and calories
                            curr_calories = 0;
                        }
                        _ => {
                            curr_calories += line.parse::<usize>().unwrap();
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        // handle case of the last elf (no newline at the end of the file)
        if curr_calories > max_calories {
            max_calories = curr_calories;
        }

        // return Solution
        return Part1Solution {
            input: self.input.clone(),
            output: max_calories,
        };
    }
}

pub struct Part2Solution {
    pub input: String,
    pub output: [usize; 3],
}

impl Part2Solution {
    pub fn new(input: String) -> Part2Solution {
        Part2Solution {
            input: input,
            output: [0; 3],
        }
    }
    pub fn solution(&mut self) -> Part2Solution {
        let file = File::open(&self.input).expect("Unable to open file");
        let reader = BufReader::new(file);

        let mut top_cals = [0; 3];
        let mut curr_calories = 0;

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    match line.as_str() {
                        "" => {
                            // unrolled logic for top 3
                            // this is faster than sorting the array
                            if curr_calories > top_cals[2] {
                                top_cals[0] = top_cals[1];
                                top_cals[1] = top_cals[2];
                                top_cals[2] = curr_calories;
                            } else if curr_calories > top_cals[1] {
                                top_cals[0] = top_cals[1];
                                top_cals[1] = curr_calories;
                            } else if curr_calories > top_cals[0] {
                                top_cals[0] = curr_calories;
                            }
                            curr_calories = 0;
                        }
                        _ => {
                            curr_calories += line.parse::<usize>().unwrap();
                        }
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        // handle case of the last elf (no newline at the end of the file)
        if curr_calories > top_cals[0] {
            top_cals[0] = curr_calories;
        }

        // return Solution
        return Part2Solution {
            input: self.input.clone(),
            output: top_cals,
        };
    }
}
