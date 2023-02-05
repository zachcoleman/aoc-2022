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
        let (mut start, mut end) = ((0, 0), (0, 0));
        let mut grid: Vec<Vec<usize>> = Vec::new();
        for (m, line) in reader.lines().enumerate() {
            grid.push(
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(n, c)| match c {
                        'S' => {
                            start = (m, n);
                            0
                        }
                        'E' => {
                            end = (m, n);
                            25
                        }
                        _ => c as usize - 97,
                    })
                    .collect(),
            );
        }

        // Dijkstra's algorithm
        let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; grid[0].len()]; grid.len()];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue: Vec<(usize, usize, usize)> = Vec::new();
        queue.push((start.0, start.1, 0));
        while queue.len() > 0 {
            let (m, n, d) = queue.remove(0);
            dist[m][n] = d;
            visited[m][n] = true;
            for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (m2, n2) = (m as i32 + dir.0, n as i32 + dir.1);
                if m2 >= 0
                    && m2 < grid.len() as i32
                    && n2 >= 0
                    && n2 < grid[0].len() as i32
                    && grid[m2 as usize][n2 as usize] <= grid[m][n] + 1
                {
                    let (m2, n2) = (m2 as usize, n2 as usize);
                    if !visited[m2][n2] && dist[m2][n2] >= d + 1 {
                        visited[m2][n2] = true;
                        if (m2, n2) == end {
                            return Part1Solution {
                                input: self.input,
                                output: d + 1,
                            };
                        }
                        queue.push((m2, n2, d + 1));
                    }
                }
            }
        }

        Part1Solution {
            input: self.input,
            output: 0,
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
        let (mut start, mut end) = ((0, 0), (0, 0));
        let mut grid: Vec<Vec<usize>> = Vec::new();
        for (m, line) in reader.lines().enumerate() {
            grid.push(
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(n, c)| match c {
                        'S' => {
                            start = (m, n);
                            0
                        }
                        'E' => {
                            end = (m, n);
                            25
                        }
                        _ => c as usize - 97,
                    })
                    .collect(),
            );
        }

        // Dijkstra's algorithm
        let mut min = std::usize::MAX;
        let mut dist: Vec<Vec<usize>> = vec![vec![std::usize::MAX; grid[0].len()]; grid.len()];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        let mut queue: Vec<(usize, usize, usize)> = Vec::new();
        queue.push((end.0, end.1, 0));
        while queue.len() > 0 {
            let (m, n, d) = queue.remove(0);
            dist[m][n] = d;
            visited[m][n] = true;
            for dir in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (m2, n2) = (m as i32 + dir.0, n as i32 + dir.1);
                // reversing the direction and checking the inverse condition
                // instead of moving <= height + 1, we move >= height - 1
                // this is because we are moving backwards
                if m2 >= 0
                    && m2 < grid.len() as i32
                    && n2 >= 0
                    && n2 < grid[0].len() as i32
                    && grid[m][n] <= grid[m2 as usize][n2 as usize] + 1
                {
                    let (m2, n2) = (m2 as usize, n2 as usize);
                    if !visited[m2][n2] && dist[m2][n2] >= d + 1 {
                        visited[m2][n2] = true;
                        if grid[m2][n2] == 0 {
                            if d + 1 < min {
                                min = d + 1;
                            }
                        }
                        queue.push((m2, n2, d + 1));
                    }
                }
            }
        }

        Part2Solution {
            input: self.input,
            output: min,
        }
    }
}
