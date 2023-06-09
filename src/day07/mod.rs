#![allow(dead_code)]
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, digit0, newline, not_line_ending, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

struct Node {
    name: String,
    size: usize,
    children: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: &str, size: usize) -> Self {
        Node {
            name: String::from(name),
            size,
            parent: None,
            children: vec![],
        }
    }

    pub fn add_child(&mut self, new_node: Rc<RefCell<Node>>) {
        self.children.push(new_node);
    }

    pub fn sum(&self) -> usize {
        let mut sum = self.size;

        for child in self.children.iter() {
            sum = sum + child.borrow().sum();
        }
        return sum;
    }

    pub fn is_dir(&self) -> bool {
        self.size == 0
    }

    pub fn visit(&self, acc: &mut Vec<usize>) {
        acc.push(self.sum());

        for child in self.children.iter() {
            if child.borrow().is_dir() {
                child.borrow().visit(acc);
            }
        }
    }

    pub fn print(&self) {
        for child in self.children.iter() {
            child.borrow().print();
        }
        if let Some(parent) = self.parent.clone() {
            println!("{} {} {}", parent.borrow().name, self.name, self.size);
        } else {
            println!("{} {}", self.name, self.size);
        }
    }
}

enum InstructionSet<'a> {
    Cd(&'a str),
    Ls,
    Dir(&'a str),
    File(&'a str, usize),
}

fn parse_cd(input: &str) -> IResult<&str, InstructionSet> {
    let (input, c) = preceded(tag("$ cd "), alt((tag("/"), tag(".."), alpha0)))(input)?;
    Ok((input, InstructionSet::Cd(c)))
}

fn parse_ls(input: &str) -> IResult<&str, InstructionSet> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, InstructionSet::Ls))
}

fn parse_dir(input: &str) -> IResult<&str, InstructionSet> {
    let (input, c) = preceded(tag("dir "), alpha0)(input)?;
    Ok((input, InstructionSet::Dir(c)))
}

fn parse_file(input: &str) -> IResult<&str, InstructionSet> {
    let (input, (c, e)) = separated_pair(digit0, space1, not_line_ending)(input)?;
    match c.parse::<usize>() {
        Ok(size) => Ok((input, InstructionSet::File(e, size))),
        _ => panic!("could not parse file"),
    }
}

fn parse(input: &str) -> Vec<InstructionSet> {
    let result = separated_list1(newline, alt((parse_cd, parse_ls, parse_dir, parse_file)))(input);
    match result {
        Ok((_, instructions)) => instructions,
        _ => panic!(""),
    }
}

fn day07a(path: &str, max_size: usize) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let instructions = parse(content.as_str());

    let tree = Rc::new(RefCell::new(Node::new("/", 0)));
    let mut current = Rc::clone(&tree);
    if let Some((_, tail)) = instructions.split_first() {
        for instruction in tail {
            match instruction {
                InstructionSet::Cd(d) if *d == ".." => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                InstructionSet::Cd(d) => {
                    let current_clone = Rc::clone(&current);
                    for child in current_clone.borrow().children.iter() {
                        if child.borrow().name == *d {
                            current = Rc::clone(child);
                        }
                    }
                }
                InstructionSet::Ls => continue,
                InstructionSet::Dir(name) => {
                    let new_node = Rc::new(RefCell::new(Node::new(*name, 0)));
                    current.borrow_mut().children.push(Rc::clone(&new_node));
                    let mut mut_child = new_node.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
                InstructionSet::File(name, size) => {
                    let new_node = Rc::new(RefCell::new(Node::new(*name, *size)));
                    current.borrow_mut().children.push(Rc::clone(&new_node));
                    let mut mut_child = new_node.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
            };
        }
    }

    let mut directory_sizes = vec![];
    tree.borrow().visit(&mut directory_sizes);
    directory_sizes.iter().filter(|&&x| x <= max_size).sum()
}

fn day07b(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    let instructions = parse(content.as_str());

    let tree = Rc::new(RefCell::new(Node::new("/", 0)));
    let mut current = Rc::clone(&tree);
    if let Some((_, tail)) = instructions.split_first() {
        for instruction in tail {
            match instruction {
                InstructionSet::Cd(d) if *d == ".." => {
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                InstructionSet::Cd(d) => {
                    let current_clone = Rc::clone(&current);
                    for child in current_clone.borrow().children.iter() {
                        if child.borrow().name == *d {
                            current = Rc::clone(child);
                        }
                    }
                }
                InstructionSet::Ls => continue,
                InstructionSet::Dir(name) => {
                    let new_node = Rc::new(RefCell::new(Node::new(*name, 0)));
                    current.borrow_mut().children.push(Rc::clone(&new_node));
                    let mut mut_child = new_node.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
                InstructionSet::File(name, size) => {
                    let new_node = Rc::new(RefCell::new(Node::new(*name, *size)));
                    current.borrow_mut().children.push(Rc::clone(&new_node));
                    let mut mut_child = new_node.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
            };
        }
    }

    let space_used = 70000000 - tree.borrow().sum();
    let space_need = 30000000 - space_used;
    let mut directory_sizes = vec![];
    tree.borrow().visit(&mut directory_sizes);
    *(directory_sizes
        .iter()
        .filter(|&&x| x >= space_need)
        .min()
        .expect("should find lowest"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total_size_a() {
        let result = day07a("./data/day07.txt", 100000);
        assert_eq!(result, 95437);
    }

    #[test]
    fn find_total_size_b() {
        let result = day07b("./data/day07.txt");
        assert_eq!(result, 24933642);
    }

    #[test]
    fn find_total_size_parta() {
        let result = day07a("./data/day07final.txt", 100000);
        assert_eq!(result, 1141028);
    }

    #[test]
    fn find_total_size_partb() {
        let result = day07b("./data/day07final.txt");
        assert_eq!(result, 8278005);
    }
}
