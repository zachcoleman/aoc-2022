mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
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
    println!("===== day1 solutions =====");
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
    println!("===== day2 solutions =====");
    let sol =
        day2::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day2/input.txt".to_string())
            .solution();
    println!("score: {}", sol.output.to_string());
    let sol =
        day2::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day2/input.txt".to_string())
            .solution();
    println!("score: {}", sol.output.to_string());

    // day3 solutions
    println!("===== day3 solutions =====");
    let sol =
        day3::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day3/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day3::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day3/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day4 solutions
    println!("===== day4 solutions =====");
    let sol =
        day4::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day4/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day4::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day4/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day5 solutions
    println!("===== day5 solutions =====");
    let sol =
        day5::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day5/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day5::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day5/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day6 solutions
    println!("===== day6 solutions =====");
    let sol =
        day6::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day6/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day6::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day6/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day7 solutions
    println!("===== day7 solutions =====");
    let sol =
        day7::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day7/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day7::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day7/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day8 solutions
    println!("===== day8 solutions =====");
    let sol =
        day8::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day8/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day8::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day8/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day9 solutions
    println!("===== day9 solutions =====");
    let sol =
        day9::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day9/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day9::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day9/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day10 solutions
    println!("===== day10 solutions =====");
    let sol =
        day10::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day10/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day10::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day10/input.txt".to_string())
            .solution();
    println!("output:");
    println!("{}", sol.output.to_string());

    // day11 solutions
    println!("===== day11 solutions =====");
    let sol =
        day11::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day11/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day11::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day11/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day12 solutions
    println!("===== day12 solutions =====");
    let sol =
        day12::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day12/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day12::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day12/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());

    // day13 solutions
    println!("===== day13 solutions =====");
    let sol =
        day13::Part1Solution::new("/Users/zachcoleman/aoc-2022/src/day13/tmp.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());
    let sol =
        day13::Part2Solution::new("/Users/zachcoleman/aoc-2022/src/day13/input.txt".to_string())
            .solution();
    println!("total: {}", sol.output.to_string());


    #[derive(Debug)]
    enum Tree<T> 
    where T: Clone {
        Node(T, Vec<Tree<T>>),
        Leaf(T),
    }
    
    impl<T> Tree<T> 
    where T: Clone
    {
        fn new(value: T) -> Self {
            Tree::Leaf(value)
        }
    
        fn add_child(&mut self, value: T) {
            match self {
                Tree::Leaf(leaf) => {
                    *self = Tree::Node(leaf.clone(), vec![Tree::Leaf(value)]);
                },
                Tree::Node(node_value, children) => {
                    children.push(Tree::Leaf(value));
                }
            }
        }
    }

    let mut tree = Tree::new(0);
    tree.add_child(1);
    tree.add_child(2);

    match &mut tree {
        Tree::Node(_, children) => {
            children[0].add_child(3);
            children[0].add_child(4);
        },
        Tree::Leaf(_) => {}
    }

    println!("{:?}", tree);

    if let Tree::Node(_, children) = &mut tree {
        if let Tree::Leaf(leaf) = &mut children[0] {
            *leaf = 2;
        }
    }

    println!("{:?}", tree);


}
