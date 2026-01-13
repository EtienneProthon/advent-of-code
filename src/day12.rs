use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ESpringState {
    Operational,
    Damaged,
    Unknown,
}

impl FromStr for ESpringState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(ESpringState::Operational),
            "#" => Ok(ESpringState::Damaged),
            "?" => Ok(ESpringState::Unknown),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(Vec<ESpringState>, Vec<u32>)> {
    let mut res = vec![];
    for line in input.lines() {
        let (springs_str, rules) = line.trim().split_once(" ").unwrap();
        let mut springs = vec![];
        for c in springs_str.chars() {
            springs.push(c.to_string().parse().unwrap());
        }
        let rules = rules.split(",").map(|x| x.parse().unwrap()).collect();
        res.push((springs, rules))
    }
    res
}

struct ProcessCount {
    springs: Vec<ESpringState>,
    rules: Vec<u32>,
}

impl ProcessCount {
    fn new(springs: Vec<ESpringState>, rules: Vec<u32>) -> Self {
        ProcessCount { springs, rules }
    }
    fn count(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if i == self.rules.len() {
            return if j == self.springs.len() { 1 } else { 0 };
        }
        if let Some(v) = cache[i][j] {
            return v;
        }
        let res = match self.springs[i] {
            ESpringState::Operational => self.count(cache, i + 1, j),
            ESpringState::Damaged => self.count_hash(cache, i, j),
            ESpringState::Unknown => self.count(cache, i + 1, j) + self.count_hash(cache, i, j),
        };
        cache[i][j] = Some(res);
        res
    }

    fn count_hash(&self, cache: &mut Vec<Vec<Option<usize>>>, i: usize, j: usize) -> usize {
        if j == self.springs.len() {
            return 0;
        }
        let end_group_idx = i + self.springs[j];
        if !self.is_rule_possible(i, end_group_idx) {
            return 0;
        }
        if end_group_idx == self.pattern.len() {
            return if j == self.springs.len() - 1 { 1 } else { 0 };
        }
        self.count(cache, end_group_idx + 1, j + 1)
    }

    fn is_rule_possible(&self, from: usize, to: usize) -> bool {
        match to.cmp(&self.springs.len()) {
            Greater => false,
            Equal => self.springs[from..to]
                .iter()
                .all(|&b| b != ESpringState::Operational),
            Less => {
                self.springs[from..to]
                    .iter()
                    .all(|&b| b != ESpringState::Operational)
                    && self.springs[to] != ESpringState::Damaged
            }
        }
    }
}

fn broken_group_possible(&self, from: usize, to: usize) -> bool {
    match to.cmp(&self.pattern.len()) {
        Greater => false,
        Equal => self.pattern[from..to].iter().all(|&b| b != b'.'),
        Less => self.pattern[from..to].iter().all(|&b| b != b'.') && self.pattern[to] != b'#',
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Vec<(Vec<ESpringState>, Vec<u32>)>) -> u64 {
    let mut res = 0;
    for row in input {
        let (springs, rules) = row;
    }
    res
}

#[aoc(day12, part2)]
pub fn part2(input: &Vec<(Vec<ESpringState>, Vec<u32>)>) -> u64 {
    0
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT: &str = "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 21);
    }

    // #[test]
    // fn test_part1_input() {
    //     let input = fs::read_to_string("input/2023/day12.txt").unwrap();
    //     assert_eq!(part1(&input_generator(&input)), 10292708);
    // }
    //
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input_generator(INPUT)), 82000210);
    // }
    //
    // #[test]
    // fn test_part2_input() {
    //     let input = fs::read_to_string("input/2023/day12.txt").unwrap();
    //     assert_eq!(part2(&input_generator(&input)), 790194712336);
    // }
}
