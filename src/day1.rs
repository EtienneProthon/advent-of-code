use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;

#[aoc_generator(day1, part1)]
fn input_generator_part1(input: &str) -> Vec<u32> {
    let mut res = vec![];
    for line in input.lines() {
        let mut str = line.to_string();
        str.retain(|x| x.is_digit(10));
        let first = str.chars().next().unwrap();
        let last = str.pop().unwrap();
        let nb = format!("{first}{last}").parse::<u32>().unwrap();
        res.push(nb);
    }
    res
}

fn parse_number(value: &str) -> u32 {
    match value {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}

#[aoc_generator(day1, part2)]
fn input_generator_part2(input: &str) -> Vec<u32> {
    let re = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    let mut res = vec![];
    for line in input.lines() {
        let mut iter_match = re.captures_iter(line);
        let first_match = iter_match
            .nth(0)
            .and_then(|x| x.unwrap().get(1))
            .map(|x| x.as_str())
            .unwrap_or("0");
        let last_match = iter_match
            .last()
            .and_then(|x| x.unwrap().get(1))
            .map(|x| x.as_str())
            .unwrap_or("0");
        let first_match = parse_number(first_match);
        let last_match = parse_number(last_match);
        let nb = if last_match == 0 {
            first_match * 10 + first_match
        } else {
            first_match * 10 + last_match
        };
        res.push(nb);
    }
    res
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator_part1, input_generator_part2, part1, part2};

    #[test]
    fn test_part1() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part1(&input_generator_part1(input)), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(&input_generator_part2(input)), 281);
    }
}
