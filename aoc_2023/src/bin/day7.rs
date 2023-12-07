use std::cmp::Ordering;
use std::{collections::HashMap, fs};

use anyhow::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/day7.txt")?;

    let part1 = part_1(&input);
    let part2 = part_2(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
}

#[derive(Debug, PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind(Hand),
    FourOfAKind(Hand),
    FullHouse(Hand),
    ThreeOfAKind(Hand),
    TwoPair(Hand),
    OnePair(Hand),
    HighCard(Hand),
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            _ => Card::N(c.to_digit(10).unwrap() as u8),
        }
    }
}

impl Hand {
    fn from_str(hand: &str) -> Self {
        Hand {
            cards: hand.chars().map(Card::from_char).collect(),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Card::A, Card::A) => Some(Ordering::Equal),
            (Card::A, _) => Some(Ordering::Greater),
            (_, Card::A) => Some(Ordering::Less),

            (Card::K, Card::K) => Some(Ordering::Equal),
            (Card::K, _) => Some(Ordering::Greater),
            (_, Card::K) => Some(Ordering::Less),

            (Card::Q, Card::Q) => Some(Ordering::Equal),
            (Card::Q, _) => Some(Ordering::Greater),
            (_, Card::Q) => Some(Ordering::Less),

            (Card::T, Card::T) => Some(Ordering::Equal),
            (Card::T, _) => Some(Ordering::Greater),
            (_, Card::T) => Some(Ordering::Less),

            (Card::N(a), Card::N(b)) => a.partial_cmp(b),
            (Card::N(_), _) => Some(Ordering::Greater),
            (_, Card::N(_)) => Some(Ordering::Less),

            (Card::J, Card::J) => Some(Ordering::Equal),
        }
    }
}

impl HandType {
    fn from_hand(hand: &str, joker: bool) -> Self {
        let mut map = HashMap::new();

        if joker {
            let mut jokers = 0;
            for card in hand.chars() {
                if card == 'J' {
                    jokers += 1;
                    continue;
                }
                map.entry(card).and_modify(|e| *e += 1).or_insert(1);
            }

            if jokers == 5 {
                return HandType::FiveOfAKind(Hand::from_str(hand));
            }

            let max = map
                .iter()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .unwrap()
                .0;

            map.entry(*max)
                .and_modify(|e| *e += jokers)
                .or_insert(jokers);
        } else {
            for card in hand.chars() {
                map.entry(card).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        match map.len() {
            1 => HandType::FiveOfAKind(Hand::from_str(hand)),
            2 => {
                if map.values().any(|&v| v == 4) {
                    HandType::FourOfAKind(Hand::from_str(hand))
                } else {
                    HandType::FullHouse(Hand::from_str(hand))
                }
            }
            3 => {
                if map.values().any(|&v| v == 3) {
                    HandType::ThreeOfAKind(Hand::from_str(hand))
                } else {
                    HandType::TwoPair(Hand::from_str(hand))
                }
            }
            4 => HandType::OnePair(Hand::from_str(hand)),
            _ => HandType::HighCard(Hand::from_str(hand)),
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (HandType::FiveOfAKind(self_hand), HandType::FiveOfAKind(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::FiveOfAKind(_), _) => Some(Ordering::Greater),
            (_, HandType::FiveOfAKind(_)) => Some(Ordering::Less),

            (HandType::FourOfAKind(self_hand), HandType::FourOfAKind(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::FourOfAKind(_), _) => Some(Ordering::Greater),
            (_, HandType::FourOfAKind(_)) => Some(Ordering::Less),

            (HandType::FullHouse(self_hand), HandType::FullHouse(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::FullHouse(_), _) => Some(Ordering::Greater),
            (_, HandType::FullHouse(_)) => Some(Ordering::Less),

            (HandType::ThreeOfAKind(self_hand), HandType::ThreeOfAKind(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::ThreeOfAKind(_), _) => Some(Ordering::Greater),
            (_, HandType::ThreeOfAKind(_)) => Some(Ordering::Less),

            (HandType::TwoPair(self_hand), HandType::TwoPair(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::TwoPair(_), _) => Some(Ordering::Greater),
            (_, HandType::TwoPair(_)) => Some(Ordering::Less),

            (HandType::OnePair(self_hand), HandType::OnePair(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
            (HandType::OnePair(_), _) => Some(Ordering::Greater),
            (_, HandType::OnePair(_)) => Some(Ordering::Less),

            (HandType::HighCard(self_hand), HandType::HighCard(other_hand)) => {
                compare_eq_hand(self_hand, other_hand)
            }
        }
    }
}

fn compare_eq_hand(a: &Hand, b: &Hand) -> Option<Ordering> {
    for (a, b) in a.cards.iter().zip(&b.cards) {
        if a != b {
            return a.partial_cmp(b);
        }
    }

    Some(Ordering::Equal)
}

fn part_1(input: &str) -> u32 {
    let mut hands: Vec<(HandType, usize)> = Vec::new();

    for line in input.lines() {
        let hand = line.split_whitespace().next().unwrap();
        let bid = line.split_whitespace().nth(1).unwrap();
        let type_ = HandType::from_hand(hand, false);

        hands.push((type_, bid.parse().unwrap()));
    }

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid) as u32
}

fn part_2(input: &str) -> u32 {
    let mut hands: Vec<(HandType, usize)> = Vec::new();

    for line in input.lines() {
        let hand = line.split_whitespace().next().unwrap();
        let bid = line.split_whitespace().nth(1).unwrap();
        let type_ = HandType::from_hand(hand, true);

        hands.push((type_, bid.parse().unwrap()));
    }

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = fs::read_to_string("input/test/day7.txt")?;

        let result = part_1(&input);
        assert_eq!(result, 6440);

        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = fs::read_to_string("input/test/day7.txt")?;

        let result = part_2(&input);
        assert_eq!(result, 5905);

        Ok(())
    }
}
