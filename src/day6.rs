#[derive(Debug, PartialEq, Eq)]
pub struct Race {
    time: usize,
    record_distance: usize,
}

impl Race {
    fn calc_distance(&self, init_speed: usize) -> usize {
        (self.time - init_speed) * init_speed
    }

    fn winning_init_speeds(&self) -> impl Iterator<Item = usize> + '_ {
        (0..=self.time)
            .map(|init_speed| self.calc_distance(init_speed))
            .filter(|distance| distance > &self.record_distance)
    }
}

#[aoc_generator(day6, part1)]
pub fn parse(input: &str) -> Vec<Race> {
    let mut line_iter = input.lines();

    let times = line_iter
        .by_ref()
        .next()
        .and_then(|s| s.strip_prefix("Time:"))
        .map(|s| {
            s.trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
        })
        .unwrap();

    let distances = line_iter
        .by_ref()
        .next()
        .and_then(|s| s.strip_prefix("Distance:"))
        .map(|s| {
            s.trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
        })
        .unwrap();

    times
        .zip(distances)
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect()
}

#[aoc_generator(day6, part2)]
pub fn parse2(input: &str) -> Race {
    let mut lines_iter = input.lines();
    let time = lines_iter
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .map(|s| {
            let mut s = s.to_string();
            s.retain(|c| !c.is_ascii_whitespace());
            s
        })
        .map(|s| s.parse().unwrap())
        .unwrap();

    let record_distance = lines_iter
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .map(|s| {
            let mut s = s.to_string();
            s.retain(|c| !c.is_ascii_whitespace());
            s
        })
        .map(|s| s.parse().unwrap())
        .unwrap();

    Race {
        time,
        record_distance,
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &[Race]) -> usize {
    input
        .iter()
        .map(|race| race.winning_init_speeds().count())
        .product()
}

#[aoc(day6, part2)]
pub fn part2(race: &Race) -> usize {
    race.winning_init_speeds().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let expected = vec![
            Race {
                time: 7,
                record_distance: 9,
            },
            Race {
                time: 15,
                record_distance: 40,
            },
            Race {
                time: 30,
                record_distance: 200,
            },
        ];
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(288, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(71503, part2(&parse2(TEST_INPUT)));
    }
}
