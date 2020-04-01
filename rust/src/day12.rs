extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;

type Input = Vec<Instruction>;

#[derive(Clone, Debug)]
enum Instruction {
    Cpy(Source, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Source, i32)
}

#[derive(Clone, Copy, Debug)]
enum Register {
    A, B, C, D
}

#[derive(Clone, Debug)]
enum Source {
    Register(Register), Value(i32)
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [i32; 4],
    pc: usize
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["cpy", a, b] => Instruction::Cpy(Source::parse(a), Register::parse(b)),
            ["inc", a]    => Instruction::Inc(Register::parse(a)),
            ["dec", a]    => Instruction::Dec(Register::parse(a)),
            ["jnz", a, b] => Instruction::Jnz(Source::parse(a), b.parse::<i32>().unwrap()),
            _ => unreachable!()
        }
    }
}

impl Register {
    fn parse(s: &str) -> Register {
        match s {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => unreachable!()
        }
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        self as usize
    }
}

impl Source {
    fn parse(s: &str) -> Source {
        s.parse::<i32>()
            .map(|v| Source::Value(v))
            .unwrap_or_else(|_| Source::Register(Register::parse(s)))
    }
}

impl Computer {
    fn new() -> Self {
        Computer { registers: [0; 4], pc: 0 }
    }

    fn read(&self, s: &Source) -> i32 {
        match s {
            Source::Register(r) => self.registers[*r as usize],
            Source::Value(v) => *v
        }
    }

    fn run(&mut self, program: &Vec<Instruction>) {

        while self.pc < program.len() {
            let ins = &program[self.pc];
            self.pc += 1;

            match ins {
                Instruction::Cpy(src, r) =>
                    self.registers[*r as usize] = self.read(src),
                Instruction::Inc(r) =>
                    self.registers[*r as usize] += 1,
                Instruction::Dec(r) =>
                    self.registers[*r as usize] -= 1,
                Instruction::Jnz(src, offset) => {
                    if self.read(src) != 0 {
                        self.pc = ((self.pc as i32) - 1 + offset) as usize;
                    }
                }
            }
        }
    }
}

fn part1(input: &Input) -> i32 {
    let mut c = Computer::new();
    c.run(input);
    c.registers[0]
}

fn part2(input: &Input) -> i32 {
    let mut c = Computer::new();
    c.registers[Register::C as usize] = 1;
    c.run(input);
    c.registers[0]
}

fn main() {
    measure(|| {
        let input = input().expect("Input failed");
        println!("Part1: {}", part1(&input));
        println!("Part2: {}", part2(&input));
    });
}

fn read_input<R: Read>(reader: BufReader<R>) -> io::Result<Input> {
    Ok(reader.lines().map(|l| Instruction::parse(l.unwrap().as_str())).collect())
}

fn input() -> io::Result<Input> {
    let f = File::open(env::args().skip(1).next().expect("No input file given"))?;
    read_input(BufReader::new(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str =
       "cpy 41 a
        inc a
        inc a
        dec a
        jnz a 2
        dec a";

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input(INPUT)), 42);
    }
}
