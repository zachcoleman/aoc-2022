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
    pub fn solution(&mut self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        let mut score = 0;
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    let round = line.split(" ").into_iter().collect::<Vec<&str>>();
                    // round result points X (lose), Y (draw), Z (win)
                    match round.as_slice() {
                        // opponent chooses rock
                        ["A", _] => {
                            match round.as_slice() {
                                [_, "X"] => {
                                    score += 3;
                                } // choose scissors to lose (3 scissors + 0 lose)
                                [_, "Y"] => {
                                    score += 4;
                                } // choose rock to draw (1 rock + 3 draw)
                                [_, "Z"] => {
                                    score += 8;
                                } // choose paper to win (2 paper + 6 win)
                                _ => {
                                    panic!("Invalid input")
                                }
                            }
                        }
                        // opponent chooses paper
                        ["B", _] => {
                            match round.as_slice() {
                                [_, "X"] => {
                                    score += 1;
                                } // choose rock to lose (1 rock + 0 lose)
                                [_, "Y"] => {
                                    score += 5;
                                } // choose paper to draw (2 paper + 3 draw)
                                [_, "Z"] => {
                                    score += 9;
                                } // choose scissors to win (3 scissors + 6 win)
                                _ => {
                                    panic!("Invalid input")
                                }
                            }
                        }
                        // opponent chooses scissors
                        ["C", _] => {
                            match round.as_slice() {
                                [_, "X"] => {
                                    score += 2;
                                } // choose paper to lose (2 paper + 0 lose)
                                [_, "Y"] => {
                                    score += 6;
                                } // choose scissors to draw (3 scissors + 3 draw)
                                [_, "Z"] => {
                                    score += 7;
                                } // choose rock to win (1 rock + 6 win)
                                _ => {
                                    panic!("Invalid input")
                                }
                            }
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

        return Part2Solution {
            input: self.input.clone(),
            output: score,
        };
    }
}
