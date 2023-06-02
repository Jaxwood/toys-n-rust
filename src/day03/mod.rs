use crate::util;
use std::collections::hash_set::HashSet;
use std::collections::HashMap;

fn day03a(path: &str) -> usize {
    let lines = util::get_content(path);
    let mut scores: HashMap<char, usize> = HashMap::new();
    let letters: Vec<_> = ('a'..='z').chain('A'..='Z').collect();
    for (i, &c) in letters.iter().enumerate() {
        scores.insert(c, i + 1);
    }
    let mut sum = 0;
    for line in lines {
        let l = line.len() / 2;
        let head: Vec<_> = line.chars().take(l).collect();
        let tail: Vec<_> = line.chars().skip(l).collect();
        let head: HashSet<&char> = HashSet::from_iter(&head);
        let tail: HashSet<&char> = HashSet::from_iter(&tail);
        let overlap = head.intersection(&tail).collect::<HashSet<_>>();
        for &letter in overlap {
            sum += scores.get(letter).expect("score not found");
        }
    }
    sum
}

fn day03b(path: &str) -> usize {
    let lines = util::get_content(path);
    let line_groups = lines.chunks(3);
    let mut scores: HashMap<char, usize> = HashMap::new();
    let letters: Vec<_> = ('a'..='z').chain('A'..='Z').collect();
    for (i, &c) in letters.iter().enumerate() {
        scores.insert(c, i + 1);
    }
    let mut sum = 0;
    for group in line_groups {
        let first: Vec<_> = group[0].chars().collect();
        let second: Vec<_> = group[1].chars().collect();
        let third: Vec<_> = group[2].chars().collect();
        let first: HashSet<&char> = HashSet::from_iter(&first);
        let second: HashSet<&char> = HashSet::from_iter(&second);
        let third: HashSet<&char> = HashSet::from_iter(&third);
        let all = vec![second, third];
        let overlap = all.iter().fold(first, |acc, next| {
            acc.intersection(next).map(|&x| x).collect()
        });
        for letter in overlap {
            sum += scores.get(letter).expect("score not found");
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total_score_a() {
        let result = day03a("./data/day03.txt");
        assert_eq!(result, 157);
    }

    #[test]
    fn find_total_score_b() {
        let result = day03b("./data/day03.txt");
        assert_eq!(result, 70);
    }

    #[test]
    fn find_total_score_parta() {
        let result = day03a("./data/day03final.txt");
        assert_eq!(result, 8394);
    }

    #[test]
    fn find_total_score_partb() {
        let result = day03b("./data/day03final.txt");
        assert_eq!(result, 2413);
    }
}
