extern crate utils;

use std::env;
use std::cmp;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<String>;

type Coord = [i32; 2];
type Axis = usize;
type BoundsFn = dyn Fn(&Coord, usize) -> (i32, i32);
type KeyFn = dyn Fn(&Coord) -> char;

fn find_code(input: &Input, coord: &Coord, bounds: &BoundsFn, key: &KeyFn) -> String {
    let mut code = String::with_capacity(input.len());
    let mut coord = coord.clone();

    for instruction in input {
        for c in instruction.chars() {
            let (axis, delta) = match c {
                'R' => (0,  1),
                'L' => (0, -1),
                'D' => (1,  1),
                'U' => (1, -1),
                _ => unreachable!()
            };
            let (min, max) = bounds(&coord, axis);
            coord[axis] = cmp::max(min, cmp::min(max, coord[axis] + delta));
        }
        code.push(key(&coord));
    }

    code
}

fn part1(input: &Input) -> String {

    let bounds = |_: &_, _: _| {
        (1, 3)
    };

    let key = |coord: &Coord| {
        let (x, y) = (coord[0], coord[1]);
        let key = x + (y - 1) * 3;
        ('0' as u8 + key as u8) as char
    };

    find_code(input, &[2, 2], &bounds, &key)
}

fn part2(input: &Input) -> String {
    const KEYS: [char; 13] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D'];

    let bounds = |coord: &Coord, axis: Axis| {
        let other_axis = (axis + 1) % 2;
        let factor = coord[other_axis].abs();
        (-2 + factor, 2 - factor)
    };

    let key = |coord: &Coord| {
        let (x, y) = (coord[0], coord[1]);
        let key_idx = (6 + (y + (4 - y.abs()) * y) + x) as usize;
        KEYS[key_idx]
    };

    find_code(input, &[-2, 0], &bounds, &key)
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| l.unwrap()).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "ULL
        RRDDD
        LURDL
        UUUUD";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), "1985");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), "5DB3");
    }
}
