use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Debug)]
pub enum ParseError {
    InvalidCard,
    InvalidHand,
    InvalidHandWithBet,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Ass = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl TryFrom<char> for Card {
    type Error = ParseError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ass),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(ParseError::InvalidCard),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()? //TODO: any chance to avoid this alloc?
            .try_into()
            .map_err(|_| ParseError::InvalidHand)?;
        Ok(Hand(cards))
    }
}

impl Hand {
    fn get_type(&self) -> Type {
        let map = self.0.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card).or_insert(0usize) += 1;
            map
        });

        let mut sorted_values: BinaryHeap<_> = map.values().collect();

        match sorted_values.pop().unwrap() {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => match sorted_values.pop().unwrap() {
                2 => Type::FullHouse,
                _ => Type::ThreeOfAKind,
            },
            2 => match sorted_values.pop().unwrap() {
                2 => Type::TwoPair,
                _ => Type::OnePair,
            },
            _ => Type::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();

        match &self_type.cmp(&other_type) {
            Ordering::Equal => {
                let zipped = self.0.iter().zip(other.0.iter());
                // get the tie breaker
                zipped
                    .map(|(a, b)| a.cmp(b))
                    .find(|cmp| cmp != &Ordering::Equal)
                    .unwrap_or(Ordering::Equal)
            }
            other => *other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HandWithBid {
    hand: Hand,
    bid: usize,
}
impl FromStr for HandWithBid {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').ok_or(ParseError::InvalidHandWithBet)?;
        let hand = Hand::from_str(hand)?;
        let bid = bid
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidHandWithBet)?;

        Ok(HandWithBid { hand, bid })
    }
}

impl PartialOrd for HandWithBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandWithBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<HandWithBid> {
    input
        .lines()
        .map(HandWithBid::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day7, part1)]
pub fn part1(input: &[HandWithBid]) -> usize {
    let len = input.len();

    // TODO: could be an iterator with nightly
    let sorted = input.iter().collect::<BinaryHeap<_>>().into_sorted_vec();

    sorted.iter().enumerate().fold(0, |acc, (i, h)| {
        let rank = i + 1;
        println!(
            "{}: {:?} -> {:?} bid: {}",
            rank,
            h.hand.get_type(),
            h.hand,
            h.bid
        );
        acc + h.bid * rank
    })
}

#[aoc(day7, part2)]
pub fn part2(input: &[HandWithBid]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse() {
        let expected: Vec<HandWithBid> = vec![
            HandWithBid {
                hand: Hand([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
                bid: 765,
            },
            HandWithBid {
                hand: Hand([Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]),
                bid: 684,
            },
            HandWithBid {
                hand: Hand([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
                bid: 28,
            },
            HandWithBid {
                hand: Hand([Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]),
                bid: 220,
            },
            HandWithBid {
                hand: Hand([Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ass]),
                bid: 483,
            },
        ];
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(6440, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5905, part2(&parse(TEST_INPUT)));
    }
}
