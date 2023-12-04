use std::cmp::{max, min};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    x: usize,
    y: usize,
    len: usize,
}

impl Position {
    fn x2(&self) -> usize {
        self.x + self.len
    }

    fn x_overlap(&self, other: &Position) -> i32 {
        min(self.x2(), other.x2()) as i32 - max(self.x, other.x) as i32
    }

    fn next_to(&self, other: &Position) -> bool {
        match self.y.abs_diff(other.y) {
            0 => self.x_overlap(other) == 0,
            1 => self.x_overlap(other) >= 0,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SymbolOrNumber {
    Symbol { pos: Position, symbol: char },
    Number { pos: Position, value: usize },
}

fn find_symbols_or_numbers(line: &str, y: usize) -> Vec<SymbolOrNumber> {
    let mut ret = vec![];
    let mut line_chars = line.chars().enumerate().peekable();
    let line_char_ref = line_chars.by_ref();

    while let Some((x, c)) = line_char_ref.find(|(_, c)| c != &'.') {
        if c.is_ascii_digit() {
            // this is a number
            let len = std::iter::from_fn(|| line_char_ref.next_if(|(_, c)| c.is_ascii_digit()))
                .count()
                + 1;

            // let len = line_char_ref
            //     .take_while(|(_, c)| c.is_ascii_digit())
            //     .count()
            //     + 1;
            let pos = Position { x, y, len };
            ret.push(SymbolOrNumber::Number {
                pos,
                value: line[x..x + len].parse().unwrap(),
            });
        } else {
            // must be a symbol for now we assume len == 1
            let pos = Position { x, y, len: 1 };
            ret.push(SymbolOrNumber::Symbol { pos, symbol: c });
        }
    }
    ret
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<SymbolOrNumber> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| find_symbols_or_numbers(line, y))
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[SymbolOrNumber]) -> usize {
    input
        .iter()
        .map(|n| match n {
            SymbolOrNumber::Number {
                pos: num_pos,
                value,
            } => {
                if input.iter().any(|s| match s {
                    SymbolOrNumber::Symbol { pos: sym_pos, .. } => num_pos.next_to(sym_pos),
                    _ => false,
                }) {
                    value
                } else {
                    &0
                }
            }
            _ => &0,
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &[SymbolOrNumber]) -> usize {
    input
        .iter()
        .filter_map(|s| match s {
            SymbolOrNumber::Symbol {
                pos: gear_pos,
                symbol: '*',
            } => {
                let adjacent_num_iter = input.iter().filter_map(|n| match n {
                    SymbolOrNumber::Number {
                        pos: num_pos,
                        value,
                    } if num_pos.next_to(gear_pos) => Some(value),
                    _ => None,
                });
                if adjacent_num_iter.clone().count() == 2 {
                    Some(adjacent_num_iter.product::<usize>())
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse() {
        let v: Vec<SymbolOrNumber> = vec![
            SymbolOrNumber::Number {
                pos: Position { x: 0, y: 0, len: 3 },
                value: 467,
            },
            SymbolOrNumber::Number {
                pos: Position { x: 5, y: 0, len: 3 },
                value: 114,
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 3, y: 1, len: 1 },
                symbol: '*',
            },
            SymbolOrNumber::Number {
                pos: Position { x: 2, y: 2, len: 2 },
                value: 35,
            },
            SymbolOrNumber::Number {
                pos: Position { x: 6, y: 2, len: 3 },
                value: 633,
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 6, y: 3, len: 1 },
                symbol: '#',
            },
            SymbolOrNumber::Number {
                pos: Position { x: 0, y: 4, len: 3 },
                value: 617,
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 3, y: 4, len: 1 },
                symbol: '*',
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 5, y: 5, len: 1 },
                symbol: '+',
            },
            SymbolOrNumber::Number {
                pos: Position { x: 7, y: 5, len: 2 },
                value: 58,
            },
            SymbolOrNumber::Number {
                pos: Position { x: 2, y: 6, len: 3 },
                value: 592,
            },
            SymbolOrNumber::Number {
                pos: Position { x: 6, y: 7, len: 3 },
                value: 755,
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 3, y: 8, len: 1 },
                symbol: '$',
            },
            SymbolOrNumber::Symbol {
                pos: Position { x: 5, y: 8, len: 1 },
                symbol: '*',
            },
            SymbolOrNumber::Number {
                pos: Position { x: 1, y: 9, len: 3 },
                value: 664,
            },
            SymbolOrNumber::Number {
                pos: Position { x: 5, y: 9, len: 3 },
                value: 598,
            },
        ];
        assert_eq!(v, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(4361, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(467835, part2(&parse(TEST_INPUT)));
    }
}
