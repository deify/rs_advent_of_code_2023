use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn matching_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter(|&n| self.winning_numbers.iter().any(|wn| wn == n))
            .count()
    }
    fn points(&self) -> usize {
        let matching_numbers = self.matching_numbers();
        if matching_numbers == 0 {
            0
        } else {
            2usize.pow((matching_numbers - 1) as u32)
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidCard,
}

impl FromStr for Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (c, numbers) = s.split_once(':').ok_or(ParseError::InvalidCard)?;
        let id = c
            .strip_prefix("Card ")
            .and_then(|id| id.trim().parse().ok())
            .ok_or(ParseError::InvalidCard)?;
        let (wn, n) = numbers.split_once('|').ok_or(ParseError::InvalidCard)?;
        let winning_numbers = wn
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError::InvalidCard)?;
        let numbers = n
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ParseError::InvalidCard)?;

        Ok(Card {
            id,
            winning_numbers,
            numbers,
        })
    }
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Card]) -> usize {
    input.iter().map(Card::points).sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Card]) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();

    input.iter().for_each(|c| {
        let count = map.entry(c.id).or_insert(0);
        *count += 1;

        let multiplier = *count;
        (c.id + 1..=c.id + c.matching_numbers())
            .for_each(|id| *map.entry(id).or_insert(0) += multiplier)
    });

    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        let expected: Vec<Card> = vec![
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
            },
            Card {
                id: 2,
                winning_numbers: vec![13, 32, 20, 16, 61],
                numbers: vec![61, 30, 68, 82, 17, 32, 24, 19],
            },
            Card {
                id: 3,
                winning_numbers: vec![1, 21, 53, 59, 44],
                numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            },
            Card {
                id: 4,
                winning_numbers: vec![41, 92, 73, 84, 69],
                numbers: vec![59, 84, 76, 51, 58, 5, 54, 83],
            },
            Card {
                id: 5,
                winning_numbers: vec![87, 83, 26, 28, 32],
                numbers: vec![88, 30, 70, 12, 93, 22, 82, 36],
            },
            Card {
                id: 6,
                winning_numbers: vec![31, 18, 13, 56, 72],
                numbers: vec![74, 77, 10, 23, 35, 67, 36, 11],
            },
        ];
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, part2(&parse(TEST_INPUT)));
    }
}
