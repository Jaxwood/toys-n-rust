use crate::util;

struct LineGroup {
    calories: Vec<i32>,
}

impl LineGroup {
    fn new() -> LineGroup {
        LineGroup { calories: vec![] }
    }

    fn sum(&self) -> i32 {
        self.calories.iter().sum()
    }
}

fn parse(path: &str) -> Vec<LineGroup> {
    let lines = util::get_content(path);
    let mut groups: Vec<LineGroup> = vec![];
    let mut group = LineGroup::new();
    for line in lines {
        if line.is_empty() {
            groups.push(group);
            group = LineGroup::new();
        } else {
            group.calories.push(line.parse().unwrap_or(0));
        }
    }
    groups.push(group);

    groups
}

fn day01a(path: &str) -> i32 {
    let groups = parse(path);

    let mut max = 0;
    for group in groups {
        if group.sum() > max {
            max = group.sum();
        }
    }
    max
}

fn day01b(path: &str) -> i32 {
    let groups = parse(path);
    let mut g: Vec<i32> = groups.iter().map(|x| x.sum()).collect();

    g.sort();
    g.reverse();
    g.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_calories() {
        let result = day01a("./data/day01.txt");
        assert_eq!(result, 24000);
    }

    #[test]
    fn find_three_most_calories() {
        let result = day01b("./data/day01.txt");
        assert_eq!(result, 45000);
    }

    #[test]
    fn find_most_calories_part_a() {
        let result = day01a("./data/day01final.txt");
        assert_eq!(result, 69310);
    }

    #[test]
    fn find_three_most_calories_part_b() {
        let result = day01b("./data/day01final.txt");
        assert_eq!(result, 206104);
    }
}
