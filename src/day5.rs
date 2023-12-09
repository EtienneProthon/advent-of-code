use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Map {
    destination: String,
    ranges: Vec<(u64, u64, u64)>,
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Vec<u64>, HashMap<String, Map>) {
    let mut res = HashMap::new();
    let mut block_iter = input.split("\n\n");
    let seeds = block_iter.next().unwrap();
    let seeds = seeds
        .trim_start_matches("seeds:")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    for block in block_iter {
        let mut line_iter = block.lines();
        let first_line = line_iter.next().unwrap();
        let (source, destination) = first_line
            .trim_end_matches("map:")
            .trim()
            .split_once("-to-")
            .unwrap();
        let mut ranges = vec![];
        for line in line_iter {
            let splited_line: Vec<&str> = line.trim().splitn(3, ' ').collect();
            let d_range_start = splited_line[0].parse::<u64>().unwrap();
            let s_range_start = splited_line[1].parse::<u64>().unwrap();
            let range_length = splited_line[2].parse::<u64>().unwrap();
            ranges.push((d_range_start, s_range_start, range_length));
        }
        res.insert(
            source.to_string(),
            Map {
                destination: destination.to_string(),
                ranges,
            },
        );
    }
    (seeds, res)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<u64>, HashMap<String, Map>)) -> u64 {
    let (seeds, categories) = input;
    let mut seeds = seeds.clone();
    let mut source_map = "seed";
    while source_map != "location" {
        let map = categories.get(source_map).unwrap();
        for seed in &mut seeds {
            for range in &map.ranges {
                let (d_range_start, s_range_start, range_length) = *range;
                if *seed >= s_range_start && *seed < s_range_start + range_length {
                    *seed = d_range_start + (*seed - s_range_start);
                    break;
                }
            }
        }
        source_map = map.destination.as_str();
    }
    *seeds.iter().min().unwrap()
}

pub fn find_new_seeds(seeds: &[(u64, u64)], map: &Map, _deep: usize) -> Vec<(u64, u64)> {
    let mut new_seeds = vec![];
    for seed in seeds {
        let (seed_range_start, seed_range_end) = *seed;
        let mut found_overlap = false;
        for range in &map.ranges {
            let (d_range_start, s_range_start, range_length) = *range;
            let s_range_end = s_range_start + range_length - 1;
            // Check if overlaping
            if seed_range_start < s_range_end && s_range_start < seed_range_end {
                let mut match_range_start = seed_range_start;
                let mut match_range_end = seed_range_end;
                let mut seeds_to_search = vec![];

                // Search for values before the range
                if seed_range_start < s_range_start {
                    seeds_to_search.push((seed_range_start, s_range_start - 1));
                    match_range_start = s_range_start;
                }
                // Search for values after the range
                if seed_range_end > s_range_end {
                    seeds_to_search.push((s_range_end + 1, seed_range_end));
                    match_range_end = s_range_end;
                }
                let new_seed_range_start = d_range_start + (match_range_start - s_range_start);
                let new_seed_range_end =
                    new_seed_range_start + (match_range_end - match_range_start);

                new_seeds.push((new_seed_range_start, new_seed_range_end));
                if !seeds_to_search.is_empty() {
                    new_seeds.append(&mut find_new_seeds(&seeds_to_search, map, _deep + 1));
                }
                found_overlap = true;
                break;
            }
        }
        // If no overlap with any range we push the seeds as the number should stay the same
        if !found_overlap {
            new_seeds.push((seed_range_start, seed_range_end))
        }
    }
    new_seeds
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<u64>, HashMap<String, Map>)) -> u64 {
    let (source_seeds, categories) = input;
    let mut seeds = vec![];
    for chunk in source_seeds.chunks(2) {
        seeds.push((chunk[0], chunk[0] + chunk[1] - 1));
    }
    let mut source_map = "seed";
    while source_map != "location" {
        let map = categories.get(source_map).unwrap();
        seeds = find_new_seeds(&seeds, map, 1);
        source_map = map.destination.as_str();
    }
    seeds.iter().map(|x| x.0).min().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!(part1(&input_generator(input)), 35);
        // 324724204
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day5.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 324724204);
    }

    #[test]
    fn test_part2() {
        let input = "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4";
        assert_eq!(part2(&input_generator(input)), 46);
        // 104070862
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day5.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 104070862);
    }
}
