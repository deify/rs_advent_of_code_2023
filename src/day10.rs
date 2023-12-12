use std::{cmp::Ordering, str::FromStr};

use crate::grid::{Grid, Position};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North = 0,
    West,
    South,
    East,
}
impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::West,
    Direction::South,
    Direction::East,
];

#[derive(Debug, PartialEq, Eq)]
pub struct Pipe(Direction, Direction);

impl Pipe {
    fn has_entry_in_direction(&self, d: Direction) -> bool {
        self.0 == d || self.1 == d
    }

    fn is_accessible_from_direction(&self, d: &Direction) -> bool {
        match d {
            Direction::North if self.has_entry_in_direction(Direction::South) => true,
            Direction::East if self.has_entry_in_direction(Direction::West) => true,
            Direction::South if self.has_entry_in_direction(Direction::North) => true,
            Direction::West if self.has_entry_in_direction(Direction::East) => true,
            _ => false,
        }
    }
    fn get_exit_direction(&self, enter_direction: &Direction) -> Option<Direction> {
        if !self.is_accessible_from_direction(enter_direction) {
            return None;
        }
        let entry = enter_direction.invert();
        if self.0 == entry {
            Some(self.1)
        } else {
            Some(self.0)
        }
    }
}

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

struct PipeIterator<'a> {
    grid: &'a Grid<Tile>,
    current_pos: Position,
    next_direction: Option<Direction>,
}

impl<'a> PipeIterator<'a> {
    fn new(grid: &'a Grid<Tile>, start_pos: Position) -> Self {
        Self {
            grid,
            current_pos: start_pos,
            next_direction: None,
        }
    }

    fn pos_in_direction(&self, dir: &Direction) -> Option<Position> {
        match dir {
            Direction::South => Some(Position {
                x: self.current_pos.x,
                y: self.current_pos.y + 1,
            }),
            Direction::East => Some(Position {
                x: self.current_pos.x + 1,
                y: self.current_pos.y,
            }),
            Direction::North => Some(Position {
                x: self.current_pos.x,
                y: self.current_pos.y.checked_sub(1)?,
            }),
            Direction::West => Some(Position {
                x: self.current_pos.x.checked_sub(1)?,
                y: self.current_pos.y,
            }),
        }
    }
}
impl<'a> Iterator for PipeIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let directions_to_check = self
            .next_direction
            .map(|d| vec![d])
            .unwrap_or(DIRECTIONS.to_vec());

        let positions = directions_to_check
            .iter()
            .map(|d| (d, self.pos_in_direction(d)))
            .filter_map(|(d, p)| p.and_then(|p| self.grid.is_position_valid(&p).then_some((d, p))));

        let tiles = positions
            // .map(|(d, p)| (d, p, self.grid.at(&p).unwrap()))
            .filter_map(|(d, p)| self.grid.at(&p).map(|t| (d, p, t)))
            .collect::<Vec<_>>();

        tiles.into_iter().find_map(|(d, p, t)| match t {
            Tile::Pipe(pipe) if pipe.is_accessible_from_direction(d) => {
                self.current_pos = p;
                self.next_direction = pipe.get_exit_direction(d);
                Some(t)
            }
            _ => None,
        })
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    let start_pos = input.find_pos(|t| t == &Tile::Start).unwrap();
    let pipes = PipeIterator::new(input, start_pos).collect::<Vec<_>>();
    (pipes.len() + 1) / 2
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
