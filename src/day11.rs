use itertools::Itertools;
use std::str::FromStr;

use crate::grid::{Grid, Position};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GalaxyMapItem {
    EmptySpace,
    Galaxy,
    Void,
}
impl Default for GalaxyMapItem {
    fn default() -> Self {
        Self::EmptySpace
    }
}

impl GalaxyMapItem {
    fn actual_size(&self) -> usize {
        match self {
            Self::EmptySpace => 1,
            Self::Galaxy => 1,
            Self::Void => 1000000,
        }
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
            'o' => Ok(Self::Void),
            _ => Err(ParseError::InvalidGalaxyItem),
        }
    }
}

impl From<GalaxyMapItem> for char {
    fn from(val: GalaxyMapItem) -> Self {
        match val {
            GalaxyMapItem::EmptySpace => '.',
            GalaxyMapItem::Galaxy => '#',
            GalaxyMapItem::Void => 'o',
        }
    }
}

#[aoc_generator(day11)]
pub fn parse(input: &str) -> Grid<GalaxyMapItem> {
    Grid::from_str(input).unwrap()
}

fn expand_universe(
    mut universe: Grid<GalaxyMapItem>,
    expand_item: &GalaxyMapItem,
) -> Grid<GalaxyMapItem> {
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

    empty_rows.iter().rev().for_each(|r| {
        universe.insert_row_with(*r, expand_item);
        if expand_item == &GalaxyMapItem::Void {
            universe.drop_row(*r + 1);
        }
    });

    empty_cols.iter().rev().for_each(|c| {
        universe.insert_col_with(*c, expand_item);
        if expand_item == &GalaxyMapItem::Void {
            universe.drop_column(*c + 1);
        }
    });

    universe
}

#[aoc(day11, part1)]
pub fn part1(input: &Grid<GalaxyMapItem>) -> usize {
    let universe = expand_universe(input.clone(), &GalaxyMapItem::EmptySpace);
    println!("{}", universe);

    let galaxies_positions = universe.find_positions(|e| e == &GalaxyMapItem::Galaxy);

    galaxies_positions
        .combinations(2)
        .map(|x| x[0].manhattan_distance(&x[1]))
        .sum()
}
#[aoc(day11, part2)]
pub fn part2(input: &Grid<GalaxyMapItem>) -> usize {
    let universe = expand_universe(input.clone(), &GalaxyMapItem::Void);
    println!("{}", universe);

    let galaxies_positions = universe.find_positions(|e| e == &GalaxyMapItem::Galaxy);
    let pairs = galaxies_positions.combinations(2);

    pairs
        .map(|p| {
            let x_min = p[0].x.min(p[1].x);
            let x_max = p[0].x.max(p[1].x);
            let y_min = p[0].y.min(p[1].y);
            let y_max = p[0].y.max(p[1].y);

            let x_fields =
                (x_min + 1..=x_max).map(|x| universe.at(&Position { x, y: y_min }).unwrap());
            let y_fields =
                (y_min + 1..=y_max).map(|y| universe.at(&Position { x: x_max, y }).unwrap());
            x_fields
                .chain(y_fields)
                .map(|e| e.actual_size())
                .sum::<usize>()
        })
        .sum()
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
        assert_eq!(82000210, part2(&parse(TEST_INPUT)));
    }
}
