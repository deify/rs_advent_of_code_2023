use itertools::Itertools;
use std::str::FromStr;
use std::vec;

use crate::day10::Direction;
use crate::grid::Grid;

#[derive(Debug, Default, PartialEq, Eq, Clone, Hash)]
pub enum Element {
    RoundedRock,
    CubeRock,
    #[default]
    EmptySpace,
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            'O' => Self::RoundedRock,
            '#' => Self::CubeRock,
            '.' => Self::EmptySpace,
            _ => panic!(),
        }
    }
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        match value {
            Element::RoundedRock => 'O',
            Element::CubeRock => '#',
            Element::EmptySpace => '.',
        }
    }
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> Grid<Element> {
    Grid::from_str(input).unwrap()
}

fn roll_rocks(elements: &mut [&mut Element]) {
    let rolling_rocks = elements
        .iter()
        .positions(|e| e == &&Element::RoundedRock)
        .collect_vec();

    for rock in rolling_rocks {
        if let Some(destination) = elements[..rock]
            .iter_mut()
            .rev()
            .take_while(|e| e == &&&Element::EmptySpace)
            .last()
        {
            **destination = Element::RoundedRock;
            *elements[rock] = Element::EmptySpace;
        }
    }
}

fn roll_all_rocks(grid: &mut Grid<Element>, dir: Direction) {
    match dir {
        Direction::North => (0..grid.columns).for_each(|c| {
            let mut col = grid.iter_col_mut(c).collect_vec();
            roll_rocks(&mut col);
        }),
        Direction::South => (0..grid.columns).for_each(|c| {
            let mut col = grid.iter_col_mut(c).collect_vec();
            col.reverse();
            roll_rocks(&mut col);
        }),
        Direction::West => (0..grid.rows()).for_each(|c| {
            let mut row = grid.iter_row_mut(c).collect_vec();
            roll_rocks(&mut row);
        }),
        Direction::East => (0..grid.rows()).for_each(|c| {
            let mut row = grid.iter_row_mut(c).collect_vec();
            row.reverse();
            roll_rocks(&mut row);
        }),
    }
}

fn cycle(grid: &mut Grid<Element>) {
    roll_all_rocks(grid, Direction::North);
    roll_all_rocks(grid, Direction::West);
    roll_all_rocks(grid, Direction::South);
    roll_all_rocks(grid, Direction::East);
}

fn calc_north_beam_load(grid: &Grid<Element>) -> usize {
    grid.iter_rows()
        .enumerate()
        .map(|(i, r)| r.iter().filter(|e| e == &&&Element::RoundedRock).count() * (grid.rows() - i))
        .sum()
}

#[aoc(day14, part1)]
pub fn part1(input: &Grid<Element>) -> usize {
    let mut grid = input.clone();
    roll_all_rocks(&mut grid, Direction::North);
    calc_north_beam_load(&grid)
}

#[aoc(day14, part2)]
pub fn part2(input: &Grid<Element>) -> usize {
    let mut grid = input.clone();
    let count: usize = 1000000000;

    let mut grids: Vec<Grid<Element>> = vec![grid.clone()];
    let mut grid_id = None;

    for i in 1..=count {
        cycle(&mut grid);
        if let Some((j, _)) = grids.iter().enumerate().find(|(_, g)| g == &&grid) {
            let first_occ = j;
            let second_occ = i;
            let repeat = second_occ - first_occ;

            grid_id = Some(first_occ + (count - first_occ) % repeat);
            break;
        } else {
            grids.push(grid.clone());
        }
    }

    calc_north_beam_load(&grids[grid_id.unwrap()])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    // #[test]
    // fn test_parse() {
    //     let grid: Grid<Element> = Grid::default();
    //     assert_eq!(grid, parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(136, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(64, part2(&parse(TEST_INPUT)));
    }
}
