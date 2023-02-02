#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::HashSet;
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

    fn is_adjacent(pos1: (i32, i32), pos2: (i32, i32)) -> bool {
        let tmp = (pos1.0 - pos2.0, pos1.1 - pos2.1);
        (-1 <= tmp.0 && tmp.0 <= 1) && (-1 <= tmp.1 && tmp.1 <= 1)
    }
    fn move_tail_to_adjacent(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
        match (head.0 - tail.0, head.1 - tail.1) {
            (2, _) => (head.0 - 1, head.1),  // head to right
            (-2, _) => (head.0 + 1, head.1), // head to left
            (_, 2) => (head.0, head.1 - 1),  // head above
            (_, -2) => (head.0, head.1 + 1), // head below
            _ => {
                panic!("somehow got off");
            }
        }
    }

    pub fn solution(self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // variables to track things
        let (mut head_pos, mut tail_pos) = ((0, 0), (0, 0));
        let mut head_move = (0, 0);
        let mut tail_visted: HashSet<(i32, i32)> = HashSet::from([tail_pos]);

        // parse file and apply movements
        for (ix, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();

            // get move
            match &line_items[..] {
                ["U", m] => {
                    head_move = (0i32, m.parse::<i32>().unwrap());
                }
                ["D", m] => {
                    head_move = (0i32, -m.parse::<i32>().unwrap());
                }
                ["R", m] => {
                    head_move = (m.parse::<i32>().unwrap(), 0);
                }
                ["L", m] => {
                    head_move = (-m.parse::<i32>().unwrap(), 0);
                }
                _ => {
                    panic!("unexpected input");
                }
            }

            // apply move to head
            // println!("{:?}: {:?} -> {:?}", line, head_pos, (head_pos.0 + head_move.0, head_pos.1 + head_move.1));
            // head_pos = (head_pos.0 + head_move.0, head_pos.1 + head_move.1);

            while head_move != (0, 0) {
                match head_move {
                    (i32::MIN..=-1, 0) => {
                        head_pos = (head_pos.0 - 1, head_pos.1);
                        head_move = (head_move.0 + 1, 0);
                    }
                    (1..=i32::MAX, 0) => {
                        head_pos = (head_pos.0 + 1, head_pos.1);
                        head_move = (head_move.0 - 1, 0);
                    }
                    (0, i32::MIN..=-1) => {
                        head_pos = (head_pos.0, head_pos.1 - 1);
                        head_move = (0, head_move.1 + 1);
                    }
                    (0, 1..=i32::MAX) => {
                        head_pos = (head_pos.0, head_pos.1 + 1);
                        head_move = (0, head_move.1 - 1);
                    }
                    _ => {}
                }
                if !Self::is_adjacent(head_pos, tail_pos) {
                    tail_pos = Self::move_tail_to_adjacent(&head_pos, &tail_pos);
                    tail_visted.insert(tail_pos);
                }
            }
        }

        Part1Solution {
            input: self.input,
            output: tail_visted.len(),
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

    fn is_adjacent(pos1: (i32, i32), pos2: (i32, i32)) -> bool {
        let tmp = (pos1.0 - pos2.0, pos1.1 - pos2.1);
        (-1 <= tmp.0 && tmp.0 <= 1) && (-1 <= tmp.1 && tmp.1 <= 1)
    }
    fn move_knot_to_adjacent(pos1: &(i32, i32), pos2: &(i32, i32)) -> (i32, i32) {
        match (pos1.0 - pos2.0, pos1.1 - pos2.1) {
            // 2-2 away
            (2, 2) => (pos1.0 - 1, pos1.1 - 1),
            (2, -2) => (pos1.0 - 1, pos1.1 + 1),
            (-2, 2) => (pos1.0 + 1, pos1.1 - 1),
            (-2, -2) => (pos1.0 + 1, pos1.1 + 1),
            // 1-2 away
            (2, _) => (pos1.0 - 1, pos1.1),
            (-2, _) => (pos1.0 + 1, pos1.1),
            (_, 2) => (pos1.0, pos1.1 - 1),
            (_, -2) => (pos1.0, pos1.1 + 1),
            _ => {
                panic!("{:?} and {:?} somehow got off", pos1, pos2);
            }
        }
    }

    pub fn solution(self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // variables to track things
        let mut knots: [(i32, i32); 10] = [(0, 0); 10];
        let mut head_move = (0, 0);
        let mut tail_visted: HashSet<(i32, i32)> = HashSet::from([*knots.last().unwrap()]);

        // parse file and apply movements
        for (ix, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();

            // get move
            match &line_items[..] {
                ["U", m] => {
                    head_move = (0i32, m.parse::<i32>().unwrap());
                }
                ["D", m] => {
                    head_move = (0i32, -m.parse::<i32>().unwrap());
                }
                ["R", m] => {
                    head_move = (m.parse::<i32>().unwrap(), 0);
                }
                ["L", m] => {
                    head_move = (-m.parse::<i32>().unwrap(), 0);
                }
                _ => {
                    panic!("unexpected input");
                }
            }

            // apply move to head
            // println!("{:?}: {:?} -> {:?}", line, head_pos, (head_pos.0 + head_move.0, head_pos.1 + head_move.1));
            // head_pos = (head_pos.0 + head_move.0, head_pos.1 + head_move.1);

            while head_move != (0, 0) {
                match head_move {
                    (i32::MIN..=-1, 0) => {
                        knots[0] = (knots[0].0 - 1, knots[0].1);
                        head_move = (head_move.0 + 1, 0);
                    }
                    (1..=i32::MAX, 0) => {
                        knots[0] = (knots[0].0 + 1, knots[0].1);
                        head_move = (head_move.0 - 1, 0);
                    }
                    (0, i32::MIN..=-1) => {
                        knots[0] = (knots[0].0, knots[0].1 - 1);
                        head_move = (0, head_move.1 + 1);
                    }
                    (0, 1..=i32::MAX) => {
                        knots[0] = (knots[0].0, knots[0].1 + 1);
                        head_move = (0, head_move.1 - 1);
                    }
                    _ => {}
                }

                // println!("knot 0: {:?}", &knots[0]);
                for i in 1..knots.len() {
                    if !Self::is_adjacent(knots[i - 1], knots[i]) {
                        knots[i] = Self::move_knot_to_adjacent(&knots[i - 1], &knots[i]);
                    }
                    // println!("knot {}: {:?}", i, &knots[i]);
                }
                tail_visted.insert(*knots.last().unwrap());
            }
        }

        Part2Solution {
            input: self.input,
            output: tail_visted.len(),
        }
    }
}
