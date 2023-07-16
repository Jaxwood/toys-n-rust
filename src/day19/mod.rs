#![allow(dead_code)]

use std::fs;

fn day19a(path: &str) -> usize {
    let content = fs::read_to_string(path).expect("file not found");
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_() {
        let actual = day19a("./data/day19.txt");
        assert_eq!(actual, 33);
    }
}
