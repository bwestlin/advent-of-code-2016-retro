extern crate utils;

use std::env;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;
use utils::*;
use CompressionFormat::*;

type Input = String;

enum CompressionFormat {
    V1, V2
}

fn decompress_len(input: &[char], format: &CompressionFormat) -> usize {
    let mut len = 0;
    let mut idx = 0;

    while idx < input.len() {

        if input[idx] == '(' {
            let mut marker = input[idx..].iter().skip(1).take_while(|&&c| c != ')');
            let n_chars = marker.by_ref().take_while(|&&c| c != 'x').collect::<String>();
            let n_times = marker.collect::<String>();

            let marker_len = n_chars.len() + n_times.len() + 3;

            let n_chars = n_chars.parse::<usize>().unwrap();
            let n_times = n_times.parse::<usize>().unwrap();

            let decompressed_len = match format {
                V1 => n_chars,
                V2 => {
                    let marked_chars = &input[(idx + marker_len)..(idx + marker_len + n_chars)];
                    decompress_len(marked_chars, format)
                }
            };

            len += decompressed_len * n_times;
            idx += marker_len + n_chars;
        } else {
            len += 1;
            idx += 1;
        }
    }

    len
}

fn part1(input: &Input) -> usize {
    decompress_len(&(input.chars().collect::<Vec<_>>()[..]), &V1)
}

fn part2(input: &Input) -> usize {
    decompress_len(&(input.chars().collect::<Vec<_>>()[..]), &V2)
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

    fn as_input(s: &str) -> Input {
        read_input(BufReader::new(s.split('\n').map(|s| s.trim()).collect::<Vec<_>>().join("\n").as_bytes())).unwrap()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&as_input("ADVENT")), "ADVENT".len());
        assert_eq!(part1(&as_input("A(1x5)BC")), "ABBBBBC".len());
        assert_eq!(part1(&as_input("(3x3)XYZ")), "XYZXYZXYZ".len());
        assert_eq!(part1(&as_input("A(2x2)BCD(2x2)EFG")), "ABCBCDEFEFG".len());
        assert_eq!(part1(&as_input("(6x1)(1x3)A")), "(1x3)A".len());
        assert_eq!(part1(&as_input("X(8x2)(3x3)ABCY")), "X(3x3)ABC(3x3)ABCY".len());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&as_input("(3x3)XYZ")), "XYZXYZXYZ".len());
        assert_eq!(part2(&as_input("X(8x2)(3x3)ABCY")), "XABCABCABCABCABCABCY".len());
        assert_eq!(part2(&as_input("(27x12)(20x12)(13x14)(7x10)(1x12)A")), 241920);
        assert_eq!(part2(&as_input("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")), 445);
    }
}
