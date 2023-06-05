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

fn day07(path: &str) -> usize {
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

    tree.borrow().print();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total_size() {
        let result = day07("./data/day07.txt");
        assert_eq!(result, 95437);
    }
}
