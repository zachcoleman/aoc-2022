use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cell::{RefCell, Ref};
use std::rc::Rc;

pub struct Part1Solution {
    pub input: String,
    pub output: usize,
}

#[derive(Debug)]
struct Node{
    parent: Option<Rc<RefCell<Node>>>,
    value: Option<i32>,
    children: Option<Vec<Rc<RefCell<Node>>>>,
}

impl Part1Solution {
    pub fn new(input: String) -> Part1Solution {
        Part1Solution {
            input: input,
            output: 0,
        }
    }
    fn print_tree_bfs(node: Rc<RefCell<Node>>) {
        let mut tmp = Rc::clone(&node);
        let mut queue = vec![node];
        println!("=====================");
        while queue.len() > 0 {
            tmp = queue.remove(0);
            match tmp.borrow().value {
                Some(x) => print!("{} ", x),
                None => println!("|-"),
            }
            if let Some(children) = tmp.borrow().children.as_ref() {
                for child in children {
                    queue.push(Rc::clone(child));
                }
            }
        }
        println!("=====================");
    }
    pub fn solution(self) -> Part1Solution {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let root = Rc::new(RefCell::new(Node {
                parent: None,
                value: None,
                children: None,
            }));
            let mut ptr = Rc::clone(&root);
            let line = line.unwrap();
            println!("line: {}", line);
            for c in line.chars() {
                match c {
                    '[' => { 
                        println!("moving down");
                        // creating a new node, and make it the current node
                        (*ptr).borrow_mut().children = Some(
                            vec![
                                Rc::new(RefCell::new(Node {
                                    parent: Some(Rc::clone(&ptr)),
                                    value: None,
                                    children: None,
                                }))
                            ]
                        );
                        let tmp = Rc::clone(&(*ptr).borrow().children.as_ref().unwrap()[0]);
                        ptr = tmp;
                    },
                    ']' => { 
                        println!("moving up");
                        // go back to the parent node
                        let tmp = (*ptr).borrow().parent.as_ref().unwrap().clone();
                        ptr = tmp;
                    }, 
                    _ => { 
                        // add a new node to the current node
                        // if it is a digit and ignore other characters
                        if let Some(x)  = c.to_digit(10){
                            println!("adding child");
                            if (*ptr).borrow().children.as_ref().is_none() {
                                (*ptr).borrow_mut().children = Some(vec![]);
                            }
                            (*ptr).borrow_mut().children.as_mut().unwrap().push(
                                Rc::new(RefCell::new(
                                    Node {
                                        parent: Some(Rc::clone(&ptr)),
                                        value: Some(x as i32),
                                        children: None,
                                    }
                                ))
                            );
                        }
                    }
                }
            }
            Self::print_tree_bfs(root);
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

        Part2Solution {
            input: self.input,
            output: 0,
        }
    }
}
