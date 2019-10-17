extern crate utils;

use std::env;
use std::collections::HashSet;
use std::str::FromStr;
use std::num::ParseIntError;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Movement>;

enum Turn {
    Left, Right
}

enum Direction {
    North, East, South, West
}

struct Movement {
    turn: Turn,
    steps: u32
}

impl FromStr for Movement {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let turn = match &s[0..1] {
            "L" => Turn::Left,
            "R" => Turn::Right,
            _ => unreachable!()
        };
        let steps = s[1..].parse::<u32>()?;
        Ok(Movement { turn, steps })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn origo() -> Position {
        Position { x: 0, y: 0 }
    }
    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn walk(direction: &Direction, p: &Position) -> Position {
    use Direction::*;
    match direction {
        North => Position { x: p.x, y: p.y + 1 },
        South => Position { x: p.x, y: p.y - 1 },
        East  => Position { x: p.x + 1, y: p.y },
        West  => Position { x: p.x - 1, y: p.y },
    }
}

fn solve(input: &Input) -> (i32, i32) {
    use Direction::*;
    const DIRECTIONS: [Direction; 4] = [North, East, South, West];

    fn next_dir_idx(dir_idx: usize, turn: &Turn) -> usize {
        let modifier = match turn {
            Turn::Right => 1,
            Turn::Left  => DIRECTIONS.len() - 1
        };
        (dir_idx + modifier) % DIRECTIONS.len()
    }

    let initial = (Position::origo(), 0, HashSet::new(), None);

    let (end_pos, _, _, hq_pos) = input.iter()
        .fold(initial, |(position, dir_idx, mut visited, mut hq_pos), Movement { turn, steps }| {

            let next_dir_idx = next_dir_idx(dir_idx, turn);
            let direction = &DIRECTIONS[next_dir_idx];

            let mut next_position = position;
            for _ in 0..*steps {
                next_position = walk(direction, &next_position);
                if hq_pos.is_none() && !visited.insert(next_position) {
                    hq_pos = Some(next_position);
                }
            }

            (next_position, next_dir_idx, visited, hq_pos)
        });

    (end_pos.distance(), hq_pos.map(|p| p.distance()).unwrap_or(0))
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    let f = BufReader::new(f);
    Ok(f.lines().next().unwrap()?.split(",").map(|m| m.trim().parse::<Movement>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_input(s: &str) -> Input {
        s.split(',').map(|m| m.trim().parse::<Movement>().unwrap()).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input("R2, L3")).0, 5);
        assert_eq!(solve(&as_input("R2, R2, R2")).0, 2);
        assert_eq!(solve(&as_input("R5, L5, R5, R3")).0, 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input("R8, R4, R4, R8")).1, 4);
    }
}
