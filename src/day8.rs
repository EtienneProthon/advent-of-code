use std::fmt::Debug;
use std::{collections::HashMap, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

pub type Instructons = HashMap<String, (String, String)>;
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EDirection {
    Left,
    Right,
}

impl FromStr for EDirection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(EDirection::Left),
            "R" => Ok(EDirection::Right),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> (Vec<EDirection>, Instructons) {
    let mut res = HashMap::new();
    let (directions, instructions) = input.split_once("\n\n").unwrap();
    let directions = directions
        .chars()
        .map(|x| EDirection::from_str(x.to_string().as_str()).unwrap())
        .collect();
    for line in instructions.lines() {
        let (source, destination) = line.trim().split_once('=').unwrap();
        let destination = destination.replace('(', "");
        let destination = destination.replace(')', "");
        let (d_left, d_right) = destination.split_once(',').unwrap();
        res.insert(
            source.trim().to_string(),
            (d_left.trim().to_string(), d_right.trim().to_string()),
        );
    }
    (directions, res)
}

#[aoc(day8, part1)]
pub fn part1(input: &(Vec<EDirection>, Instructons)) -> u32 {
    let (directions, instructions) = input;
    let mut current_position = "AAA";
    let mut step = 0;
    while current_position != "ZZZ" {
        let instruction = instructions.get(current_position).unwrap();
        let direction = directions[step % directions.len()];
        match direction {
            EDirection::Left => {
                current_position = &instruction.0;
            }
            EDirection::Right => {
                current_position = &instruction.1;
            }
        }
        step += 1;
    }
    step as u32
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

#[aoc(day8, part2)]
pub fn part2(input: &(Vec<EDirection>, Instructons)) -> u64 {
    let (directions, instructions) = input;
    let start_positions: Vec<&String> = instructions.keys().filter(|k| k.ends_with('A')).collect();
    let mut res_steps = vec![];
    for pos in start_positions {
        let mut current_position = pos;
        let mut step = 0;
        while !current_position.ends_with('Z') {
            let direction = directions[step % directions.len()];
            let instruction = instructions.get(current_position.as_str()).unwrap();
            match direction {
                EDirection::Left => {
                    current_position = &instruction.0;
                }
                EDirection::Right => {
                    current_position = &instruction.1;
                }
            }
            step += 1;
        }
        res_steps.push(step as u64);
    }
    // Find lowest common multiple of res_steps
    res_steps.into_iter().reduce(lcm).unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT_1: &str = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &str = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";

    const INPUT_3: &str = "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 2);
        assert_eq!(part1(&input_generator(INPUT_2)), 6);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day8.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 16897);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_3)), 6);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day8.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 16563603485021);
    }
}
