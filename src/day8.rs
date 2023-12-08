use std::{collections::BTreeMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct NodeMap {
    lr: String,
    node_map: BTreeMap<String, (String, String)>,
}

impl NodeMap {
    fn iter_nodes(&self) -> NodeMapIter {
        let start_node = self.node_map.first_key_value().unwrap().0;
        self.iter_nodes_from(start_node)
    }
    fn iter_nodes_from<'a>(&'a self, node: &'a str) -> NodeMapIter<'a> {
        let lr_iter = Box::new(self.lr.chars().cycle());
        NodeMapIter {
            n: self,
            start_node: node,
            lr_iter,
        }
    }
}

pub struct NodeMapIter<'a> {
    n: &'a NodeMap,
    start_node: &'a str,
    lr_iter: Box<dyn Iterator<Item = char> + 'a>,
}

impl<'a> Iterator for NodeMapIter<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let choices = self.n.node_map.get(self.start_node)?;
        let dest_node = match self.lr_iter.next()? {
            'L' => &choices.0,
            'R' => &choices.1,
            _ => panic!(),
        };
        self.start_node = dest_node;
        Some(dest_node)
    }
}

impl NodeMap {}

#[derive(Debug)]
pub enum ParseErr {
    InvalidNodeMap,
}

impl FromStr for NodeMap {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();
        let lr = line_iter
            .by_ref()
            .next()
            .ok_or(ParseErr::InvalidNodeMap)?
            .to_string();

        let node_map = line_iter
            .skip(1)
            .map(|l| {
                let (key, values) = l.split_once('=').unwrap(); //TODO
                let (a, b) = values.trim().split_once(',').unwrap();
                let a = a.strip_prefix('(').map(|s| s.trim()).unwrap();
                let b = b.strip_suffix(')').map(|s| s.trim()).unwrap();
                (key.trim().to_string(), (a.to_string(), b.to_string()))
            })
            .collect();

        Ok(NodeMap { lr, node_map })
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> NodeMap {
    NodeMap::from_str(input).unwrap()
}

#[aoc(day8, part1)]
pub fn part1(input: &NodeMap) -> usize {
    let a = input.iter_nodes().enumerate().find(|(i, s)| *s == "ZZZ");
    a.unwrap().0 + 1
}

#[aoc(day8, part2)]
pub fn part2(input: &NodeMap) -> usize {
    let start_node_iters = input
        .node_map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|n| input.iter_nodes_from(n));

    start_node_iters
        .map(|mut i| {
            let (_, end_node) = i
                .by_ref()
                .enumerate()
                .find(|(i, k)| k.ends_with('Z'))
                .unwrap();
            let period = i
                .by_ref()
                .enumerate()
                .find(|(i, k)| *k == end_node)
                .unwrap()
                .0
                + 1;
            period
        })
        .reduce(num::integer::lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_parse() {
        assert_eq!(
            NodeMap {
                lr: "LLR".to_owned(),
                node_map: BTreeMap::from([
                    ("AAA".to_owned(), ("BBB".to_owned(), "BBB".to_owned())),
                    ("BBB".to_owned(), ("AAA".to_owned(), "ZZZ".to_owned())),
                    ("ZZZ".to_owned(), ("ZZZ".to_owned(), "ZZZ".to_owned()))
                ])
            },
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&parse(TEST_INPUT2)));
    }
}
