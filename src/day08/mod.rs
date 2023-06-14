#![allow(dead_code)]
use std::collections::HashMap;

use crate::util;

fn scenic(forest: &HashMap<(usize, usize), u32>, (x, y): &(usize, usize), width: usize) -> usize {
    let candidate = forest.get(&(*x, *y)).expect("key not found");

    // go left
    let mut left_trees = 0;
    for xx in (0..*x).rev() {
        let tree = forest.get(&(xx, *y)).expect("key not found");
        left_trees += 1;
        if tree >= candidate {
            break;
        }
    }

    // go rigth
    let mut right_trees = 0;
    for xx in *x + 1..width {
        let tree = forest.get(&(xx, *y)).expect("key not found");
        right_trees += 1;
        if tree >= candidate {
            break;
        }
    }

    // go up
    let mut up_trees = 0;
    for yy in (0..*y).rev() {
        let tree = forest.get(&(*x, yy)).expect("key not found");
        up_trees += 1;
        if tree >= candidate {
            break;
        }
    }

    let mut down_trees = 0;
    for yy in *y + 1..width {
        let tree = forest.get(&(*x, yy)).expect("key not found");
        down_trees += 1;
        if tree >= candidate {
            break;
        }
    }

    up_trees * down_trees * left_trees * right_trees
}

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
    for yy in *y + 1..width {
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
    for xx in *x + 1..width {
        let tree = forest.get(&(xx, *y)).expect("key not found");
        if tree >= candidate {
            visible = false;
        }
    }

    visible
}

fn day08a(path: &str, width: usize) -> usize {
    let lines = util::get_content(path);

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

fn day08b(path: &str, width: usize) -> usize {
    let lines = util::get_content(path);

    let mut forest: HashMap<(usize, usize), u32> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().into_iter().enumerate() {
            forest.insert((x, y), c.to_digit(10).expect("convertion failed"));
        }
    }

    let scenic_score: Option<usize> = forest
        .keys()
        .map(|&tree| scenic(&forest, &tree, width))
        .max();

    scenic_score.unwrap()
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
    fn find_scenic_score() {
        let result = day08b("./data/day08.txt", 5);
        assert_eq!(result, 8);
    }

    #[test]
    fn find_visible_trees_part_a() {
        let result = day08a("./data/day08final.txt", 99);
        assert_eq!(result, 1684);
    }

    #[test]
    fn find_visible_trees_part_b() {
        let result = day08b("./data/day08final.txt", 99);
        assert_eq!(result, 486540);
    }
}
