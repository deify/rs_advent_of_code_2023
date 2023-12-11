use std::str::FromStr;

use crate::grid::Grid;

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Pipe(Direction, Direction);

#[derive(Debug)]
pub enum ParseError {
    InvalidPipe,
}

impl TryFrom<char> for Pipe {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '|' => Ok(Pipe(North, South)),
            '-' => Ok(Pipe(East, West)),
            'L' => Ok(Pipe(North, East)),
            'J' => Ok(Pipe(North, West)),
            '7' => Ok(Pipe(South, West)),
            'F' => Ok(Pipe(South, East)),
            _ => Err(ParseError::InvalidPipe),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Ground,
    Start,
    Pipe(Pipe),
}
impl TryFrom<char> for Tile {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => Self::Pipe(Pipe::try_from(value)?),
        })
    }
}

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Grid<Tile> {
    Grid::from_str(input).unwrap()
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    let start_pos = input.find_pos(|t| t == &Tile::Start);
    0
}

#[aoc(day10, part2)]
pub fn part2(input: &Grid<Tile>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    // #[test]
    // fn test_parse() {
    //     let expected = Grid {
    //         data: vec![],
    //         columns: 0,
    //     };
    //     assert_eq!(expected, parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
