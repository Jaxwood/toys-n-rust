use crate::util;
use regex::Regex;
use std::collections::HashMap;

fn day05a(path: &str, arrangement: &mut HashMap<i32, String>) -> String {
    let content = util::get_content(path);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("regex invalid");

    for line in content {
        for capture in re.captures_iter(line.as_str()) {
            let amount: &i32 = &capture[1].parse().expect("could not parse amount");
            let from: &i32 = &capture[2].parse().expect("could not parse from");
            let to: &i32 = &capture[3].parse().expect("could not parse to");
            for _ in 0..*amount {
                let from_m = arrangement.get_mut(from).expect("could not read from index");
                match from_m.pop() {
                    Some(ch) => {
                        if let Some(to_m) = arrangement.get_mut(to) {
                            to_m.push(ch);
                        }
                    },
                    None => println!("string is empty"),
                }
            }
        }
    }
    let mut result = String::from("");
    for x in 1..=(arrangement.len() as i32) {
        if let Some(candidate) = arrangement.get_mut(&x) {
            if let Some(c) = candidate.pop() {
                result.push(c);
            }
        }
    }

    result
}

fn day05b(path: &str, arrangement: &mut HashMap<i32, String>) -> String {
    let content = util::get_content(path);
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").expect("regex invalid");

    for line in content {
        for capture in re.captures_iter(line.as_str()) {
            let amount: &i32 = &capture[1].parse().expect("could not parse amount");
            let from: &i32 = &capture[2].parse().expect("could not parse from");
            let to: &i32 = &capture[3].parse().expect("could not parse to");

            let from_m = arrangement.get_mut(from).expect("could not read from index");
            let mut result = String::from("");
            for _ in 0..*amount {
                if let Some(c) = from_m.pop() {
                    result.push(c);
                }
            }
            let to_m = arrangement.get_mut(to).expect("could not read from index");
            for _ in 0..result.len() {
                if let Some(c) = result.pop() {
                    to_m.push(c);
                }
            }
        }
    }

    let mut result = String::from("");
    for x in 1..=(arrangement.len() as i32) {
        if let Some(candidate) = arrangement.get_mut(&x) {
            if let Some(c) = candidate.pop() {
                result.push(c);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn find_total_score_a() {
        let mut setup = HashMap::from([
            (1, String::from("ZN")),
            (2, String::from("MCD")),
            (3, String::from("P")),
        ]);
        let result = day05a("./data/day05.txt", &mut setup);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn find_total_score_b() {
        let mut setup = HashMap::from([
            (1, String::from("ZN")),
            (2, String::from("MCD")),
            (3, String::from("P")),
        ]);
        let result = day05b("./data/day05.txt", &mut setup);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn find_total_score_part_a() {
        let mut setup = HashMap::from([
            (1, String::from("HCR")),
            (2, String::from("BJHLSF")),
            (3, String::from("RMDHJTQ")),
            (4, String::from("SGRHZBJ")),
            (5, String::from("RPFZTDCB")),
            (6, String::from("THCG")),
            (7, String::from("SNVZBPWL")),
            (8, String::from("RJQGC")),
            (9, String::from("LDTRHPFS")),
        ]);
        let result = day05a("./data/day05final.txt", &mut setup);
        assert_eq!(result, "SHQWSRBDL");
    }

    #[test]
    fn find_total_score_part_b() {
        let mut setup = HashMap::from([
            (1, String::from("HCR")),
            (2, String::from("BJHLSF")),
            (3, String::from("RMDHJTQ")),
            (4, String::from("SGRHZBJ")),
            (5, String::from("RPFZTDCB")),
            (6, String::from("THCG")),
            (7, String::from("SNVZBPWL")),
            (8, String::from("RJQGC")),
            (9, String::from("LDTRHPFS")),
        ]);
        let result = day05b("./data/day05final.txt", &mut setup);
        assert_eq!(result, "CDTQZHBRS");
    }
}
