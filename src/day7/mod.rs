#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub struct Part1Solution {
    pub input: String,
    pub output: usize,
}

#[derive(Debug)]
struct Node {
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    children: Option<HashMap<String, Rc<RefCell<Node>>>>,
    file_sizes: usize,
}

impl Node {
    fn print(&self) -> String {
        let mut parent_name = String::from("None");
        if self.parent.is_some() {
            let tmp = self.parent.clone();
            parent_name = String::from(tmp.unwrap().as_ref().borrow().name.clone());
        }

        let mut children_str = String::from("None");
        if self.children.is_some() {
            let tmp = self.children.clone();
            children_str = format!("{:?}", tmp.unwrap().keys());
        }

        format!(
            "Node{{
    name: {},
    parent: {},
    children: {:?},
    file_sizes: {},
}}
        ",
            self.name, parent_name, children_str, self.file_sizes,
        )
    }
}

impl Part1Solution {
    pub fn new(input: String) -> Part1Solution {
        Part1Solution {
            input: input,
            output: 0,
        }
    }

    fn dfs(head: Rc<RefCell<Node>>, folder_sizes: &mut BinaryHeap<usize>) -> usize {
        match head.borrow().children.as_ref() {
            Some(children) => {
                let mut total_children: usize = head.borrow().file_sizes;
                for c in children.values() {
                    total_children += Self::dfs(Rc::clone(c), folder_sizes);
                }
                folder_sizes.push(total_children);
                total_children
            }
            None => {
                folder_sizes.push(head.borrow().file_sizes);
                head.borrow().file_sizes
            }
        }
    }

    pub fn solution(self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // start file tree
        // references in Node don't need to be mutable
        let head = Rc::new(RefCell::new(Node {
            name: String::from("/"),
            parent: None,
            children: None,
            file_sizes: 0,
        }));

        // ptr used during parsing and construction needs
        // to be able to mutate Nodes
        let mut ptr = Rc::clone(&head);

        // parse and populate file tree system
        for (ix, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();
            // println!("{} -> {:?}", ix, line_items);
            match &line_items[..] {
                // move to root
                ["$", "cd", "/"] => {
                    ptr = Rc::clone(&head);
                }
                // move up
                ["$", "cd", ".."] => {
                    let tmp =
                        Rc::clone(&(*ptr).borrow().parent.as_ref().unwrap_or_else(|| {
                            panic!("no parent found for {:?}", (*ptr).borrow())
                        }));
                    ptr = tmp;
                }
                // move down file tree
                ["$", "cd", folder_name] => {
                    let tmp = Rc::clone(
                        (*ptr)
                            .borrow()
                            .children
                            .as_ref()
                            .unwrap()
                            .get(&String::from(*folder_name))
                            .unwrap_or_else(|| {
                                panic!("{} not found in {:?}", folder_name, (*ptr).borrow())
                            }),
                    );
                    ptr = tmp;
                }
                // list all dir and files
                ["$", "ls"] => {}
                // info - directory
                ["dir", dir_name] => {
                    if ptr.borrow().children.is_none() {
                        ptr.borrow_mut().children = Some(HashMap::new());
                    }

                    ptr.borrow_mut().children.as_mut().unwrap().insert(
                        String::from(*dir_name),
                        Rc::from(RefCell::new(Node {
                            name: String::from(*dir_name),
                            parent: Some(Rc::clone(&ptr)),
                            children: None,
                            file_sizes: 0,
                        })),
                    );
                }
                // info - file
                [size, _] => {
                    ptr.borrow_mut().file_sizes += size.parse::<usize>().unwrap();
                }
                // unexpected input
                _ => {
                    panic!("unexpected input")
                }
            }
            // println!("{}", (*ptr).borrow().print());
        }

        // DFS traverse tree to find totals
        let mut folder_sizes = BinaryHeap::<usize>::new();
        Self::dfs(head, &mut folder_sizes);

        let mut total_at_most_100_000: usize = 0;
        loop {
            if *folder_sizes.peek().unwrap_or(&0) >= 100_000 {
                folder_sizes.pop();
            } else {
                break;
            }
        }
        for folder_size in folder_sizes.drain() {
            total_at_most_100_000 += folder_size;
        }

        Part1Solution {
            input: self.input,
            output: total_at_most_100_000,
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
    fn dfs(head: Rc<RefCell<Node>>, folder_sizes: &mut BinaryHeap<usize>) -> usize {
        match head.borrow().children.as_ref() {
            Some(children) => {
                let mut total_children: usize = head.borrow().file_sizes;
                for c in children.values() {
                    total_children += Self::dfs(Rc::clone(c), folder_sizes);
                }
                folder_sizes.push(total_children);
                total_children
            }
            None => {
                folder_sizes.push(head.borrow().file_sizes);
                head.borrow().file_sizes
            }
        }
    }

    pub fn solution(self) -> Part2Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        // start file tree
        // references in Node don't need to be mutable
        let head = Rc::new(RefCell::new(Node {
            name: String::from("/"),
            parent: None,
            children: None,
            file_sizes: 0,
        }));

        // ptr used during parsing and construction needs
        // to be able to mutate Nodes
        let mut ptr = Rc::clone(&head);

        // parse and populate file tree system
        for (ix, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let line_items = line.split(" ").collect::<Vec<&str>>();
            // println!("{} -> {:?}", ix, line_items);
            match &line_items[..] {
                // move to root
                ["$", "cd", "/"] => {
                    ptr = Rc::clone(&head);
                }
                // move up
                ["$", "cd", ".."] => {
                    let tmp =
                        Rc::clone(&(*ptr).borrow().parent.as_ref().unwrap_or_else(|| {
                            panic!("no parent found for {:?}", (*ptr).borrow())
                        }));
                    ptr = tmp;
                }
                // move down file tree
                ["$", "cd", folder_name] => {
                    let tmp = Rc::clone(
                        (*ptr)
                            .borrow()
                            .children
                            .as_ref()
                            .unwrap()
                            .get(&String::from(*folder_name))
                            .unwrap_or_else(|| {
                                panic!("{} not found in {:?}", folder_name, (*ptr).borrow())
                            }),
                    );
                    ptr = tmp;
                }
                // list all dir and files
                ["$", "ls"] => {}
                // info - directory
                ["dir", dir_name] => {
                    if ptr.borrow().children.is_none() {
                        ptr.borrow_mut().children = Some(HashMap::new());
                    }

                    ptr.borrow_mut().children.as_mut().unwrap().insert(
                        String::from(*dir_name),
                        Rc::from(RefCell::new(Node {
                            name: String::from(*dir_name),
                            parent: Some(Rc::clone(&ptr)),
                            children: None,
                            file_sizes: 0,
                        })),
                    );
                }
                // info - file
                [size, _] => {
                    ptr.borrow_mut().file_sizes += size.parse::<usize>().unwrap();
                }
                // unexpected input
                _ => {
                    panic!("unexpected input")
                }
            }
            // println!("{}", (*ptr).borrow().print());
        }

        // DFS traverse tree to find totals
        let mut folder_sizes = BinaryHeap::<usize>::new();
        Self::dfs(head, &mut folder_sizes);

        let need_free_space: usize = 30_000_000 - (70_000_000 - folder_sizes.peek().unwrap());
        let mut biggest_ish_directory: usize = 0;
        while *folder_sizes.peek().unwrap_or(&0) >= need_free_space {
            biggest_ish_directory = folder_sizes.pop().unwrap();
        }

        Part2Solution {
            input: self.input,
            output: biggest_ish_directory,
        }
    }
}
