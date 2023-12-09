use aoc_runner_derive::{aoc, aoc_generator};

pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn is_possible(&self, bag: &Set) -> bool {
        for set in &self.sets {
            if set.blue > bag.blue || set.red > bag.red || set.green > bag.green {
                return false;
            }
        }
        true
    }
    fn get_power(&self) -> u32 {
        let (mut blue, mut green, mut red) = (0, 0, 0);
        for set in &self.sets {
            if set.red > red {
                red = set.red;
            }
            if set.green > green {
                green = set.green;
            }
            if set.blue > blue {
                blue = set.blue;
            }
        }
        red * green * blue
    }
}

struct Set {
    blue: u32,
    red: u32,
    green: u32,
}

impl Set {
    pub fn new(blue: u32, red: u32, green: u32) -> Self {
        Self { blue, red, green }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Game> {
    let mut res = vec![];
    for line in input.lines() {
        let (game_name, sets) = line.split_once(':').unwrap();
        let mut game_name = game_name.to_string();
        game_name.retain(|x| x.is_ascii_digit());
        let game_id = game_name.parse::<u32>().unwrap();
        let mut res_sets = vec![];
        for set in sets.split(';') {
            let (mut blue, mut green, mut red) = (0, 0, 0);
            for cubes in set.split(',') {
                let (nb, color) = cubes.trim().split_once(' ').unwrap();
                let nb = nb.parse::<u32>().unwrap();
                match color {
                    "blue" => blue = nb,
                    "green" => green = nb,
                    "red" => red = nb,
                    _ => {}
                }
            }
            res_sets.push(Set::new(blue, red, green));
        }
        res.push(Game {
            id: game_id,
            sets: res_sets,
        });
    }
    res
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> u32 {
    let bag = Set::new(14, 12, 13);
    input
        .iter()
        .filter(|game| game.is_possible(&bag))
        .fold(0, |acc, g| acc + g.id)
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|game| game.get_power()).sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(&input_generator(input)), 8);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day2.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 2006);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(&input_generator(input)), 2286);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day2.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 84911);
    }
}
