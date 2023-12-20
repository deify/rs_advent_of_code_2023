use std::{cmp::Ordering, collections::VecDeque, ops, str::FromStr};

use itertools::Itertools;

use crate::grid::{Direction, Grid, Position};

#[derive(Debug, PartialEq, Eq)]
pub struct DigEntry {
    direction: Direction,
    distance: usize,
    color_str: String,
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct IntPosition {
    x: isize,
    y: isize,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidDigEntry,
}

impl FromStr for DigEntry {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let direction = parts
            .next()
            .map_or(Err(ParseError::InvalidDigEntry), |s| match s {
                "R" => Ok(Direction::East),
                "L" => Ok(Direction::West),
                "U" => Ok(Direction::North),
                "D" => Ok(Direction::South),
                _ => Err(ParseError::InvalidDigEntry),
            })?;

        let distance = parts.next().map_or(Err(ParseError::InvalidDigEntry), |s| {
            s.parse().map_err(|_| ParseError::InvalidDigEntry)
        })?;

        let color_str = parts
            .next()
            .map(|s| s.to_string())
            .ok_or(ParseError::InvalidDigEntry)?;

        Ok(Self {
            direction,
            distance,
            color_str,
        })
    }
}

#[aoc_generator(day18, part1)]
pub fn parse1(input: &str) -> Vec<DigEntry> {
    input
        .lines()
        .map(|l| DigEntry::from_str(l).unwrap())
        .collect()
}

#[aoc_generator(day18, part2)]
pub fn parse2(input: &str) -> Vec<Position> {
    let instructions = input.lines().map(|l| {
        let (_, code) = l.split_once('#').unwrap();
        let distance = usize::from_str_radix(&code[..5], 16).unwrap();
        let direction = match code.chars().nth(5).unwrap() {
            '0' => Direction::East,
            '1' => Direction::South,
            '2' => Direction::West,
            '3' => Direction::North,
            _ => panic!(),
        };

        DigEntry {
            direction,
            distance,
            color_str: String::new(),
        }
    });

    let positions = instructions
        .scan(IntPosition { x: 0, y: 0 }, |state, step| {
            match step.direction {
                Direction::North => state.y -= step.distance as isize,
                Direction::East => state.x += step.distance as isize,
                Direction::South => state.y += step.distance as isize,
                Direction::West => state.x -= step.distance as isize,
            };
            Some(state.clone())
        })
        .collect_vec();

    let min_pos = positions.iter().min().unwrap().clone();

    positions
        .iter()
        .map(|p| Position {
            x: (p.x - min_pos.x) as usize,
            y: (p.y - min_pos.y) as usize,
        })
        .collect()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Terrain {
    #[default]
    GroundLevel,
    Trench(Option<String>),
}
impl From<Terrain> for char {
    fn from(value: Terrain) -> Self {
        match value {
            Terrain::GroundLevel => '.',
            Terrain::Trench(_) => '#',
        }
    }
}

fn expand_grid<T: Default + Clone>(
    grid: &mut Grid<T>,
    position: &mut Position,
    direction: &Direction,
    distance: usize,
) {
    match direction {
        Direction::North => {
            let existing_space = position.y;
            if distance.cmp(&existing_space) == std::cmp::Ordering::Greater {
                let expansion = distance - existing_space;
                (0..expansion).for_each(|_| grid.insert_row_default(0));
                position.y += expansion;
            }
        }
        Direction::East => {
            let existing_space = grid.columns - position.x - 1;
            if distance.cmp(&existing_space) == std::cmp::Ordering::Greater {
                let expansion = distance - existing_space;
                (0..expansion).for_each(|_| grid.insert_col_default(grid.columns));
            }
        }
        Direction::South => {
            let existing_space = grid.rows() - position.y - 1;
            if distance.cmp(&existing_space) == std::cmp::Ordering::Greater {
                let expansion = distance - existing_space;
                (0..expansion).for_each(|_| grid.insert_row_default(grid.rows()));
            }
        }
        Direction::West => {
            let existing_space = position.x;
            if distance.cmp(&existing_space) == std::cmp::Ordering::Greater {
                let expansion = distance - existing_space;
                (0..expansion).for_each(|_| grid.insert_col_default(0));
                position.x += expansion;
            }
        }
    }
}

fn dig_trench(instructions: &[DigEntry]) -> Grid<Terrain> {
    let mut grid: Grid<Terrain> = Grid::new(vec![Terrain::Trench(None)], 1);
    let mut current_pos = Position { x: 0, y: 0 };

    for step in instructions {
        expand_grid(&mut grid, &mut current_pos, &step.direction, step.distance);
        (0..step.distance).for_each(|_| {
            current_pos = current_pos.move_dir(&step.direction).unwrap();
            let e = grid.at_mut(&current_pos).unwrap();
            *e = Terrain::Trench(Some(step.color_str.clone()));
        });
    }

    grid
}

fn flood_filll(grid: &mut Grid<Terrain>, position: &Position) {
    let mut queue = VecDeque::new();
    queue.push_back(*position);

    while let Some(current_pos) = queue.pop_front() {
        let current = grid.at_mut(&current_pos).unwrap();
        if matches!(current, Terrain::Trench(_)) {
            continue;
        }

        *current = Terrain::Trench(None);
        if let Some(pos) = current_pos.move_dir(&Direction::North) {
            queue.push_back(pos);
        }
        if let Some(pos) = current_pos.move_dir(&Direction::East) {
            queue.push_back(pos);
        }
        if let Some(pos) = current_pos.move_dir(&Direction::West) {
            queue.push_back(pos);
        }
        if let Some(pos) = current_pos.move_dir(&Direction::South) {
            queue.push_back(pos);
        }
    }
}

fn fill_trench(grid: &mut Grid<Terrain>) {
    let (i, _) = grid
        .iter_row(0)
        .enumerate()
        .find(|(i, e)| {
            matches!(e, Terrain::Trench(_))
                && matches!(
                    grid.at(&Position { x: *i, y: 1 }).unwrap(),
                    Terrain::GroundLevel
                )
        })
        .unwrap();

    let start_pos = Position { x: i, y: 1 };
    flood_filll(grid, &start_pos);
}

#[aoc(day18, part1)]
pub fn part1(input: &[DigEntry]) -> usize {
    let mut grid = dig_trench(input);

    println!("path:");
    println!("{}", grid);

    fill_trench(&mut grid);

    println!("filled");
    println!("{}", grid);

    grid.iter()
        .filter(|e| matches!(e, Terrain::Trench(_)))
        .count()
}

#[aoc(day18, part2)]
pub fn part2(input: &[Position]) -> usize {
    let len = input.len();
    input
        .iter()
        .cycle()
        .tuple_windows()
        .take(len)
        .map(|(a, b)| {
            dbg!((a, b));
            let x_diff = a.x as isize - b.x as isize;
            match x_diff.cmp(&0) {
                Ordering::Less => (x_diff - 1) * (a.y as isize),
                Ordering::Greater => (x_diff + 1) * (a.y as isize + 1),
                _ => 0,
            }
        })
        .sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_parse1() {
        let expected: Vec<DigEntry> = vec![];
        assert_eq!(expected, parse1(TEST_INPUT));
    }

    #[test]
    fn test_parse2() {
        let positions = parse2(TEST_INPUT);

        let min = positions.iter().min().unwrap();
        assert_eq!(min, &Position { x: 1, y: 1 });
    }

    #[test]
    fn test_part1() {
        assert_eq!(62, part1(&parse1(TEST_INPUT)));
    }

    #[test]
    fn solve_part1() {
        assert_eq!(
            35244,
            part1(&parse1(include_str!("../input/2023/day18.txt")))
        );
    }

    #[test]
    fn test_part2_trivial() {
        let positions = [
            Position { x: 1, y: 1 },
            Position { x: 4, y: 1 },
            Position { x: 4, y: 4 },
            Position { x: 1, y: 4 },
        ];
        assert_eq!(16, part2(&positions))
    }
    #[test]
    fn test_part2_trivial_2() {
        let positions = [
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 2 },
            Position { x: 3, y: 1 },
            Position { x: 3, y: 0 },
            Position { x: 4, y: 0 },
            Position { x: 4, y: 3 },
            Position { x: 0, y: 3 },
        ];
        assert_eq!(18, part2(&positions))
    }

    #[test]
    fn test_part2() {
        assert_eq!(952408144115, part2(&parse2(TEST_INPUT)));
    }

    #[test]
    fn solve_part2() {
        assert_eq!(1, part2(&parse2(include_str!("../input/2023/day18.txt"))));
    }
}
