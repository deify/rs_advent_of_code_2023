use std::str::FromStr;

use itertools::Itertools;

pub fn aoc_hash(input: &str) -> u8 {
    input.chars().fold(0u8, |hash, c| {
        assert!(c.is_ascii());

        let hash = hash.wrapping_add(c as u8);
        hash.wrapping_mul(17)
    })
}

#[derive(Debug)]
pub enum ParseError {
    InvalidLensOperation,
    InvalidLensInstruction,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LensOperation {
    Remove,
    Insert(u8),
}
impl FromStr for LensOperation {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Remove),
            _ => s
                .strip_prefix("=")
                .and_then(|s| s.parse::<u8>().ok().map(|u| Self::Insert(u)))
                .ok_or(ParseError::InvalidLensOperation),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LensInstruction {
    label: String,
    operation: LensOperation,
}
impl FromStr for LensInstruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_index = s
            .find('-')
            .or_else(|| s.find('='))
            .ok_or(ParseError::InvalidLensInstruction)?;

        let (l, o) = s.split_at(split_index);
        let label = l.to_string();
        let operation = LensOperation::from_str(o)?;
        Ok(Self { label, operation })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Lens {
    label: String,
    focal_length: u8,
}

#[aoc_generator(day15, part2)]
pub fn parse(input: &str) -> Vec<LensInstruction> {
    let input = input.replace('\n', "");
    input
        .split(',')
        .map(LensInstruction::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.replace('\n', "");
    input.split(',').map(|s| aoc_hash(s) as usize).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &[LensInstruction]) -> usize {
    let mut boxes: [Vec<Lens>; 256] = std::iter::repeat(vec![])
        .take(256)
        .collect_vec()
        .try_into()
        .unwrap();

    input.iter().for_each(|i| {
        let hash = aoc_hash(&i.label);
        let _box = boxes.get_mut(hash as usize).unwrap();

        match i.operation {
            LensOperation::Remove => {
                if let Some(lens_pos) = _box.iter().position(|l| l.label == i.label) {
                    _box.remove(lens_pos);
                }
            }
            LensOperation::Insert(focal_length) => {
                let new_lens = Lens {
                    label: i.label.clone(),
                    focal_length,
                };
                if let Some(lens) = _box.iter_mut().find(|l| l.label == i.label) {
                    *lens = new_lens;
                } else {
                    _box.push(new_lens)
                }
            }
        };
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| {
            (1 + i)
                * b.iter()
                    .enumerate()
                    .map(|(i, l)| (i + 1) * l.focal_length as usize)
                    .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(52, aoc_hash("HASH"))
    }

    #[test]
    fn test_parse() {
        use LensOperation::*;
        let expected: Vec<LensInstruction> = vec![
            LensInstruction {
                label: "rn".to_string(),
                operation: Insert(1),
            },
            LensInstruction {
                label: "cm".to_string(),
                operation: Remove,
            },
            LensInstruction {
                label: "qp".to_string(),
                operation: Insert(3),
            },
            LensInstruction {
                label: "cm".to_string(),
                operation: Insert(2),
            },
            LensInstruction {
                label: "qp".to_string(),
                operation: Remove,
            },
            LensInstruction {
                label: "pc".to_string(),
                operation: Insert(4),
            },
            LensInstruction {
                label: "ot".to_string(),
                operation: Insert(9),
            },
            LensInstruction {
                label: "ab".to_string(),
                operation: Insert(5),
            },
            LensInstruction {
                label: "pc".to_string(),
                operation: Remove,
            },
            LensInstruction {
                label: "pc".to_string(),
                operation: Insert(6),
            },
            LensInstruction {
                label: "ot".to_string(),
                operation: Insert(7),
            },
        ];
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1320, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(145, part2(&parse(TEST_INPUT)));
    }
}
