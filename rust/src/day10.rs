extern crate utils;

use std::env;
use std::collections::HashMap;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Option<Bot>>;

#[derive(Clone, Copy)]
enum Target {
    Bot(usize),
    Output(usize)
}

#[derive(Clone, Copy)]
struct Instruction {
    low: Target,
    high: Target
}

#[derive(Clone)]
struct Bot {
    microchips: Vec<usize>,
    instruction: Option<Instruction>
}

fn solve(input: &Input, sought_responsibility: (usize, usize)) -> (usize, usize) {
    let mut bots = input.iter().map(|b| b.clone().unwrap()).collect::<Vec<_>>();
    let mut outputs: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut responsibilities: HashMap<(usize, usize), usize> = HashMap::new();

    let mut done = false;
    while !done {
        done = true;

        for idx in 0..bots.len() {
            if bots[idx].microchips.len() == 2 {

                if let Some(instruction) = bots[idx].instruction {
                    let low  = *bots[idx].microchips.iter().min().unwrap();
                    let high = *bots[idx].microchips.iter().max().unwrap();

                    for &(target, value) in [(instruction.low, low), (instruction.high, high)].iter() {
                        match target {
                            Target::Bot(bot_idx) => bots[bot_idx].microchips.push(value),
                            Target::Output(output_idx) => outputs.entry(output_idx).or_default().push(value)
                        }
                    }

                    bots[idx].microchips.clear();
                    responsibilities.insert((low, high), idx);
                    done = false;
                }
            }
        }
    }

    (
        *responsibilities.get(&sought_responsibility).unwrap_or(&0),
        [0, 1, 2].iter().map(|i| outputs.get(i).unwrap_or(&vec![0])[0]).fold(1, |acc, value| acc * value)
    )
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        let (part1, part2) = solve(&input, (17, 61));
        println!("Part1: {}", part1);
        println!("Part2: {}", part2);
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    let mut bots: Input = vec![];

    fn parse_target(name: &str, ref_idx: usize) -> Target {
        match name {
            "bot" => Target::Bot(ref_idx),
            "output" => Target::Output(ref_idx),
            _ => unreachable!()
        }
    }

    for line in reader.lines().map(|l| l.unwrap()) {
        let mut iter = line.split_ascii_whitespace();

        match iter.next() {
            Some("value") => {
                let value = iter.next().unwrap().parse::<usize>().unwrap();
                let bot_idx = iter.by_ref().skip(3).next().unwrap().parse::<usize>().unwrap();

                bots.resize(std::cmp::max(bots.len(), bot_idx + 1), None);
                if let Some(bot) = &bots[bot_idx] {
                    let mut microchips = bot.microchips.clone();
                    microchips.push(value);
                    bots[bot_idx] = Some(Bot { microchips, instruction: bot.instruction });
                } else {
                    bots[bot_idx] = Some(Bot { microchips: vec![value], instruction: None });
                }
            },
            Some("bot") => {
                let bot_idx = iter.next().unwrap().parse::<usize>().unwrap();
                let low  = parse_target(iter.by_ref().skip(3).next().unwrap(), iter.next().unwrap().parse::<usize>().unwrap());
                let high = parse_target(iter.by_ref().skip(3).next().unwrap(), iter.next().unwrap().parse::<usize>().unwrap());
                let instruction = Some(Instruction { low, high });

                bots.resize(std::cmp::max(bots.len(), bot_idx + 1), None);
                if let Some(bot) = &bots[bot_idx] {
                    bots[bot_idx] = Some(Bot { microchips: bot.microchips.clone(), instruction });
                } else {
                    bots[bot_idx] = Some(Bot { microchips: vec![], instruction });
                }
            },
            _ => unreachable!()
        }
    }

    Ok(bots)
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "value 5 goes to bot 2
        bot 2 gives low to bot 1 and high to bot 0
        value 3 goes to bot 1
        bot 1 gives low to output 1 and high to bot 0
        bot 0 gives low to output 2 and high to output 0
        value 2 goes to bot 2";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(solve(&as_input(INPUT), (2, 5)).0, 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve(&as_input(INPUT), (2, 5)).1, 30);
    }
}
