use std::{fmt::Debug, str::FromStr};

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

#[derive(Debug)]
pub enum MirrorDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct MirrorLineIter<'a, T> {
    grid: &'a Grid<T>,
    mirror_line: usize,
    offset: usize,
    dir: MirrorDirection,
}
impl<'a, T> MirrorLineIter<'a, T> {
    fn new(grid: &'a Grid<T>, mirror_line: usize, dir: MirrorDirection) -> Self {
        Self {
            grid,
            mirror_line,
            offset: 0,
            dir,
        }
    }
}

impl<'a, T> Iterator for MirrorLineIter<'a, T> {
    type Item = (Vec<&'a T>, Vec<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        let lower_index = self.mirror_line.checked_sub(self.offset)?;
        let upper_index = self.mirror_line + 1 + self.offset;
        self.offset += 1;

        match self.dir {
            MirrorDirection::Horizontal => {
                if upper_index >= self.grid.rows() {
                    return None;
                }
                Some((
                    self.grid.iter_row(lower_index).collect_vec(),
                    self.grid.iter_row(upper_index).collect_vec(),
                ))
            }
            MirrorDirection::Vertical => {
                if upper_index >= self.grid.columns {
                    return None;
                }
                Some((
                    self.grid.iter_col(lower_index).collect_vec(),
                    self.grid.iter_col(upper_index).collect_vec(),
                ))
            }
        }
    }
}

fn check_mirror<T: PartialEq + Debug>(
    grid: &Grid<T>,
    mirror_line: usize,
    dir: MirrorDirection,
) -> bool {
    let mut mirror_iter = MirrorLineIter::new(grid, mirror_line, dir);
    mirror_iter.all(|(a, b)| dbg!(a == b))
}

fn calc_reflection_count<T: PartialEq + Debug>(grid: &Grid<T>) -> usize {
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
