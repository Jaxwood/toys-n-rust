#![allow(dead_code)]

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs, iter,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::map,
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pixel {
    Blizzard(Direction),
    Rock,
    Open,
}

struct MapInfo {
    rows: usize,
    cols: usize,
    walls: HashSet<Coord>,
    blizzard_maps: HashMap<usize, HashSet<Coord>>,
    repeats_at: usize,
}

#[derive(PartialEq, Eq)]
struct Node {
    cost: usize,
    pos: Coord,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_map(input: &str) -> IResult<&str, Vec<Pixel>> {
    let (input, map) = many1(alt((
        map(tag("."), |_| Pixel::Open),
        map(tag("#"), |_| Pixel::Rock),
        map(tag("^"), |_| Pixel::Blizzard(Direction::Up)),
        map(tag("v"), |_| Pixel::Blizzard(Direction::Down)),
        map(tag("<"), |_| Pixel::Blizzard(Direction::Left)),
        map(tag(">"), |_| Pixel::Blizzard(Direction::Right)),
    )))(input)?;

    Ok((input, map))
}

fn parse(input: &str) -> IResult<&str, (usize, usize, HashMap<Coord, Pixel>)> {
    let (input, map) = separated_list1(newline, parse_map)(input)?;

    let width = map[0].len();
    let height = map.len();

    let mut result = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            result.insert(Coord { col: x, row: y }, pixel.clone());
        }
    }

    Ok((input, (width, height, result)))
}

impl Coord {
    fn neighbours(&self, rows: usize, cols: usize) -> Vec<Self> {
        use Direction::*;
        let mut neighbours = Vec::new();
        if self.row > 0 {
            neighbours.push(self.add_dir(&Up));
        }
        if self.col < cols - 1 {
            neighbours.push(self.add_dir(&Right));
        }
        if self.row < rows - 1 {
            neighbours.push(self.add_dir(&Down));
        }
        if self.col > 0 {
            neighbours.push(self.add_dir(&Left));
        }
        neighbours
    }

    fn add_dir(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            Up => Coord {
                row: self.row - 1,
                col: self.col,
            },
            Right => Coord {
                row: self.row,
                col: self.col + 1,
            },
            Down => Coord {
                row: self.row + 1,
                col: self.col,
            },
            Left => Coord {
                row: self.row,
                col: self.col - 1,
            },
        }
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn bliz_maps(
    map: &HashMap<Coord, Pixel>,
    rows: usize,
    cols: usize,
    max_time: usize,
) -> HashMap<usize, HashSet<Coord>> {
    let mut cache = HashMap::new();

    let mut blizzards: Vec<(Coord, Direction)> = map
        .iter()
        .filter_map(|(pos, pixel)| match pixel {
            Pixel::Rock => None,
            Pixel::Blizzard(dir) => Some((*pos, *dir)),
            Pixel::Open => None,
        })
        .collect();

    let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
    cache.insert(0, coords);

    for time in 1..max_time {
        for (coord, dir) in blizzards.iter_mut() {
            *coord = coord.add_dir(dir);
            match dir {
                Direction::Left => {
                    if coord.col == 0 {
                        coord.col = cols - 2;
                    }
                }
                Direction::Right => {
                    if coord.col == cols - 1 {
                        coord.col = 1;
                    }
                }
                Direction::Up => {
                    if coord.row == 0 {
                        coord.row = rows - 2;
                    }
                }
                Direction::Down => {
                    if coord.row == rows - 1 {
                        coord.row = 1;
                    }
                }
            }
        }
        let coords = blizzards.iter().map(|(coord, _)| *coord).collect();
        cache.insert(time, coords);
    }

    cache
}

fn shortest(from: Coord, to: Coord, start_time: usize, map_info: &MapInfo) -> usize {
    let MapInfo {
        rows,
        cols,
        walls,
        blizzard_maps,
        repeats_at,
    } = map_info;
    let mut pq = BinaryHeap::new();
    // backtracking is allowed, keep track of visited coords at a certain time
    let mut seen = HashSet::new();

    pq.push(Node {
        cost: start_time,
        pos: from,
    });
    seen.insert((from, start_time));

    // keep stepping through time until the priority queue is empty
    while let Some(Node { cost, pos }) = pq.pop() {
        // did we pop a node that's at the target position? It's guaranteed to be the shortest path
        if pos == to {
            return cost;
        }

        let new_cost = cost + 1;
        let blizzards = &blizzard_maps[&(new_cost % repeats_at)];

        let candidates = pos
            // moving to a neighbour is an option
            .neighbours(*rows, *cols)
            .into_iter()
            // not moving is an option
            .chain(iter::once(pos))
            // can not share a coordinate with a wall
            .filter(|coord| !walls.contains(coord))
            // can not share a coordinate with a blizzard
            .filter(|coord| !blizzards.contains(coord));

        for new_pos in candidates {
            // only push to pq if we didn't already see that coord at the same time
            if seen.insert((new_pos, new_cost)) {
                pq.push(Node {
                    cost: new_cost,
                    pos: new_pos,
                });
            }
        }
    }
    usize::MAX
}

fn day24a(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, (cols, rows, map)) = parse(&input).unwrap();

    let walls: HashSet<Coord> = map
        .iter()
        .filter(|(_, tile)| **tile == Pixel::Rock)
        .map(|(pos, _)| *pos)
        .collect();

    let lcm = lcm(rows - 2, cols - 2);
    let blizzard_maps = bliz_maps(&map, rows, cols, lcm);
    let start = Coord { row: 0, col: 1 };
    let end = Coord {
        row: rows - 1,
        col: cols - 2,
    };

    let map_info = MapInfo {
        rows,
        cols,
        repeats_at: lcm,
        walls,
        blizzard_maps,
    };

    shortest(start, end, 0, &map_info)
}

pub fn day24b(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let (_, (cols, rows, map)) = parse(&input).unwrap();

    let walls: HashSet<Coord> = map
        .iter()
        .filter(|(_, tile)| **tile == Pixel::Rock)
        .map(|(pos, _)| *pos)
        .collect();

    let lcm = lcm(rows - 2, cols - 2);
    let blizzard_maps = bliz_maps(&map, rows, cols, lcm);
    let start = Coord { row: 0, col: 1 };
    let end = Coord {
        row: rows - 1,
        col: cols - 2,
    };

    let map_info = MapInfo {
        rows,
        cols,
        repeats_at: lcm,
        walls,
        blizzard_maps,
    };

    let there = shortest(start, end, 0, &map_info);
    let back = shortest(end, start, there, &map_info);
    shortest(start, end, back, &map_info)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_shortest_path() {
        let actual = day24a("./data/day24.txt");
        assert_eq!(actual, 18);
    }

    #[test]
    fn find_shortest_path_when_going_back() {
        let actual = day24b("./data/day24.txt");
        assert_eq!(actual, 54);
    }

    #[test]
    fn find_shortest_path_part_a() {
        let actual = day24a("./data/day24final.txt");
        assert_eq!(actual, 269);
    }

    #[test]
    fn find_shortest_path_part_b() {
        let actual = day24b("./data/day24final.txt");
        assert_eq!(actual, 825);
    }
}
