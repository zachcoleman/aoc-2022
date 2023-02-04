use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

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

        // using two grids for performance
        let mut row_grid: Vec<Vec<usize>> = Vec::new(); // index i, j
        let mut col_grid: Vec<Vec<usize>> = Vec::new(); // index j, i
        for line in reader.lines() {
            let line = line.unwrap();
            let mut row: Vec<usize> = Vec::new();
            for (ix, c) in line.chars().enumerate() {
                row.push(c.to_digit(10).unwrap() as usize);
                if col_grid.get(ix).is_none() {
                    col_grid.push(Vec::new());
                }
                col_grid[ix].push(c.to_digit(10).unwrap() as usize);
            }
            row_grid.push(row);
        }

        // get the row-wise and col-wise "max from outside" grids
        let mut row_outer_max_grid: Vec<Vec<usize>> = Vec::new();
        let mut col_outer_max_grid: Vec<Vec<usize>> = Vec::new();

        for row in row_grid.iter() {
            // get cumulative max from left-to-right and right-to-left
            let mut curr_max: usize = 0;
            let forward_dir = row
                .iter()
                .map(|x| {
                    curr_max = usize::max(*x, curr_max);
                    curr_max
                })
                .collect::<Vec<usize>>();
            curr_max = 0;
            let reverse_dir = row
                .iter()
                .rev()
                .map(|x| {
                    curr_max = usize::max(*x, curr_max);
                    curr_max
                })
                .collect::<Vec<usize>>();

            // min of maxes from each direction (for determining max from outside)
            row_outer_max_grid.push(
                zip(forward_dir.iter(), reverse_dir.iter().rev())
                    .map(|x| usize::min(*x.0, *x.1))
                    .collect(),
            );
        }
        for col in col_grid.iter() {
            let mut curr_max: usize = 0;
            let forward_dir = col
                .iter()
                .map(|x| {
                    curr_max = usize::max(*x, curr_max);
                    curr_max
                })
                .collect::<Vec<usize>>();
            curr_max = 0;
            let reverse_dir = col
                .iter()
                .rev()
                .map(|x| {
                    curr_max = usize::max(*x, curr_max);
                    curr_max
                })
                .collect::<Vec<usize>>();

            // min of maxes from each direction (for determining max from outside)
            col_outer_max_grid.push(
                zip(forward_dir.iter(), reverse_dir.iter().rev())
                    .map(|x| usize::min(*x.0, *x.1))
                    .collect(),
            );
        }

        // the criteria is being greater than max from i+1, i-1, j+1, j-1
        let mut visible_count: usize = 0;
        for (i, row) in row_outer_max_grid.iter().enumerate() {
            for (j, col) in col_outer_max_grid.iter().enumerate() {
                let tmp = row_grid[i][j];
                // wrapping_sub will return usize::MAX if i == 0
                // this is just a hack to avoid if/else statements
                // and use match instead knowing row.get(usize::MAX)
                // will return None
                match row.get(j.wrapping_sub(1)) {
                    Some(x) => {
                        if tmp > *x {
                            visible_count += 1;
                            continue;
                        }
                    } // visible from left
                    None => {
                        visible_count += 1;
                        continue;
                    } // on outer edge
                }
                match row.get(j + 1) {
                    Some(x) => {
                        if tmp > *x {
                            visible_count += 1;
                            continue;
                        }
                    } // visible from right
                    None => {
                        visible_count += 1;
                        continue;
                    } // on outer edge
                }
                match col.get(i.wrapping_sub(1)) {
                    Some(x) => {
                        if tmp > *x {
                            visible_count += 1;
                            continue;
                        }
                    } // visible from top
                    None => {
                        visible_count += 1;
                        continue;
                    } // on outer edge
                }
                match col.get(i + 1) {
                    Some(x) => {
                        if tmp > *x {
                            visible_count += 1;
                            continue;
                        }
                    } // visible from bottom
                    None => {
                        visible_count += 1;
                        continue;
                    } // on outer edge
                }
            }
        }

        Part1Solution {
            input: self.input,
            output: visible_count,
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
    // an O(1) since they're fixed size sub loops
    // fn update_tracker(tracker: &mut [Option<usize>; 10], height: &usize, ix: &usize) {
    //     for i in 0..(*height+1) {
    //         tracker[i] = Some(*ix);
    //     }
    // }
    // could be better performance since compiler will unroll it (less assembly generated via godbolt)
    fn update_tracker(tracker: &mut [Option<usize>; 10], height: &usize, ix: &usize) {
        for i in 0..10 {
            if i <= *height {
                tracker[i] = Some(*ix);
            }
        }
    }
    pub fn solution(self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // using two grids for performance
        let mut row_grid: Vec<Vec<usize>> = Vec::new(); // index i, j
        let mut col_grid: Vec<Vec<usize>> = Vec::new(); // index j, i
        for line in reader.lines() {
            let line = line.unwrap();
            let mut row: Vec<usize> = Vec::new();
            for (ix, c) in line.chars().enumerate() {
                row.push(c.to_digit(10).unwrap() as usize);
                if col_grid.get(ix).is_none() {
                    col_grid.push(Vec::new());
                }
                col_grid[ix].push(c.to_digit(10).unwrap() as usize);
            }
            row_grid.push(row);
        }

        let mut row_res: Vec<Vec<usize>> = Vec::new();
        for row in &row_grid {
            // left view
            let mut left_res: Vec<usize> = Vec::new();
            let mut tracker: [Option<usize>; 10] = [None; 10];
            for (ix, val) in row.iter().enumerate() {
                match tracker[*val] {
                    Some(x) => {
                        left_res.push(ix - x);
                    }
                    None => {
                        left_res.push(ix);
                    }
                }
                Self::update_tracker(&mut tracker, val, &ix);
            }
            // right view
            let mut right_res: Vec<usize> = Vec::new();
            tracker = [None; 10];
            for (ix, val) in row.iter().rev().enumerate() {
                match tracker[*val] {
                    Some(x) => {
                        right_res.push(ix - x);
                    }
                    None => {
                        right_res.push(ix);
                    }
                }
                Self::update_tracker(&mut tracker, val, &ix);
            }
            right_res = right_res.iter().rev().map(|x| *x).collect();

            row_res.push(zip(left_res, right_res).map(|x| x.0 * x.1).collect());
        }

        let mut col_res: Vec<Vec<usize>> = Vec::new();
        for col in &col_grid {
            // top view
            let mut top_res: Vec<usize> = Vec::new();
            let mut tracker: [Option<usize>; 10] = [None; 10];
            for (ix, val) in col.iter().enumerate() {
                match tracker[*val] {
                    Some(x) => {
                        top_res.push(ix - x);
                    }
                    None => {
                        top_res.push(ix);
                    }
                }
                Self::update_tracker(&mut tracker, val, &ix);
            }
            // bottom view
            let mut bottom_res: Vec<usize> = Vec::new();
            tracker = [None; 10];
            for (ix, val) in col.iter().rev().enumerate() {
                match tracker[*val] {
                    Some(x) => {
                        bottom_res.push(ix - x);
                    }
                    None => {
                        bottom_res.push(ix);
                    }
                }
                Self::update_tracker(&mut tracker, val, &ix);
            }
            bottom_res = bottom_res.iter().rev().map(|x| *x).collect();

            col_res.push(zip(top_res, bottom_res).map(|x| x.0 * x.1).collect());
        }

        let mut max: usize = 0;
        for (i, row) in row_res.iter().enumerate() {
            for (j, col) in col_res.iter().enumerate() {
                max = usize::max(row[j] * col[i], max);
            }
        }

        Part2Solution {
            input: self.input,
            output: max,
        }
    }
}
