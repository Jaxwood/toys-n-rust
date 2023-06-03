use std::collections::HashSet;

use crate::util;

fn get_nums(start: Option<&&str>, end: Option<&&str>) -> Vec<usize> {
    let from = start.expect("start not found");
    let to = end.expect("end not found");
    let from: usize = (*from).parse().expect("not a number");
    let to: usize = (*to).parse().expect("not a number");
    vec![from, to]
}

fn contain_other(ranges: &Vec<Vec<usize>>) -> bool {
    let first = ranges.get(0).expect("should have range");
    let second = ranges.get(1).expect("should have range");
    let from = first.get(0).expect("should have number");
    let to = first.get(1).expect("should have number");
    let other_from = second.get(0).expect("should have number");
    let other_to = second.get(1).expect("should have number");

    if from >= other_from && to <= other_to {
        return true;
    }
    if other_from >= from && other_to <= to {
        return true;
    }

    false
}

fn overlap_other(ranges: &Vec<Vec<usize>>) -> bool {
    let first = ranges.get(0).expect("should have range");
    let second = ranges.get(1).expect("should have range");
    let from = first.get(0).expect("should have number");
    let to = first.get(1).expect("should have number");
    let other_from = second.get(0).expect("should have number");
    let other_to = second.get(1).expect("should have number");

    let range: HashSet<usize> = (*from..=*to).collect();
    let other_range: HashSet<usize> = (*other_from..=*other_to).collect();

    !range.is_disjoint(&other_range)
}

fn day04a(path: &str) -> usize {
    let lines = util::get_content(path);
    let mut total = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let mut ranges: Vec<Vec<usize>> = vec![];
        for part in parts {
            let nums: Vec<&str> = part.split('-').collect();
            let range = get_nums(nums.get(0), nums.get(1));
            ranges.push(range);
        }
        if contain_other(&ranges) {
            total += 1;
        }
    }
    total
}

fn day04b(path: &str) -> usize {
    let lines = util::get_content(path);
    let mut total = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let mut ranges: Vec<Vec<usize>> = vec![];
        for part in parts {
            let nums: Vec<&str> = part.split('-').collect();
            let range = get_nums(nums.get(0), nums.get(1));
            ranges.push(range);
        }
        if overlap_other(&ranges) {
            total += 1;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_contains_other() {
        let result = day04a("./data/day04.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn find_overlaps() {
        let result = day04b("./data/day04.txt");
        assert_eq!(result, 4);
    }

    #[test]
    fn find_contains_other_parta() {
        let result = day04a("./data/day04final.txt");
        assert_eq!(result, 567);
    }

    #[test]
    fn find_contains_other_partb() {
        let result = day04b("./data/day04final.txt");
        assert_eq!(result, 907);
    }
}
