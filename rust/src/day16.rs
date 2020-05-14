extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

fn fill_disk(initial: &str, length: usize) -> String {
    let mut buffer = String::with_capacity(length);
    buffer.push_str(initial);

    while buffer.len() < length {
        let tail = buffer
            .chars()
            .rev()
            .map(|c| if c == '1' { '0' } else { '1' })
            .collect::<String>();

        buffer.push('0');
        buffer.push_str(&tail);
    }

    buffer.truncate(length);
    buffer
}

fn checksum(data: &str) -> String {
    let mut buffer = data.chars().collect::<Vec<_>>();

    while buffer.len() % 2 == 0 {
        buffer = buffer
            .chunks_exact(2)
            .map(|cs| if cs[0] == cs[1] { '1' } else { '0' })
            .collect();
    }

    buffer.iter().collect::<String>()
}

fn part1(input: &Input) -> String {
    checksum(&fill_disk(input, 272))
}

fn part2(input: &Input) -> String {
    checksum(&fill_disk(input, 35651584))
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
       "10000";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_fill_disk() {
        assert_eq!(fill_disk(&as_input(INPUT), 20), "10000011110010000111".to_string());
    }

    #[test]
    fn test_checksum() {
        assert_eq!(checksum("110010110100"), "100");
        assert_eq!(checksum("10000011110010000111"), "01100");
    }
}
