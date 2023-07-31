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

fn mod_floor(a: i32, n: i32) -> i32 {
    return ((a % n) + n) % n;
}

fn day20a(path: &str) -> i32 {
    let content = fs::read_to_string(path).expect("file not found");
    let (_, numbers) = parse(&content).unwrap();
    let result = numbers.iter().fold(numbers.clone(), |mut acc, num| {
        println!("{:?}", acc);
        let idx = acc.iter().position(|x| x == num).unwrap();
        let new_idx = idx as i32 + num; 
        acc.remove(idx);
        acc.insert(new_idx as usize, *num);
        acc
    });
    println!("{:?}", result);
    0
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
    #[ignore]
    fn find_groove_coordinates_part_a() {
        let actual = day20a("./data/day20final.txt");
        assert_eq!(actual, 3);
    }
}
