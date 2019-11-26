extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<String>;

fn solve(input: &Input) -> (String, String) {
    const MESSAGE_LEN: usize = 6;

    let (p1, p2): (Vec<_>, Vec<_>) = input.iter()
        .fold(vec![[0_u8; 256]; MESSAGE_LEN], |mut c_counts, message| {
            let bytes = message.as_bytes();
            for bi in 0..MESSAGE_LEN {
                c_counts[bi][bytes[bi] as usize] += 1;
            }
            c_counts
        })
        .iter()
        .map(|counts| (
            counts.iter().enumerate()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(i, _)| i as u8)
                .unwrap() as char,
            counts.iter().enumerate()
                .filter(|(_, &c)| c > 0)
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(i, _)| i as u8)
                .unwrap() as char
        ))
        .unzip();

    (p1.iter().collect(), p2.iter().collect())
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
       "eedadn
        drvtee
        eandsr
        raavrd
        atevrs
        tsrnev
        sdttsa
        rasrtv
        nssdts
        ntnada
        svetve
        tesnvt
        vntsnd
        vrdear
        dvrsen
        enarar";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT)).0, "easter".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT)).1, "advent".to_string());
    }
}
