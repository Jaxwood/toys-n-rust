#![allow(dead_code)]

use std::fs;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, numbers) = separated_list1(newline, complete::i32)(input)?;
    Ok((input, numbers))
}

fn day20a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, numbers) = parse(&content).unwrap();

    let mut indexes = (0..numbers.len()).collect::<Vec<_>>();
    for (idx, &num) in numbers.iter().enumerate() {
        let pos = indexes.iter().position(|&i| i == idx).unwrap();
        indexes.remove(pos);
        let new_i = (pos as i32 + num).rem_euclid(indexes.len() as i32) as usize;
        indexes.insert(new_i, idx);
    }

    let zero_idx = numbers.iter().position(|&i| i == 0).unwrap();
    let zero = indexes.iter().position(|&i| i == zero_idx).unwrap();
    [1000, 2000, 3000].iter().map(|i| numbers[indexes[(zero + i) % indexes.len()]]).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_groove_coordinates() {
        let actual = day20a("./data/day20.txt");
        assert_eq!(actual, 3);
    }

    #[test]
    fn find_groove_coordinates_part_a() {
        let actual = day20a("./data/day20final.txt");
        // not correct
        assert_eq!(actual, 2275);
    }
}
