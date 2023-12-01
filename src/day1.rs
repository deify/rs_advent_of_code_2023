const STR_DIGITS_PART1: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];

const STR_DIGITS_PART2: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", //
    "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

fn str_to_digit(str: &str) -> Option<usize> {
    match str {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

fn solve(input: &str, str_digits: &[&str]) -> usize {
    input
        .lines()
        .map(|line| {
            // for each line
            // find the first digit
            let first = str_digits
                .iter()
                // enumerate all possible str_digits
                .enumerate()
                // find the position of all possible str_digits from left
                .filter_map(|(i, p)| line.find(p).map(|start| (i, start)))
                // take the str_digit with the lowest index returned by find
                .min_by_key(|v| v.1)
                .map(|(i, _)| str_digits[i])
                .unwrap();
            let last = str_digits
                .iter()
                // enumerate all possible str_digits
                .enumerate()
                // find the position of all possible str_digits from right
                .filter_map(|(i, p)| line.rfind(p).map(|start| (i, start)))
                // take the str_digit with the highest index returned by rfind
                .max_by_key(|v| v.1)
                .map(|(i, _)| str_digits[i])
                .unwrap();

            str_to_digit(first).unwrap() * 10 + str_to_digit(last).unwrap()
        })
        .sum()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    solve(input, STR_DIGITS_PART1)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    solve(input, STR_DIGITS_PART2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const TEST_INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part1() {
        assert_eq!(142, part1(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(281, part2(TEST_INPUT2));
    }
}
