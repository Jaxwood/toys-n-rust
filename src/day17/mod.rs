#![allow(dead_code)]

use core::fmt;
use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
    fs,
};

struct Chamber {
    bricks: HashSet<(i32, i32)>,

    height: i32,
    width: i32,

    position: (i32, i32),
}

#[derive(Debug)]
enum Brick {
    Straight(Vec<(i32, i32)>, i32, i32),
    Cross(Vec<(i32, i32)>, i32, i32),
    L(Vec<(i32, i32)>, i32, i32),
    I(Vec<(i32, i32)>, i32, i32),
    Square(Vec<(i32, i32)>, i32, i32),
}

impl Brick {
    fn height(&self) -> i32 {
        match self {
            Brick::Straight(_, height, _) => *height,
            Brick::Cross(_, height, _) => *height,
            Brick::L(_, height, _) => *height,
            Brick::I(_, height, _) => *height,
            Brick::Square(_, height, _) => *height,
        }
    }

    fn width(&self) -> i32 {
        match self {
            Brick::Straight(_, _, width) => *width,
            Brick::Cross(_, _, width) => *width,
            Brick::L(_, _, width) => *width,
            Brick::I(_, _, width) => *width,
            Brick::Square(_, _, width) => *width,
        }
    }

    fn bricks(&self) -> &Vec<(i32, i32)> {
        match self {
            Brick::Straight(bricks, _, _) => bricks,
            Brick::Cross(bricks, _, _) => bricks,
            Brick::L(bricks, _, _) => bricks,
            Brick::I(bricks, _, _) => bricks,
            Brick::Square(bricks, _, _) => bricks,
        }
    }
}

impl Chamber {
    fn left(&mut self, brick: &Brick) {
        let (x, y) = self.position;
        let candidates = brick
            .bricks()
            .iter()
            .map(|&(i, j)| (x + i - 1, y - j))
            .collect::<HashSet<_>>();
        if candidates.iter().any(|&(i, _)| i < 1) {
            return;
        }
        if self.bricks.intersection(&candidates).count() == 0 {
            self.position = (x - 1, y);
        }
    }

    fn right(&mut self, brick: &Brick) {
        let (x, y) = self.position;
        let candidates = brick
            .bricks()
            .iter()
            .map(|&(i, j)| (x + i + 1, y - j))
            .collect::<HashSet<_>>();
        if candidates.iter().any(|&(i, _)| i > 7) {
            return;
        }
        if self.bricks.intersection(&candidates).count() == 0 {
            self.position = (x + 1, y);
        }
    }

    fn down(&mut self, brick: &Brick) -> bool {
        let (x, y) = self.position;
        let candidates = brick
            .bricks()
            .iter()
            .map(|&(i, j)| (x + i, y - j - 1))
            .collect::<HashSet<_>>();
        if candidates.iter().any(|&(_, j)| j < 1) {
            return false;
        }
        if self.bricks.intersection(&candidates).count() == 0 {
            self.position = (x, y - 1);
            return true;
        }

        false
    }

    fn reset_position(&mut self, brick: &Brick) {
        // position is measured from the top left corner
        self.position = (3, self.height + brick.height() + 3);
    }

    fn store_brick(&mut self, brick: &mut Brick) {
        let (x, y) = self.position;
        let bricks = brick.bricks();
        for &(i, j) in bricks {
            self.bricks.insert((i + x, y - j));
        }
        self.height = self.bricks.iter().map(|&(_, j)| j).max().unwrap();
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for y in (0..=20).rev() {
            for x in 0..=8 {
                if (x == 0 || x == 8) && y == 0 {
                    s.push_str("+");
                    continue;
                } else if x == 0 || x == 8 {
                    s.push_str("|");
                    continue;
                } else if y == 0 {
                    s.push_str("-");
                    continue;
                }
                if self.bricks.contains(&(x, y)) {
                    s.push_str("#");
                } else {
                    s.push_str(".");
                }
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

fn load_bricks() -> Vec<Brick> {
    vec![
        Brick::Straight(vec![(0, 0), (1, 0), (2, 0), (3, 0)], 1, 4),
        Brick::Cross(vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 3, 3),
        Brick::L(vec![(2, 0), (2, 1), (2, 2), (1, 2), (0, 2)], 3, 3),
        Brick::I(vec![(0, 0), (0, 1), (0, 2), (0, 3)], 4, 1),
        Brick::Square(vec![(0, 0), (0, 1), (1, 0), (1, 1)], 2, 2),
    ]
}

fn day17a(path: &str, target: usize) -> i32 {
    let wind = fs::read_to_string(path)
        .expect("file not found")
        .trim()
        .to_string();

    let wind_len = wind.len();
    let mut bricks = load_bricks();
    let bricks_len = bricks.len();

    let mut chamber = Chamber {
        bricks: HashSet::new(),
        height: 0,
        width: 7,
        position: (2, 4),
    };

    let mut brick_idx: usize = 0;
    let mut wind_idx: usize = 0;

    while target > brick_idx {
        let brick = bricks.get_mut(brick_idx % bricks_len).unwrap();
        chamber.reset_position(brick);
        loop {
            let direction = wind.chars().nth(wind_idx % wind_len).unwrap();
            wind_idx += 1;
            match direction {
                '<' => chamber.left(brick),
                '>' => chamber.right(brick),
                _ => panic!("invalid wind direction"),
            };

            if !chamber.down(brick) {
                break;
            }
        }

        brick_idx += 1;
        chamber.store_brick(brick);
    }

    chamber.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_most_pressure() {
        let actual = day17a("./data/day17.txt", 2022);
        assert_eq!(actual, 3068);
    }

    #[test]
    fn find_most_pressure_part_a() {
        let actual = day17a("./data/day17final.txt", 2022);
        assert_eq!(actual, 3135);
    }
}
