#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn calc_derivatives(values: &[Vec<i32>]) -> Vec<Vec<Vec<i32>>> {
    values
        .iter()
        .cloned()
        .map(|mut n| {
            let mut derivatives = Vec::new();
            derivatives.push(n.clone());
            loop {
                let d: Vec<_> = n.windows(2).map(|w| w[1] - w[0]).collect();
                n = d.clone();
                derivatives.push(d);

                if n.iter().all(|v| v == &0) {
                    break;
                }
            }
            derivatives
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<i32>]) -> i32 {
    let derivatives = calc_derivatives(input);
    derivatives
        .iter()
        .map(|d| d.iter().rev().fold(0, |last, v| v.last().unwrap() + last))
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<i32>]) -> i32 {
    let derivatives = calc_derivatives(input);
    derivatives
        .iter()
        .map(|d| d.iter().rev().fold(0, |last, v| v.first().unwrap() - last))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                vec![0, 3, 6, 9, 12, 15],
                vec![1, 3, 6, 10, 15, 21],
                vec![10, 13, 16, 21, 30, 45],
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(114, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(2, part2(&parse(TEST_INPUT)));
    }
}
