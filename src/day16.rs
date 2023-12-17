use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use std::str::FromStr;

use crate::grid::{Direction, Grid, Position};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Element {
    #[default]
    EmptySpace,
    MirrorForward,
    MirrorBackward,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<Element> for char {
    fn from(value: Element) -> Self {
        match value {
            Element::EmptySpace => '.',
            Element::MirrorForward => '/',
            Element::MirrorBackward => '\\',
            Element::HorizontalSplitter => '-',
            Element::VerticalSplitter => '|',
        }
    }
}

impl From<char> for Element {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::EmptySpace,
            '/' => Self::MirrorForward,
            '\\' => Self::MirrorBackward,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => panic!(),
        }
    }
}

pub enum LightAction {
    Unchanged,
    Redirect(Direction),
    Split(Direction, Direction),
}

impl Element {
    fn procss_light_beam(&self, moving_dir: Direction) -> LightAction {
        use Direction::*;
        use LightAction::*;

        match self {
            Self::EmptySpace => Unchanged,
            Self::MirrorForward => match moving_dir {
                North => Redirect(East),
                East => Redirect(North),
                South => Redirect(West),
                West => Redirect(South),
            },
            Self::MirrorBackward => match moving_dir {
                North => Redirect(West),
                East => Redirect(South),
                South => Redirect(East),
                West => Redirect(North),
            },
            Self::VerticalSplitter => match moving_dir {
                North => Unchanged,
                East => Split(North, South),
                South => Unchanged,
                West => Split(North, South),
            },
            Self::HorizontalSplitter => match moving_dir {
                North => Split(West, East),
                East => Unchanged,
                South => Split(West, East),
                West => Unchanged,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct EnergizedElement {
    element: Element,
    energy_count: usize,
}
impl From<char> for EnergizedElement {
    fn from(value: char) -> Self {
        Self {
            element: Element::from(value),
            energy_count: 0,
        }
    }
}
impl From<EnergizedElement> for char {
    fn from(value: EnergizedElement) -> Self {
        match value.element {
            Element::EmptySpace if value.energy_count > 0 => '#',
            _ => char::from(value.element),
        }
    }
}

#[derive(Debug)]
pub struct LightBeam {
    position: Position,
    direction: Direction,
}
impl LightBeam {
    fn move_beam(&mut self) -> bool {
        self.position.move_dir(self.direction)
    }
}

pub struct LightBeamIter<'a> {
    grid: &'a mut Grid<EnergizedElement>,
    beams: Vec<LightBeam>,
}

impl<'a> Iterator for LightBeamIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut beams_to_remove = vec![];
        let mut new_beams = vec![];

        self.beams.iter_mut().enumerate().for_each(|(i, b)| {
            if let Some(element) = self.grid.at_mut(&b.position) {
                match element.element.procss_light_beam(b.direction) {
                    LightAction::Unchanged => (),
                    LightAction::Redirect(dir) => b.direction = dir,
                    LightAction::Split(dir_a, dir_b) => {
                        b.direction = dir_b;
                        new_beams.push(LightBeam {
                            position: b.position,
                            direction: dir_a,
                        });
                    }
                }
                element.energy_count += 1;
            } else {
                // remove out of bounds beams
                beams_to_remove.push(i);
            }
        });
        while let Some(i) = beams_to_remove.pop() {
            self.beams.remove(i);
        }

        self.beams.append(&mut new_beams);

        self.beams.iter_mut().enumerate().for_each(|(i, b)| {
            if !b.move_beam() {
                beams_to_remove.push(i);
            }
        });

        while let Some(i) = beams_to_remove.pop() {
            self.beams.remove(i);
        }

        if self.beams.is_empty() {
            None
        } else {
            // println!("{}", self.grid);
            Some(get_energy_level(self.grid))
        }
    }
}

#[aoc_generator(day16)]
pub fn parse(input: &str) -> Grid<EnergizedElement> {
    Grid::from_str(input).unwrap()
}

fn get_energy_level(grid: &Grid<EnergizedElement>) -> usize {
    grid.iter().filter(|e| e.energy_count > 0).count()
}

fn process_light_beams(grid: &mut Grid<EnergizedElement>, init_beam: LightBeam) {
    let beam_iter = LightBeamIter {
        grid,
        beams: vec![init_beam],
    };
    let mut old_energy = 0;
    let mut steady_count = 0;
    for energy in beam_iter {
        if old_energy == energy {
            steady_count += 1;
            if steady_count > 10 {
                return;
            }
        } else {
            steady_count = 0;
        }
        old_energy = energy;
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Grid<EnergizedElement>) -> usize {
    let mut grid = input.clone();

    let init_beam = LightBeam {
        position: Position { x: 0, y: 0 },
        direction: Direction::East,
    };
    process_light_beams(&mut grid, init_beam);
    get_energy_level(&grid)
}

#[aoc(day16, part2)]
pub fn part2(input: &Grid<EnergizedElement>) -> usize {
    let beams = (0..input.columns).map(|y| LightBeam {
        position: Position { x: 0, y },
        direction: Direction::East,
    });
    let beams = beams.chain((0..input.columns).map(|y| LightBeam {
        position: Position {
            x: input.rows() - 1,
            y,
        },
        direction: Direction::West,
    }));
    let beams = beams.chain((0..input.rows()).map(|x| LightBeam {
        position: Position { x, y: 0 },
        direction: Direction::South,
    }));
    let beams = beams.chain((0..input.rows()).map(|x| LightBeam {
        position: Position {
            x,
            y: input.columns - 1,
        },
        direction: Direction::North,
    }));

    beams
        .collect_vec()
        .into_par_iter()
        .progress()
        .map(|b| {
            let mut g = input.clone();
            process_light_beams(&mut g, b);
            get_energy_level(&g)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_parse() {
        let g = parse(TEST_INPUT);

        assert_eq!(g.to_string().trim_end(), TEST_INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(46, part1(&parse(TEST_INPUT)));
    }
    #[test]
    fn solve_part1() {
        assert_eq!(6605, part1(&parse(include_str!("../input/2023/day16.txt"))));
    }

    #[test]
    fn test_part2() {
        assert_eq!(51, part2(&parse(TEST_INPUT)));
    }
}
