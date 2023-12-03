use std::{cmp::max, str::FromStr};

#[derive(Debug)]
pub enum ParseError {
    InvalidSet,
    InvalidGame,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Set {
    red: usize,
    green: usize,
    blue: usize,
}
impl Set {
    fn contains(&self, other: &Self) -> bool {
        other.red <= self.red && other.blue <= self.blue && other.green <= self.green
    }
    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn max_set(&self, other: &Self) -> Self {
        Set {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }
}

impl FromStr for Set {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = Set::default();
        for c in s.split(',') {
            let (count, color) = c.trim().split_once(' ').ok_or(ParseError::InvalidSet)?;
            let count: usize = count.parse().unwrap();
            match color {
                "red" => set.red = count,
                "blue" => set.blue = count,
                "green" => set.green = count,
                _ => return Err(ParseError::InvalidSet),
            }
        }
        Ok(set)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, sets) = s.split_once(':').ok_or(ParseError::InvalidGame)?;
        let id: usize = game
            .strip_prefix("Game ")
            .and_then(|x| x.parse().ok())
            .ok_or(ParseError::InvalidGame)?;
        let sets = sets
            .split(';')
            .map(Set::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, sets })
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| Game::from_str(line).expect("Failet do parse game"))
        .collect()
}

fn is_possible(game: &Game, set: &Set) -> bool {
    game.sets.iter().all(|s| set.contains(s))
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> usize {
    let set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .iter()
        .filter(|g| is_possible(g, &set))
        .map(|g| g.id)
        .sum()
}

fn find_min_set(game: &Game) -> Set {
    game.sets
        .iter()
        .cloned()
        .reduce(|acc, x| acc.max_set(&x))
        .unwrap()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> usize {
    input.iter().map(find_min_set).map(|s| s.power()).sum()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Game {
                    id: 1,
                    sets: vec![
                        Set {
                            blue: 3,
                            red: 4,
                            green: 0
                        },
                        Set {
                            blue: 6,
                            red: 1,
                            green: 2
                        },
                        Set {
                            blue: 0,
                            red: 0,
                            green: 2
                        },
                    ]
                },
                Game {
                    id: 2,
                    sets: vec![
                        Set {
                            blue: 1,
                            red: 0,
                            green: 2
                        },
                        Set {
                            blue: 4,
                            red: 1,
                            green: 3
                        },
                        Set {
                            blue: 1,
                            red: 0,
                            green: 1
                        },
                    ]
                },
                Game {
                    id: 3,
                    sets: vec![
                        Set {
                            blue: 6,
                            red: 20,
                            green: 8
                        },
                        Set {
                            blue: 5,
                            red: 4,
                            green: 13
                        },
                        Set {
                            blue: 0,
                            red: 1,
                            green: 5
                        },
                    ]
                },
                Game {
                    id: 4,
                    sets: vec![
                        Set {
                            blue: 6,
                            red: 3,
                            green: 1
                        },
                        Set {
                            blue: 0,
                            red: 6,
                            green: 3
                        },
                        Set {
                            blue: 15,
                            red: 14,
                            green: 3
                        },
                    ]
                },
                Game {
                    id: 5,
                    sets: vec![
                        Set {
                            blue: 1,
                            red: 6,
                            green: 3
                        },
                        Set {
                            blue: 2,
                            red: 1,
                            green: 2
                        },
                    ]
                },
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(8, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2286, part2(&parse(TEST_INPUT)));
    }
}
