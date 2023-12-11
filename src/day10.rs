use aoc_runner_derive::{aoc, aoc_generator};
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Grid = HashMap<(usize, usize), ECell>;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ECell {
    PVertical,
    PHorizontal,
    PUpLeft,
    PUpRight,
    PDownLeft,
    PDownRight,
    Ground,
    Start,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EDirection {
    Up,
    Down,
    Left,
    Right,
}

impl ECell {
    pub fn to_direction(cell: &ECell) -> Vec<EDirection> {
        match cell {
            ECell::PVertical => vec![EDirection::Up, EDirection::Down],
            ECell::PHorizontal => vec![EDirection::Left, EDirection::Right],
            ECell::PUpRight => vec![EDirection::Up, EDirection::Right],
            ECell::PUpLeft => vec![EDirection::Up, EDirection::Left],
            ECell::PDownLeft => vec![EDirection::Down, EDirection::Left],
            ECell::PDownRight => vec![EDirection::Down, EDirection::Right],
            ECell::Start => vec![
                EDirection::Up,
                EDirection::Down,
                EDirection::Right,
                EDirection::Left,
            ],
            _ => vec![],
        }
    }
}

impl FromStr for ECell {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(ECell::PVertical),
            "-" => Ok(ECell::PHorizontal),
            "L" => Ok(ECell::PUpRight),
            "J" => Ok(ECell::PUpLeft),
            "7" => Ok(ECell::PDownLeft),
            "F" => Ok(ECell::PDownRight),
            "." => Ok(ECell::Ground),
            "S" => Ok(ECell::Start),
            _ => Err(()),
        }
    }
}

#[aoc_generator(day10)]
fn input_generator(input: &str) -> ((usize, usize), Grid) {
    let mut res = Grid::new();
    let mut start_pos = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            let cell = c.to_string().parse().unwrap();
            if cell == ECell::Start {
                start_pos = (x, y);
            }
            res.insert((x, y), cell);
        }
    }
    (start_pos, res)
}

fn match_direction(
    grid: &Grid,
    current_pos: (usize, usize),
    direction: EDirection,
) -> Option<(usize, usize, EDirection)> {
    let (cell_pos, contain_dir) = match direction {
        EDirection::Up => {
            if current_pos.1 == 0 {
                return None;
            }
            ((current_pos.0, current_pos.1 - 1), EDirection::Down)
        }
        EDirection::Down => ((current_pos.0, current_pos.1 + 1), EDirection::Up),
        EDirection::Left => {
            if current_pos.0 == 0 {
                return None;
            }
            ((current_pos.0 - 1, current_pos.1), EDirection::Right)
        }
        EDirection::Right => ((current_pos.0 + 1, current_pos.1), EDirection::Left),
    };
    let cell = grid.get(&cell_pos);
    if let Some(c) = cell {
        if ECell::to_direction(c).contains(&contain_dir) {
            return Some((cell_pos.0, cell_pos.1, contain_dir));
        }
    }
    None
}

fn find_next_pipe(
    grid: &Grid,
    current_pos: (usize, usize),
    direction: EDirection,
) -> (usize, usize, EDirection) {
    let current_cell = grid.get(&current_pos).unwrap();
    let mut directions = ECell::to_direction(current_cell);
    directions.retain(|&x| x != direction);
    for dir in directions {
        if let Some(res) = match_direction(grid, current_pos, dir) {
            return res;
        }
    }
    panic!("no next pipe")
}

#[aoc(day10, part1)]
pub fn part1(input: &((usize, usize), Grid)) -> u32 {
    let (start_pos, grid) = input;
    // Find one adjacent pipes from the start position
    let left_pos = match_direction(grid, *start_pos, EDirection::Left);
    let right_pos = match_direction(grid, *start_pos, EDirection::Right);
    let up_pos = match_direction(grid, *start_pos, EDirection::Up);
    let down_pos = match_direction(grid, *start_pos, EDirection::Down);
    let mut positions = vec![left_pos, right_pos, up_pos, down_pos]
        .into_iter()
        .flatten();
    let mut search = positions.next().unwrap();
    let mut step = 1;
    while search.0 != start_pos.0 || search.1 != start_pos.1 {
        search = find_next_pipe(grid, (search.0, search.1), search.2);
        step += 1;
    }
    step / 2
}

#[aoc(day10, part2)]
pub fn part2(input: &((usize, usize), Grid)) -> u32 {
    let (start_pos, grid) = input;

    // Find one adjacent pipes from the start position
    let left_pos = match_direction(grid, *start_pos, EDirection::Left);
    let right_pos = match_direction(grid, *start_pos, EDirection::Right);
    let up_pos = match_direction(grid, *start_pos, EDirection::Up);
    let down_pos = match_direction(grid, *start_pos, EDirection::Down);
    let mut positions = vec![left_pos, right_pos, up_pos, down_pos]
        .into_iter()
        .flatten();
    let mut search = positions.next().unwrap();
    let mut pipe_pos = HashSet::from([*start_pos, (search.0, search.1)]);
    while search.0 != start_pos.0 || search.1 != start_pos.1 {
        search = find_next_pipe(grid, (search.0, search.1), search.2);
        pipe_pos.insert((search.0, search.1));
    }
    let max_x = pipe_pos.iter().map(|x| x.0).max().unwrap();
    let max_y = pipe_pos.iter().map(|x| x.1).max().unwrap();
    let mut count_inside = 0;
    for y in 0..max_y {
        let mut inside = false;
        for x in 0..max_x {
            if pipe_pos.contains(&(x, y)) {
                let cell = grid.get(&(x, y)).unwrap();
                if [
                    ECell::PVertical,
                    ECell::PUpLeft,
                    ECell::PUpRight,
                    ECell::Start,
                ]
                .contains(cell)
                {
                    inside = !inside;
                }
            } else if inside {
                count_inside += 1;
            }
        }
    }
    count_inside
}

#[cfg(test)]
pub mod tests {
    use super::{input_generator, part1, part2};
    use std::fs;

    const INPUT_1: &str = "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF";

    const INPUT_2: &str = "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ";

    const INPUT_3: &str = "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........";

    const INPUT_4: &str = ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...";

    const INPUT_5: &str = "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 4);
        assert_eq!(part1(&input_generator(INPUT_2)), 8);
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("input/2023/day10.txt").unwrap();
        assert_eq!(part1(&input_generator(&input)), 7063);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_1)), 1);
        assert_eq!(part2(&input_generator(INPUT_2)), 0);
        assert_eq!(part2(&input_generator(INPUT_3)), 4);
        assert_eq!(part2(&input_generator(INPUT_4)), 8);
        assert_eq!(part2(&input_generator(INPUT_5)), 10);
    }

    #[test]
    fn test_part2_input() {
        let input = fs::read_to_string("input/2023/day10.txt").unwrap();
        assert_eq!(part2(&input_generator(&input)), 589);
    }
}
