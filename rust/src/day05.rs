extern crate utils;
extern crate md5;
extern crate rayon;
extern crate num_cpus;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use rayon::prelude::*;
use utils::*;

type Input = String;

fn password_matches(input: &Input, range: std::ops::Range<usize>) -> Vec<(usize, usize, usize)> {
    const PREFIX: [u8; 3] = [0xFF, 0xFF, 0xF0];
    let mut matches = vec![];

    let mut seed: Vec<u8> = vec![];
    seed.extend_from_slice(format!("{}{}", input, range.start).as_bytes());
    let start = range.start;

    'outer: for index in range {

        // Inrement seed
        if index > start {
            let mut remainder = 1;
            for i in 0..(seed.len() - input.len()) {
                let s_idx = seed.len() - 1 - i;
                if seed[s_idx] == '9' as u8 {
                    seed[s_idx] -= 9;
                } else {
                    seed[s_idx] += 1;
                    remainder = 0;
                    break;
                }
            }
            if remainder != 0 {
                seed.insert(input.len(), '1' as u8);
            }
        }

        let digest = md5::compute(&seed);

        for i in 0..PREFIX.len() {
            if digest[i] & PREFIX[i] != 0 {
                continue 'outer;
            }
        }

        matches.push((index, (digest[2] & 0x0F) as usize, ((digest[3] >> 4) & 0x0F) as usize));
    }

    matches
}

fn solve2(input: &Input) -> (String, String) {
    const HEX: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    let n_per_thread = 100000;
    let n_threads = num_cpus::get();

    let mut password1 = String::new();
    let mut password2: Vec<_> = (0..8).map(|_| '_').collect();

    'outer: for i in (1..).step_by(n_per_thread * n_threads) {
        let ranges: Vec<_> = (0..n_threads)
            .map(|ti| (i + (ti * n_per_thread))..(i + (ti * n_per_thread) + n_per_thread))
            .collect();

        let mut matches: Vec<_> = ranges.into_par_iter()
            .flat_map(|r| password_matches(input, r))
            .collect();

        matches.sort_unstable_by(|(a, _, _), (b, _, _)| a.cmp(b));

        for (_, nibble6, nibble7) in matches {
            if password1.len() < 8 {
                password1.push(HEX[nibble6]);
            }

            if nibble6 < 8 && password2[nibble6] == '_' {
                password2[nibble6] = HEX[nibble7];
                if !password2.contains(&'_') {
                    break 'outer;
                }
            }
        }
    }

    (password1, password2.iter().collect())
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve2(&input);
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().next().unwrap().unwrap())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "abc";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve2(&as_input(INPUT)).0, "18f47a30".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve2(&as_input(INPUT)).1, "05ace8e3".to_string());
    }
}
