use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Match {
    id: u32,
    win_nb: HashSet<u32>,
    my_nb: HashSet<u32>,
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Match> {
    let mut res = vec![];
    for (i, line) in input.lines().enumerate() {
        let (_, numbers) = line.split_once(':').unwrap();
        let (win_nb, my_nb) = numbers.split_once('|').unwrap();
        let win_nb = win_nb
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let my_nb = my_nb
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        res.push(Match {
            id: i as u32 + 1,
            win_nb,
            my_nb,
        });
    }
    res
}

#[aoc(day4, part1)]
pub fn part1(input: &[Match]) -> u32 {
    let mut res = 0;
    for m in input {
        let matching_nb = m.win_nb.intersection(&m.my_nb).count();
        res += if matching_nb > 0 {
            u32::pow(2, matching_nb as u32 - 1)
        } else {
            0
        };
    }
    res
}

#[aoc(day4, part2)]
pub fn part2(input: &[Match]) -> u32 {
    let mut results = input.iter().map(|x| (x.id, 1)).collect::<HashMap<_, _>>();
    for m in input {
        let matching_nb = m.win_nb.intersection(&m.my_nb).count();
        let current_card_nb = *results.get(&m.id).unwrap_or(&1);
        for i in m.id + 1..m.id + matching_nb as u32 + 1 {
            let mut nb = 0;
            if let Some(m_nb) = results.get(&i) {
                nb += m_nb + current_card_nb;
            };
            results.insert(i, nb);
        }
    }
    results.values().sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    #[test]
    fn test_part1() {
        let input ="Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(&input_generator(input)), 13);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day4.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 28750);
    }

    #[test]
    fn test_part2() {
        let input ="Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(&input_generator(input)), 30);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day4.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 10212704);
    }
}
