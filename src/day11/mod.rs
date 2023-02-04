struct Monkey {
    items: Vec<usize>,
    op: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    throw_true: usize,
    throw_false: usize,
    inspected: usize,
}

impl Monkey {
    fn new(
        items: Vec<usize>,
        op: Box<dyn Fn(usize) -> usize>,
        test: Box<dyn Fn(usize) -> bool>,
        throw_true: usize,
        throw_false: usize,
    ) -> Monkey {
        Monkey {
            items: items,
            op: op,
            test: test,
            throw_true: throw_true,
            throw_false: throw_false,
            inspected: 0,
        }
    }
}

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
    fn tick(monkeys: &mut Vec<Monkey>, ix: usize) {
        let mut monkey = monkeys.get_mut(ix).unwrap();
        let (monkey_true_ix, monkey_false_ix) = (monkey.throw_true, monkey.throw_false);
        let (mut monkey_true_items, mut monkey_false_items) = (vec![], vec![]);

        for item in &monkey.items {
            let item = (monkey.op)(*item) / 3;
            monkey.inspected += 1;
            if (monkey.test)(item) {
                monkey_true_items.push(item);
            } else {
                monkey_false_items.push(item);
            }
        }
        monkey.items = vec![];

        monkey = monkeys.get_mut(monkey_true_ix).unwrap();
        for item in monkey_true_items {
            monkey.items.push(item);
        }

        monkey = monkeys.get_mut(monkey_false_ix).unwrap();
        for item in monkey_false_items {
            monkey.items.push(item);
        }
    }
    pub fn solution(self) -> Part1Solution {
        let mut monkeys: Vec<Monkey> = Vec::from([
            Monkey::new(
                vec![97, 81, 57, 57, 91, 61],
                Box::new(|x| x * 7),
                Box::new(|x| x % 11 == 0),
                5,
                6,
            ),
            Monkey::new(
                vec![88, 62, 68, 90],
                Box::new(|x| x * 17),
                Box::new(|x| x % 19 == 0),
                4,
                2,
            ),
            Monkey::new(
                vec![74, 87],
                Box::new(|x| x + 2),
                Box::new(|x| x % 5 == 0),
                7,
                4,
            ),
            Monkey::new(
                vec![53, 81, 60, 87, 90, 99, 75],
                Box::new(|x| x + 1),
                Box::new(|x| x % 2 == 0),
                2,
                1,
            ),
            Monkey::new(
                vec![57],
                Box::new(|x| x + 6),
                Box::new(|x| x % 13 == 0),
                7,
                0,
            ),
            Monkey::new(
                vec![54, 84, 91, 55, 59, 72, 75, 70],
                Box::new(|x| x * x),
                Box::new(|x| x % 7 == 0),
                6,
                3,
            ),
            Monkey::new(
                vec![95, 79, 79, 68, 78],
                Box::new(|x| x + 3),
                Box::new(|x| x % 3 == 0),
                1,
                3,
            ),
            Monkey::new(
                vec![61, 97, 67],
                Box::new(|x| x + 4),
                Box::new(|x| x % 17 == 0),
                0,
                5,
            ),
        ]);

        for _ in 0..20 {
            for ix in 0..monkeys.len() {
                Self::tick(&mut monkeys, ix);
            }
        }

        monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

        Part1Solution {
            input: self.input,
            output: monkeys[0].inspected * monkeys[1].inspected,
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
    fn tick(monkeys: &mut Vec<Monkey>, ix: usize) {
        let mut monkey = monkeys.get_mut(ix).unwrap();
        let (monkey_true_ix, monkey_false_ix) = (monkey.throw_true, monkey.throw_false);
        let (mut monkey_true_items, mut monkey_false_items) = (vec![], vec![]);

        for item in &monkey.items {
            let item = (monkey.op)(*item) % 9699690;
            monkey.inspected += 1;
            if (monkey.test)(item) {
                monkey_true_items.push(item);
            } else {
                monkey_false_items.push(item);
            }
        }
        monkey.items = vec![];

        monkey = monkeys.get_mut(monkey_true_ix).unwrap();
        for item in monkey_true_items {
            monkey.items.push(item);
        }

        monkey = monkeys.get_mut(monkey_false_ix).unwrap();
        for item in monkey_false_items {
            monkey.items.push(item);
        }
    }
    pub fn solution(self) -> Part2Solution {
        let mut monkeys: Vec<Monkey> = Vec::from([
            Monkey::new(
                vec![97, 81, 57, 57, 91, 61],
                Box::new(|x| x * 7),
                Box::new(|x| x % 11 == 0),
                5,
                6,
            ),
            Monkey::new(
                vec![88, 62, 68, 90],
                Box::new(|x| x * 17),
                Box::new(|x| x % 19 == 0),
                4,
                2,
            ),
            Monkey::new(
                vec![74, 87],
                Box::new(|x| x + 2),
                Box::new(|x| x % 5 == 0),
                7,
                4,
            ),
            Monkey::new(
                vec![53, 81, 60, 87, 90, 99, 75],
                Box::new(|x| x + 1),
                Box::new(|x| x % 2 == 0),
                2,
                1,
            ),
            Monkey::new(
                vec![57],
                Box::new(|x| x + 6),
                Box::new(|x| x % 13 == 0),
                7,
                0,
            ),
            Monkey::new(
                vec![54, 84, 91, 55, 59, 72, 75, 70],
                Box::new(|x| x * x),
                Box::new(|x| x % 7 == 0),
                6,
                3,
            ),
            Monkey::new(
                vec![95, 79, 79, 68, 78],
                Box::new(|x| x + 3),
                Box::new(|x| x % 3 == 0),
                1,
                3,
            ),
            Monkey::new(
                vec![61, 97, 67],
                Box::new(|x| x + 4),
                Box::new(|x| x % 17 == 0),
                0,
                5,
            ),
        ]);

        for _ in 0..10_000 {
            for ix in 0..monkeys.len() {
                Self::tick(&mut monkeys, ix);
            }
        }

        monkeys.sort_by(|a, b| b.inspected.cmp(&a.inspected));

        Part2Solution {
            input: self.input,
            output: monkeys[0].inspected * monkeys[1].inspected,
        }
    }
}
