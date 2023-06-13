#![allow(dead_code)]
use std::collections::HashMap;

use crate::util;

fn is_visible(
    forest: &HashMap<(usize, usize), u32>,
    (x, y): &(usize, usize),
    width: usize,
) -> bool {
    let candidate = forest.get(&(*x, *y)).expect("key not found");
    let mut visible = true;

    // check left y-axis
    for yy in 0..*y {
        let tree = forest.get(&(*x, yy)).expect("key not found");
        if tree >= candidate {
            visible = false;
        }
    }

    if visible == true {
        return visible;
    }
    visible = true;

    // check right y-axis
    for yy in *y+1..width {
        let tree = forest.get(&(*x, yy)).expect("key not found");
        if tree >= candidate {
            visible = false;
        }
    }

    if visible == true {
        return visible;
    }
    visible = true;

    // check top x-axis
    for xx in 0..*x {
        let tree = forest.get(&(xx, *y)).expect("key not found");
        if tree >= candidate {
            visible = false;
        }
    }

    if visible == true {
        return visible;
    }
    visible = true;

    // check bottom x-axis
    for xx in *x+1..width {
        let tree = forest.get(&(xx, *y)).expect("key not found");
        if tree >= candidate {
            visible = false;
        }
    }

    visible
}

fn day08a(path: &str, width: usize) -> usize {
    let lines = util::get_content(path);
    dbg!(lines.len());

    let mut forest: HashMap<(usize, usize), u32> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().into_iter().enumerate() {
            forest.insert((x, y), c.to_digit(10).expect("convertion failed"));
        }
    }

    let visible: Vec<&(usize, usize)> = forest
        .keys()
        .filter(|&tree| is_visible(&forest, tree, width))
        .collect();

    visible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_visible_trees() {
        let result = day08a("./data/day08.txt", 5);
        assert_eq!(result, 21);
    }

    #[test]
    fn find_visible_trees_part_a() {
        let result = day08a("./data/day08final.txt", 99);
        assert_eq!(result, 1684);
    }
}
