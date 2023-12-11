use itertools::Itertools;
use std::str::FromStr;

use crate::grid::Grid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GalaxyMapItem {
    EmptySpace,
    Galaxy,
}
impl Default for GalaxyMapItem {
    fn default() -> Self {
        Self::EmptySpace
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidGalaxyItem,
}

impl TryFrom<char> for GalaxyMapItem {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::EmptySpace),
            '#' => Ok(Self::Galaxy),
            _ => Err(ParseError::InvalidGalaxyItem),
        }
    }
}

impl From<GalaxyMapItem> for char {
    fn from(val: GalaxyMapItem) -> Self {
        match val {
            GalaxyMapItem::EmptySpace => '.',
            GalaxyMapItem::Galaxy => '#',
        }
    }
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Grid<GalaxyMapItem> {
    Grid::from_str(input).unwrap()
}

fn expand_universe(mut universe: Grid<GalaxyMapItem>) -> Grid<GalaxyMapItem> {
    let empty_rows = universe
        .iter_rows()
        .enumerate()
        .filter_map(|(i, v)| {
            v.iter()
                .all(|e| e == &&GalaxyMapItem::EmptySpace)
                .then_some(i)
        })
        .collect::<Vec<_>>();
    let empty_cols = universe
        .iter_columns()
        .enumerate()
        .filter_map(|(i, v)| {
            v.iter()
                .all(|e| e == &&GalaxyMapItem::EmptySpace)
                .then_some(i)
        })
        .collect::<Vec<_>>();

    empty_rows
        .iter()
        .rev()
        .for_each(|r| universe.insert_row_default(*r));

    empty_cols
        .iter()
        .rev()
        .for_each(|c| universe.insert_col_default(*c));

    universe
}

#[aoc(day11, part1)]
pub fn part1(input: &Grid<GalaxyMapItem>) -> usize {
    let mut universe = expand_universe(input.clone());

    println!("{}", universe);
    let mut galaxies_positions = universe.find_positions(|e| e == &GalaxyMapItem::Galaxy);

    galaxies_positions
        .combinations(2)
        .map(|x| x[0].distance(&x[1]))
        .sum()
}
#[aoc(day11, part2)]
pub fn part2(input: &Grid<GalaxyMapItem>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    // #[test]
    // fn test_parse() {
    //     let expected = Grid::new(vec![], 0);
    //     assert_eq!(expected, parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(374, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
