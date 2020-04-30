extern crate utils;
extern crate md5;

use std::env;
use std::str;
use std::collections::HashSet;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = String;

fn find_triplet_and_quintlets(s: &str) -> (Option<char>, HashSet<char>) {

    let mut triplet = None;
    let mut quintlets = HashSet::with_capacity(1);

    let mut last_char = ' ';
    let mut last_char_cnt = 0;
    for c in s.chars() {
        if c == last_char {
            last_char_cnt += 1;
            if last_char_cnt == 3 && triplet.is_none() {
                triplet = Some(c);
            } else if last_char_cnt == 5 {
                quintlets.insert(c);
            }
        } else {
            last_char = c;
            last_char_cnt = 1;
        }
    }

    (triplet, quintlets)
}

fn find_key_index(key_index: usize, salt: &str, key_stretch: usize) -> usize {
    let mut key_candidates = vec![];
    let mut keys = vec![];

    for i in 0.. {
        let seed = format!("{}{}", salt, i);
        let seed = seed.as_bytes();

        let mut digest = md5::compute(&seed);

        for _ in 0..key_stretch {
            let seed = format!("{:?}", digest);
            digest = md5::compute(&seed.as_bytes());
        }

        let digest_str = format!("{:?}", digest);
        let (triplet, quintlets) = find_triplet_and_quintlets(&digest_str[..]);

        for quintlet in &quintlets {
            for (idx, triplet) in &key_candidates {
                if i <= idx + 1000 && *triplet == *quintlet {
                    keys.push(*idx);
                }
            }
        }

        if keys.len() > key_index {
            break;
        }

        if let Some(triplet) = triplet {
            key_candidates.push((i, triplet));
        }
    }

    keys.sort();
    keys[key_index]
}

fn part1(input: &Input) -> usize {
    find_key_index(63, &input[..], 0)
}

fn part2(input: &Input) -> usize {
    find_key_index(63, &input[..], 2016)
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
       "abc";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 22728);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input(INPUT)), 22551);
    }
}
