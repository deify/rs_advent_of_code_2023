use std::collections::BTreeMap;

trait Module {
    fn connections(&self) -> &[&'static str];
    fn process(&mut self, input: &'static str, pulse: bool);
    fn get_output(&self) -> bool;
}

pub struct FlipFlop {
    state: bool,
    connections: Vec<&'static str>,
}

impl Module for FlipFlop {
    fn process(&mut self, _input: &'static str, pulse: bool) {
        if !pulse {
            self.state = !self.state;
        }
    }
    fn get_output(&self) -> bool {
        self.state
    }

    fn connections(&self) -> &[&'static str] {
        &self.connections
    }
}

pub struct Conjunction {
    input_map: BTreeMap<&'static str, bool>,
    connections: Vec<&'static str>,
}
impl Module for Conjunction {
    fn process(&mut self, input: &'static str, pulse: bool) {
        self.input_map.insert(input, pulse);
    }
    fn get_output(&self) -> bool {
        self.input_map.values().all(|v| *v)
    }
    fn connections(&self) -> &[&'static str] {
        &self.connections
    }
}

// #[aoc_generator(day20)]
pub fn parse(input: &str) -> BTreeMap<&'static str, Box<dyn Module>> {
    BTreeMap::new()
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    0
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    const TEST_INPUT2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    // #[test]
    // fn test_parse() {
    //     assert_eq!("a", parse(TEST_INPUT));
    // }

    #[test]
    fn test_part1() {
        assert_eq!(32000000, part1(&parse(TEST_INPUT1)));
        assert_eq!(11687500, part1(&parse(TEST_INPUT2)));
    }

    #[test]
    fn solve_part1() {
        assert_eq!(1, part1(&parse(include_str!("../input/2023/day20.txt"))));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(&parse(TEST_INPUT1)));
    }

    #[test]
    fn solve_part2() {
        assert_eq!(1, part2(&parse(include_str!("../input/2023/day20.txt"))));
    }
}
