use core::panic;
use itertools::iproduct;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringStatus {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

impl Into<char> for SpringStatus {
    fn into(self) -> char {
        match self {
            Self::Operational => '.',
            Self::Damaged => '#',
            Self::Unknown => '?',
        }
    }
}

fn product<T: Clone>(vector: &[T], n: usize) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec![vec![]];

    for _ in 0..n {
        result = iproduct!(result.iter(), vector.iter())
            .map(|(v, x)| {
                let mut v1 = v.clone();
                v1.push(x.clone());
                v1
            })
            .collect();
    }

    result
}

#[derive(Debug)]
pub struct Report {
    visual: Vec<SpringStatus>,
    numeric: Vec<usize>,
}

fn visual_from_str(value: &str) -> Vec<SpringStatus> {
    value.chars().map(SpringStatus::from).collect()
}

fn visual_to_string(visual: &[SpringStatus]) -> String {
    visual
        .iter()
        .cloned()
        .map(std::convert::Into::<char>::into)
        .collect()
}

impl Report {
    fn matches_numeric(&self, visual: &[SpringStatus]) -> bool {
        if visual.iter().any(|v| v == &SpringStatus::Unknown) {
            return false;
        }

        let mut v_iter = visual.iter().peekable();

        let all_matching = self
            .numeric
            .iter()
            .map(|n| {
                let _ = v_iter
                    .peeking_take_while(|s| s == &&SpringStatus::Operational)
                    .count();
                *n == v_iter
                    .peeking_take_while(|s| s == &&SpringStatus::Damaged)
                    .count()
            })
            .all(|b| b);

        all_matching && !v_iter.any(|s| s == &SpringStatus::Damaged)
    }

    fn iter_matches_bruteforce(&self) -> impl Iterator<Item = Vec<SpringStatus>> + '_ {
        let unknown = self
            .visual
            .iter()
            .filter(|s| s == &&SpringStatus::Unknown)
            .count();

        let combinations = product(&[SpringStatus::Operational, SpringStatus::Damaged], unknown);

        combinations
            .into_iter()
            .map(|u| {
                let mut v = self.visual.clone();
                v.iter_mut()
                    .filter(|s| s == &&SpringStatus::Unknown)
                    .zip(u)
                    .for_each(|(x, u)| *x = u);
                v
            })
            .filter(|visual| self.matches_numeric(visual))
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidReport,
    InvalidVisual,
}

impl FromStr for Report {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (v, n) = s.split_once(' ').ok_or(ParseError::InvalidReport)?;
        let visual = visual_from_str(v);

        let numeric = n
            .split(',')
            .map(|s| s.parse().map_err(|_| ParseError::InvalidReport))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { visual, numeric })
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|l| Report::from_str(l).unwrap())
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Report]) -> usize {
    input
        .iter()
        .map(|r| r.iter_matches_bruteforce().count())
        .sum()

    // input.iter().for_each(|r| {
    //     let solutions = r.iter_matches_bruteforce().collect_vec();
    //     println!("{:?}", visual_to_string(&r.visual));
    //     println!("solutions ({}): ", solutions.len());
    //     solutions.iter().for_each(|s| {
    //         println!("{}", visual_to_string(s));
    //     });
    //     println!("\n");
    // });
    // 0
}

#[aoc(day12, part2)]
pub fn part2(input: &[Report]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    // #[test]
    // fn test_parse() {
    //     assert_eq!("a", parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_matches_numeric() {
        let r = Report {
            visual: vec![],
            numeric: vec![1, 1, 3],
        };

        let v1 = visual_from_str(".#...#....###.");
        let v2 = visual_from_str("#.#.###");
        let v3 = visual_from_str("##.###");

        assert!(r.matches_numeric(&v1));
        assert!(r.matches_numeric(&v2));
        assert!(!r.matches_numeric(&v3));

        let r = Report {
            visual: vec![],
            numeric: vec![3, 2, 1],
        };

        let v1 = visual_from_str(".###.##.#.##");
        assert!(!r.matches_numeric(&v1));
    }

    #[test]
    fn test_combinations_of_two() {
        let n = 3;
        let input = ['a', 'b'];
        let a = product(&input, n);
        let b: Vec<Vec<char>> = vec![
            vec!['a', 'a', 'a'],
            vec!['a', 'a', 'b'],
            vec!['a', 'b', 'a'],
            vec!['a', 'b', 'b'],
            vec!['b', 'a', 'a'],
            vec!['b', 'a', 'b'],
            vec!['b', 'b', 'a'],
            vec!['b', 'b', 'b'],
        ];
        assert_eq!(a, b)
    }
}
