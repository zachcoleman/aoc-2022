mod day1;
mod day2;

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
}
