extern crate utils;

use std::env;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = u32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Pos {
    x: u32,
    y: u32
}

impl Pos {
    fn adjacent(&self) -> Vec<Pos> {
        [(-1i32, 0i32), (0, -1), (1, 0), (0, 1)].iter()
            .filter(|(mx, my)| !((self.x as i32 + mx) < 0) && !((self.y as i32 + my) < 0))
            .map(|(mx, my)| Pos { x: (self.x as i32 + mx) as u32, y: (self.y as i32 + my) as u32 })
            .collect()
    }
}

#[cfg(test)]
const TARGET_POS: Pos = Pos { x: 7, y: 4 };
#[cfg(not(test))]
const TARGET_POS: Pos = Pos { x: 31, y: 39 };

fn is_open(pos: &Pos, magic_number: &u32) -> bool {
    let &Pos { x, y } = pos;
    let sum = (x * x + 3 * x + 2 * x * y + y + y * y) + magic_number;
    sum.count_ones() % 2 == 0
}

fn solve(input: &Input) -> (usize, usize) {
    let mut queue = VecDeque::new();
    queue.push_front((Pos { x: 1, y: 1 }, 0));
    let mut visited = HashSet::new();
    let mut visited_50_steps = HashSet::new();

    while let Some((pos, steps)) = queue.pop_front() {

        if pos == TARGET_POS {
            return (steps, visited_50_steps.len());
        }

        if steps <= 50 {
            visited_50_steps.insert(pos);
        }

        visited.insert(pos);

        for adj in pos.adjacent() {
            if !is_open(&adj, input) || visited.contains(&adj) {
                continue;
            }

            queue.push_back((adj, steps + 1));
        }
    }
    (0, 0)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().next().unwrap()?.parse::<u32>().unwrap())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "10";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, 11);
    }
}
