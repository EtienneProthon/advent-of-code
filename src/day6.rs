use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().trim();
    let distances = lines.next().unwrap().trim();
    let times: Vec<u64> = times
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let distances: Vec<u64> = distances
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    times.into_iter().zip(distances).collect()
}

fn is_race_win(speed: u64, race_time: u64, best_distance: u64) -> bool {
    let run_time = race_time - speed;
    let distance_reached = run_time * speed;
    distance_reached > best_distance
}

#[aoc(day6, part1)]
pub fn part1(input: &[(u64, u64)]) -> u64 {
    let mut race_win_time = vec![];
    for race in input {
        let (time, distance) = *race;
        let mut wins = 0;
        for hold_time in 0..time {
            if is_race_win(hold_time, time, distance) {
                wins += 1;
            }
        }
        race_win_time.push(wins);
    }
    race_win_time.iter().product()
}

pub fn binary_search(min: u64, max: u64, race_time: u64, best_distance: u64, asc: bool) -> u64 {
    let mut min = min;
    let mut max = max;
    let mut mid = (min + max) / 2;
    let mut is_win = false;
    while min < max || !is_win {
        is_win = is_race_win(mid, race_time, best_distance);

        if is_win {
            if asc {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        } else if asc {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
        mid = (min + max) / 2;
    }
    mid
}

#[aoc(day6, part2)]
pub fn part2(input: &[(u64, u64)]) -> u64 {
    let time: String = input.iter().map(|(t, _)| t.to_string()).collect();
    let distance: String = input.iter().map(|(_, d)| d.to_string()).collect();
    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();
    let win_range_min = binary_search(0, time / 2, time, distance, true);
    let win_range_max = binary_search(time / 2, time, time, distance, false);
    win_range_max - win_range_min
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT: &str = "Time:      7  15   30
        Distance:  9  40  200";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 288);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day6.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 219849);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 71503);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day6.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 29432455);
    }
}
