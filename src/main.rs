mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    // day1 solutions
    let sol =
        day1::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day1/input.txt".to_string())
            .solution();
    println!("top elf total cals: {}", sol.output.to_string());
    let sol =
        day1::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day1/input.txt".to_string())
            .solution();
    println!(
        "top 3 elves total cals: {:?}",
        sol.output.iter().sum::<usize>()
    );

    // day2 solutions
    let sol =
        day2::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day2/input.txt".to_string())
            .solution();
    println!("score: {}", sol.output.to_string());
    let sol =
        day2::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day2/input.txt".to_string())
            .solution();
    println!("score: {}", sol.output.to_string());

    // day3 solutions
    let sol =
        day3::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day3/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day3::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day3/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day4 solutions
    let sol =
        day4::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day4/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day4::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day4/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day5 solutions
    let sol =
        day5::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day5/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day5::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day5/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day6 solutions
    let sol =
        day6::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day6/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day6::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day6/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day7 solutions
    let sol =
        day7::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day7/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day7::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day7/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day8 solutions
    let sol =
        day8::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day8/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day8::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day8/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day9 solutions
    let sol =
        day9::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day9/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day9::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day9/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day10 solutions
    let sol =
        day10::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day10/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day10::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day10/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
}
