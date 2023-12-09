use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let mut res = vec![];
    for line in input.lines() {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        res.push(values);
    }
    res
}

fn compute_diff_steps(input: &[i32]) -> Vec<Vec<i32>> {
    let mut diff_steps = vec![input.to_vec()];
    loop {
        let values_to_diff = diff_steps.last().unwrap();
        let mut diff_between_values = Vec::with_capacity(values_to_diff.len() - 1);
        for i in 0..values_to_diff.len() - 1 {
            diff_between_values.push(values_to_diff[i + 1] - values_to_diff[i]);
        }
        if diff_between_values.iter().all(|&x| x == 0) {
            break;
        }
        diff_steps.push(diff_between_values);
    }
    diff_steps
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<Vec<i32>>) -> i32 {
    let mut res = vec![];
    for history in input {
        let diff_steps = compute_diff_steps(history);

        // Find the new step
        let mut new_step = 0;
        for step in diff_steps.iter().rev() {
            let last_value = step.last().unwrap();
            new_step += last_value;
        }
        res.push(new_step);
    }
    res.iter().sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<Vec<i32>>) -> i32 {
    let mut res = vec![];
    for history in input {
        let diff_steps = compute_diff_steps(history);

        // Find the new step
        let mut new_step = 0;
        for step in diff_steps.iter().rev() {
            let last_value = step.first().unwrap();
            new_step = last_value - new_step;
        }
        res.push(new_step);
    }
    res.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT: &str = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 114);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day9.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 2101499000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 2);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day9.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 1089);
    }
}
