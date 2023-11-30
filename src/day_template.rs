// #[aoc_generator(day1)]
pub fn parse(input: &str) -> &str {
    input
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_parse() {
        assert_eq!("a", parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(1, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT)));
    }
}
