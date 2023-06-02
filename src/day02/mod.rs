use crate::util;

#[derive(Debug)]
struct Round {
    player1: Choice,
    player2: Choice,
}

impl Round {
    fn score(&self) -> usize {
        match (&self.player1, &self.player2) {
            (Choice::Rock, Choice::Rock) => 1 + 3,
            (Choice::Rock, Choice::Paper) => 2 + 6,
            (Choice::Rock, Choice::Scissor) => 3 + 0,
            (Choice::Paper, Choice::Rock) => 1 + 0,
            (Choice::Paper, Choice::Paper) => 2 + 3,
            (Choice::Paper, Choice::Scissor) => 3 + 6,
            (Choice::Scissor, Choice::Rock) => 1 + 6,
            (Choice::Scissor, Choice::Paper) => 2 + 0,
            (Choice::Scissor, Choice::Scissor) => 3 + 3,
        }
    }

    fn calculate(&self) -> usize {
        match (&self.player1, &self.player2) {
            (Choice::Rock, Choice::Rock) => 3 + 0,
            (Choice::Rock, Choice::Paper) => 1 + 3,
            (Choice::Rock, Choice::Scissor) => 2 + 6,
            (Choice::Paper, Choice::Rock) => 1 + 0,
            (Choice::Paper, Choice::Paper) => 2 + 3,
            (Choice::Paper, Choice::Scissor) => 3 + 6,
            (Choice::Scissor, Choice::Rock) => 2 + 0,
            (Choice::Scissor, Choice::Paper) => 3 + 3,
            (Choice::Scissor, Choice::Scissor) => 1 + 6,
        }
    }
}

#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

fn parse(lines: Vec<String>) -> Vec<Round> {
    let mut result: Vec<Round> = vec![];
    for line in lines {
        let res = match line.as_str() {
            "A X" => Round{player1: Choice::Rock, player2: Choice::Rock},
            "A Y" => Round{player1: Choice::Rock, player2: Choice::Paper},
            "A Z" => Round{player1: Choice::Rock, player2: Choice::Scissor},
            "B X" => Round{player1: Choice::Paper, player2: Choice::Rock},
            "B Y" => Round{player1: Choice::Paper, player2: Choice::Paper},
            "B Z" => Round{player1: Choice::Paper, player2: Choice::Scissor},
            "C X" => Round{player1: Choice::Scissor, player2: Choice::Rock},
            "C Y" => Round{player1: Choice::Scissor, player2: Choice::Paper},
            "C Z" => Round{player1: Choice::Scissor, player2: Choice::Scissor},
            _ => panic!("unmatched combination"),
        };
        result.push(res);
    }
    result
}

fn day02a(path: &str) -> usize {
    let content = util::get_content(path);
    let rounds = parse(content);
    rounds.iter().map(|x| x.score()).sum()
}

fn day02b(path: &str) -> usize {
    let content = util::get_content(path);
    let rounds = parse(content);
    rounds.iter().map(|x| x.calculate()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_total_score_a() {
        let result = day02a("./data/day02.txt");
        assert_eq!(result, 15);
    }

    #[test]
    fn find_total_score_b() {
        let result = day02b("./data/day02.txt");
        assert_eq!(result, 12);
    }

    #[test]
    fn find_total_score_parta() {
        let result = day02a("./data/day02final.txt");
        assert_eq!(result, 10404);
    }

    #[test]
    fn find_total_score_partb() {
        let result = day02b("./data/day02final.txt");
        assert_eq!(result, 10334);
    }
}

