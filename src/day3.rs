use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type GridPos = HashMap<(usize, usize), char>;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> (GridPos, GridPos) {
    let mut symbols = HashMap::new();
    let mut numbers = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                numbers.insert((x, y), c);
            } else if c != '.' {
                symbols.insert((x, y), c);
            }
        }
    }
    (numbers, symbols)
}

#[aoc(day3, part1)]
pub fn part1(input: &(GridPos, GridPos)) -> u32 {
    let (mut numbers, symbols) = input.clone();
    let mut res = vec![];
    for s in symbols {
        let (x, y) = s.0;
        for y in y - 1..=y + 1 {
            for x in x - 1..=x + 1 {
                if let Some(n) = numbers.remove(&(x, y)) {
                    let mut all_around_nb: Vec<(i32, char)> = vec![(x as i32, n)];
                    // Search backward
                    let mut search_x: i32 = x as i32 - 1;
                    while search_x >= 0 {
                        if let Some(search_n) = numbers.remove(&(search_x as usize, y)) {
                            all_around_nb.push((search_x, search_n));
                            search_x -= 1;
                        } else {
                            break;
                        }
                    }
                    // Search forward
                    search_x = x as i32 + 1;
                    while let Some(search_n) = numbers.remove(&(search_x as usize, y)) {
                        all_around_nb.push((search_x, search_n));
                        search_x += 1;
                    }
                    all_around_nb.sort_by(|a, b| a.0.cmp(&b.0));
                    let res_number: String = all_around_nb.into_iter().map(|(_, c)| c).collect();
                    res.push(res_number.parse::<u32>().unwrap());
                }
            }
        }
    }
    res.iter().sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &(GridPos, GridPos)) -> u32 {
    let (mut numbers, symbols) = input.clone();
    let mut res = vec![];
    for s in symbols {
        if s.1 != '*' {
            continue;
        }
        let mut gear_nb = vec![];
        let (x, y) = s.0;
        for y in y - 1..=y + 1 {
            for x in x - 1..=x + 1 {
                if let Some(n) = numbers.remove(&(x, y)) {
                    let mut all_around_nb: Vec<(i32, char)> = vec![(x as i32, n)];
                    // Search backward
                    let mut search_x: i32 = x as i32 - 1;
                    while search_x >= 0 {
                        if let Some(search_n) = numbers.remove(&(search_x as usize, y)) {
                            all_around_nb.push((search_x, search_n));
                            search_x -= 1;
                        } else {
                            break;
                        }
                    }
                    // Search forward
                    search_x = x as i32 + 1;
                    while let Some(search_n) = numbers.remove(&(search_x as usize, y)) {
                        all_around_nb.push((search_x, search_n));
                        search_x += 1;
                    }
                    all_around_nb.sort_by(|a, b| a.0.cmp(&b.0));
                    let res_number: String = all_around_nb.into_iter().map(|(_, c)| c).collect();
                    gear_nb.push(res_number.parse::<u32>().unwrap());
                }
            }
        }
        if gear_nb.len() == 2 {
            res.push(gear_nb[0] * gear_nb[1]);
        }
    }
    res.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    #[test]
    fn test_part1() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(part1(&input_generator(input)), 4361);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day3.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 540025);
    }

    #[test]
    fn test_part2() {
        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(part2(&input_generator(input)), 467835);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day3.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 84584891);
    }
}
