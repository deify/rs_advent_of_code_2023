use std::str::FromStr;

use itertools::Itertools;

use crate::grid::Grid;

#[derive(Debug, PartialEq, Eq)]
pub enum GroundType {
    Ash,
    Rocks,
}
#[derive(Debug)]
pub enum ParseError {
    InvalidGroundType,
}

impl TryFrom<char> for GroundType {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rocks),
            _ => Err(ParseError::InvalidGroundType),
        }
    }
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> Vec<Grid<GroundType>> {
    let mut line_iter = input.lines();
    let mut ret = vec![];
    loop {
        let lines = line_iter
            .by_ref()
            .take_while(|l| !l.is_empty())
            .collect_vec();
        if lines.is_empty() {
            break;
        }
        let grid_str = lines.join("\n");
        ret.push(Grid::from_str(&grid_str).unwrap());
    }
    ret
}

pub enum MirrorDirection {
    Vertical,
    Horizontal,
}

fn check_mirror<T: PartialEq>(grid: &Grid<T>, mirror_index: usize, dir: MirrorDirection) -> bool {
    let mirror_line = mirror_index + 1;
    match dir {
        MirrorDirection::Horizontal => {
            let max_mirror_space = mirror_line.min(grid.rows() - mirror_line);
            if max_mirror_space == 0 {
                return false;
            }

            (0..max_mirror_space).all(|offset| {
                let r1 = grid.iter_row(mirror_index - offset);
                let r2 = grid.iter_row(mirror_index + 1 + offset);
                r1.eq(r2)
            })
        }
        MirrorDirection::Vertical => {
            let max_mirror_space = mirror_line.min(grid.columns - mirror_line);
            if max_mirror_space == 0 {
                return false;
            }

            (0..max_mirror_space).all(|offset| {
                let r1 = grid.iter_col(mirror_index - offset);
                let r2 = grid.iter_col(mirror_index + 1 + offset);
                r1.eq(r2)
            })
        }
    }
}

fn calc_reflection_count<T: PartialEq>(grid: &Grid<T>) -> usize {
    if let Some(horizontal_mirror) =
        (0..grid.rows()).find(|i| check_mirror(grid, *i, MirrorDirection::Horizontal))
    {
        (horizontal_mirror + 1) * 100
    } else if let Some(vertical_mirror) =
        (0..grid.columns).find(|i| check_mirror(grid, *i, MirrorDirection::Vertical))
    {
        vertical_mirror + 1
    } else {
        0
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &[Grid<GroundType>]) -> usize {
    input.iter().map(calc_reflection_count).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Grid<GroundType>]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    // #[test]
    // fn test_parse() {
    //     let expected: Vec<Grid<GroundType>> = vec![];
    //     assert_eq!(expected, parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(405, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
