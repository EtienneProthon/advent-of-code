use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> HashSet<(usize, usize, u32)> {
    let mut galaxies = HashSet::new();
    let mut galaxy_id = 1;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '#' {
                galaxies.insert((x, y, galaxy_id));
                galaxy_id += 1;
            }
        }
    }
    galaxies
}

fn expand_universe(
    input: &HashSet<(usize, usize, u32)>,
    expansion_factor: usize,
) -> Vec<(usize, usize, u32)> {
    let max_x = input.iter().map(|x| x.0).max().unwrap();
    let max_y = input.iter().map(|x| x.1).max().unwrap();
    let mut x_to_expand = vec![];
    for x in 0..max_x {
        if !input.iter().any(|g| g.0 == x) {
            x_to_expand.push(x);
        }
    }
    let mut y_to_expand = vec![];
    for y in 0..max_y {
        if !input.iter().any(|g| g.1 == y) {
            y_to_expand.push(y);
        }
    }
    let mut galaxies: Vec<_> = input.clone().into_iter().collect();

    for galaxy in galaxies.iter_mut() {
        let add_x = x_to_expand.iter().filter(|x| *x < &galaxy.0).count() * expansion_factor;
        let add_y = y_to_expand.iter().filter(|y| *y < &galaxy.1).count() * expansion_factor;
        *galaxy = (galaxy.0 + add_x, galaxy.1 + add_y, galaxy.2);
    }
    galaxies
}

fn compute_distances(galaxies: Vec<(usize, usize, u32)>) -> HashMap<(u32, u32), u64> {
    let mut res = HashMap::new();
    for g1 in &galaxies {
        for g2 in &galaxies {
            if g1.2 != g2.2 && !res.contains_key(&(g1.2, g2.2)) && !res.contains_key(&(g2.2, g1.2))
            {
                let distance =
                    ((g1.0 as i64 - g2.0 as i64).abs() + (g1.1 as i64 - g2.1 as i64).abs()) as u64;
                res.insert((g1.2, g2.2), distance);
            }
        }
    }
    res
}

#[aoc(day11, part1)]
pub fn part1(input: &HashSet<(usize, usize, u32)>) -> u64 {
    let galaxies = expand_universe(input, 1);
    let distances = compute_distances(galaxies);
    distances.values().sum()
}

#[aoc(day11, part2)]
pub fn part2(input: &HashSet<(usize, usize, u32)>) -> u64 {
    let galaxies = expand_universe(input, 1000000 - 1);
    let distances = compute_distances(galaxies);
    distances.values().sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT: &str = "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 374);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day11.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 10292708);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 82000210);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day11.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 790194712336);
    }
}
