use std::collections::HashMap;
use std::fmt::Debug;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EVersion {
    V1,
    V2(u32),
}

#[derive(Eq, PartialEq, Clone)]
pub struct HandAndBid {
    hand: String,
    cards: Vec<(char, u32)>,
    bid: u32,
    version: EVersion,
}

impl Debug for HandAndBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -  {:?} - {:?}", self.hand, self.cards, self.version)
    }
}

pub fn card_to_digit(card: char, version: EVersion) -> u32 {
    match card {
        'T' => 10,
        'J' => match version {
            EVersion::V1 => 11,
            EVersion::V2(_) => 1,
        },
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap(),
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut x = 0;
        // PHASE 1
        // This mean the hand is full of J so it's a win for phase 1
        if self.cards.is_empty() || other.cards.is_empty() {
            if self.cards.is_empty() && other.cards.len() > 1 {
                return std::cmp::Ordering::Greater;
            }
            if other.cards.is_empty() && self.cards.len() > 1 {
                return std::cmp::Ordering::Less;
            }
        } else {
            while x < self.cards.len() {
                let mut self_card_count = self.cards.get(x).map(|x| x.1).unwrap_or(1);
                let mut other_card_count = other.cards.get(x).map(|x| x.1).unwrap_or(1);
                if x == 0 {
                    if let EVersion::V2(j_count) = self.version {
                        self_card_count += j_count;
                    }
                    if let EVersion::V2(j_count) = other.version {
                        other_card_count += j_count;
                    }
                }
                if self_card_count > other_card_count {
                    return std::cmp::Ordering::Greater;
                }
                if other_card_count > self_card_count {
                    return std::cmp::Ordering::Less;
                }
                if self_card_count == other_card_count {
                    x += 1;
                }
            }
        }
        // PHASE 2
        // If we ge there that mean two hands have same card numbers so we compare cards
        x = 0;
        while x < self.hand.len() {
            let self_card = self.hand.chars().nth(x).unwrap();
            let other_card = other.hand.chars().nth(x).unwrap();
            let self_card = card_to_digit(self_card, self.version);
            let other_card = card_to_digit(other_card, self.version);
            if self_card > other_card {
                return std::cmp::Ordering::Greater;
            }
            if other_card > self_card {
                return std::cmp::Ordering::Less;
            }
            if self_card == other_card {
                x += 1;
            }
        }

        // This should be impossible
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day7, part1)]
fn input_generator_part1(input: &str) -> Vec<HandAndBid> {
    let mut res = vec![];
    for line in input.lines() {
        let (hand, bid) = line.trim().split_once(' ').unwrap();
        let mut card_counts: HashMap<char, u32> = HashMap::new();
        for c in hand.chars() {
            *card_counts.entry(c).or_insert(0) += 1;
        }
        let mut card_counts: Vec<(char, u32)> = card_counts.into_iter().collect();
        card_counts.sort_by(|a, b| b.1.cmp(&a.1));
        res.push(HandAndBid {
            hand: hand.to_string(),
            cards: card_counts,
            bid: bid.parse().unwrap(),
            version: EVersion::V1,
        });
    }
    res
}

#[aoc_generator(day7, part2)]
fn input_generator_part2(input: &str) -> Vec<HandAndBid> {
    let mut res = vec![];
    for line in input.lines() {
        let (hand, bid) = line.trim().split_once(' ').unwrap();
        let mut card_counts: HashMap<char, u32> = HashMap::new();
        for c in hand.chars() {
            *card_counts.entry(c).or_insert(0) += 1;
        }
        let j_count = card_counts.remove(&'J').unwrap_or(0);
        let mut card_counts: Vec<(char, u32)> = card_counts.into_iter().collect();
        card_counts.sort_by(|a, b| b.1.cmp(&a.1));
        res.push(HandAndBid {
            hand: hand.to_string(),
            cards: card_counts,
            bid: bid.parse().unwrap(),
            version: EVersion::V2(j_count),
        });
    }
    res
}

#[aoc(day7, part1)]
pub fn part1(input: &[HandAndBid]) -> u32 {
    let mut res = input.to_vec();
    res.sort();
    res.into_iter()
        .enumerate()
        .map(|(i, HandAndBid { bid, .. })| (i as u32 + 1) * bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[HandAndBid]) -> u32 {
    let mut res = input.to_vec();
    res.sort();
    res.into_iter()
        .enumerate()
        .map(|(i, HandAndBid { bid, .. })| (i as u32 + 1) * bid)
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator_part1, input_generator_part2, part1, part2};
    use std::fs;

    const INPUT: &str = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator_part1(INPUT)), 6440);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day7.txt").unwrap();
        assert_eq!(part1(&input_generator_part1(&input)), 251287184);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator_part2(INPUT)), 5905);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day7.txt").unwrap();
        assert_eq!(part2(&input_generator_part2(&input)), 250757288);
    }
}
