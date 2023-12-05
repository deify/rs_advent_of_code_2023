use std::str::FromStr;
#[derive(Debug)]
pub enum ParseError {
    InvalidMapRange,
    InvalidMap,
    InvalidAlmanac,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MapRange {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize,
}

impl MapRange {
    fn to_destination(&self, source: &usize) -> Option<usize> {
        let source_range = self.source_range_start..self.source_range_start + self.range_length;
        if !source_range.contains(source) {
            None
        } else {
            Some(self.destination_range_start + source - self.source_range_start)
        }
    }
}

impl FromStr for MapRange {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .trim()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().map_err(|_| ParseError::InvalidMapRange));
        let destination_range_start = iter.next().ok_or(ParseError::InvalidMapRange)??;
        let source_range_start = iter.next().ok_or(ParseError::InvalidMapRange)??;
        let range_length = iter.next().ok_or(ParseError::InvalidMapRange)??;

        Ok(MapRange {
            destination_range_start,
            source_range_start,
            range_length,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Map {
    name: String,
    map_ranges: Vec<MapRange>,
}

impl Map {
    fn to_destination(&self, source: &usize) -> usize {
        self.map_ranges
            .iter()
            .find_map(|m| m.to_destination(source))
            .unwrap_or(*source)
    }
}

impl FromStr for Map {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines();
        let name = iter
            .next()
            .and_then(|s| s.strip_suffix(':'))
            .ok_or(ParseError::InvalidMap)?
            .to_string();
        let map_ranges = iter
            .map(MapRange::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Map { name, map_ranges })
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl Almanac {
    fn apply_map(sources: &[usize], m: &Map) -> Vec<usize> {
        sources.iter().map(|s| m.to_destination(s)).collect()
    }
}

impl FromStr for Almanac {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines_iter = s.lines();
        let seeds = lines_iter
            .by_ref()
            .next()
            .and_then(|s| s.strip_prefix("seeds: "))
            .map(|s| s.trim())
            .and_then(|s| {
                s.split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .ok_or(ParseError::InvalidAlmanac)?;

        lines_iter.by_ref().next(); //drop empty line

        let mut maps = vec![];
        loop {
            let map_string = lines_iter
                .by_ref()
                .take_while(|l| !l.is_empty())
                .fold(String::new(), |a, b| a + b + "\n");
            if map_string.is_empty() {
                break;
            }
            maps.push(Map::from_str(&map_string)?);
        }
        Ok(Almanac { seeds, maps })
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Almanac {
    Almanac::from_str(input).unwrap()
}

fn solve(init_seeds: Vec<usize>, maps: &[Map]) -> usize {
    *maps
        .iter()
        .fold(init_seeds, |sources, map| Almanac::apply_map(&sources, map))
        .iter()
        .min()
        .unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> usize {
    solve(input.seeds.clone(), &input.maps)
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanac) -> usize {
    input
        .seeds
        .chunks(2)
        .map(|c| {
            let seeds: Vec<_> = (c[0]..c[0] + c[1]).collect();
            solve(seeds, &input.maps)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_parse() {
        assert_eq!(
            Almanac {
                seeds: vec![79, 14, 55, 13],
                maps: vec![
                    Map {
                        name: "seed-to-soil map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 50,
                                source_range_start: 98,
                                range_length: 2
                            },
                            MapRange {
                                destination_range_start: 52,
                                source_range_start: 50,
                                range_length: 48
                            }
                        ]
                    },
                    Map {
                        name: "soil-to-fertilizer map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 0,
                                source_range_start: 15,
                                range_length: 37
                            },
                            MapRange {
                                destination_range_start: 37,
                                source_range_start: 52,
                                range_length: 2
                            },
                            MapRange {
                                destination_range_start: 39,
                                source_range_start: 0,
                                range_length: 15
                            }
                        ]
                    },
                    Map {
                        name: "fertilizer-to-water map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 49,
                                source_range_start: 53,
                                range_length: 8
                            },
                            MapRange {
                                destination_range_start: 0,
                                source_range_start: 11,
                                range_length: 42
                            },
                            MapRange {
                                destination_range_start: 42,
                                source_range_start: 0,
                                range_length: 7
                            },
                            MapRange {
                                destination_range_start: 57,
                                source_range_start: 7,
                                range_length: 4
                            }
                        ]
                    },
                    Map {
                        name: "water-to-light map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 88,
                                source_range_start: 18,
                                range_length: 7
                            },
                            MapRange {
                                destination_range_start: 18,
                                source_range_start: 25,
                                range_length: 70
                            }
                        ]
                    },
                    Map {
                        name: "light-to-temperature map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 45,
                                source_range_start: 77,
                                range_length: 23
                            },
                            MapRange {
                                destination_range_start: 81,
                                source_range_start: 45,
                                range_length: 19
                            },
                            MapRange {
                                destination_range_start: 68,
                                source_range_start: 64,
                                range_length: 13
                            }
                        ]
                    },
                    Map {
                        name: "temperature-to-humidity map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 0,
                                source_range_start: 69,
                                range_length: 1
                            },
                            MapRange {
                                destination_range_start: 1,
                                source_range_start: 0,
                                range_length: 69
                            }
                        ]
                    },
                    Map {
                        name: "humidity-to-location map".to_string(),
                        map_ranges: vec![
                            MapRange {
                                destination_range_start: 60,
                                source_range_start: 56,
                                range_length: 37
                            },
                            MapRange {
                                destination_range_start: 56,
                                source_range_start: 93,
                                range_length: 4
                            }
                        ]
                    }
                ]
            },
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(35, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(46, part2(&parse(TEST_INPUT)));
    }
}
