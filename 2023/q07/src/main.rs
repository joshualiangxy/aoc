use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use std::{
    cmp::Ordering, collections::HashMap, env::current_dir, fs::read_to_string, path::PathBuf,
};

#[derive(Debug, Parser)]
#[clap(name = "q07", version)]
struct App {
    #[clap(subcommand)]
    parts: Parts,
}

#[derive(Debug, Subcommand)]
enum Parts {
    /// Problem for part 1
    P1 {
        /// The path to read from
        path: Utf8PathBuf,
    },
    /// Problem for part 2
    P2 {
        /// The path to read from
        path: Utf8PathBuf,
    },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum P1Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Number(u8),
}

impl PartialOrd for P1Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for P1Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (P1Card::Ace, P1Card::Ace)
            | (P1Card::King, P1Card::King)
            | (P1Card::Queen, P1Card::Queen)
            | (P1Card::Jack, P1Card::Jack)
            | (P1Card::Ten, P1Card::Ten) => Ordering::Equal,
            (P1Card::Ace, _) => Ordering::Greater,
            (_, P1Card::Ace) => Ordering::Less,
            (P1Card::King, _) => Ordering::Greater,
            (_, P1Card::King) => Ordering::Less,
            (P1Card::Queen, _) => Ordering::Greater,
            (_, P1Card::Queen) => Ordering::Less,
            (P1Card::Jack, _) => Ordering::Greater,
            (_, P1Card::Jack) => Ordering::Less,
            (P1Card::Ten, _) => Ordering::Greater,
            (_, P1Card::Ten) => Ordering::Less,
            (P1Card::Number(s), P1Card::Number(o)) => s.cmp(o),
        }
    }
}

impl P1Card {
    fn new(c: char) -> P1Card {
        match c {
            '2'..='9' => P1Card::Number(c.to_digit(10).unwrap().try_into().unwrap()),
            'T' => P1Card::Ten,
            'J' => P1Card::Jack,
            'Q' => P1Card::Queen,
            'K' => P1Card::King,
            'A' => P1Card::Ace,
            _ => unreachable!("Should not happen!"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum P2Card {
    Ace,
    King,
    Queen,
    Ten,
    Number(u8),
    Jack,
}

impl PartialOrd for P2Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for P2Card {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (P2Card::Ace, P2Card::Ace)
            | (P2Card::King, P2Card::King)
            | (P2Card::Queen, P2Card::Queen)
            | (P2Card::Jack, P2Card::Jack)
            | (P2Card::Ten, P2Card::Ten) => Ordering::Equal,
            (P2Card::Ace, _) => Ordering::Greater,
            (_, P2Card::Ace) => Ordering::Less,
            (P2Card::King, _) => Ordering::Greater,
            (_, P2Card::King) => Ordering::Less,
            (P2Card::Queen, _) => Ordering::Greater,
            (_, P2Card::Queen) => Ordering::Less,
            (P2Card::Ten, _) => Ordering::Greater,
            (_, P2Card::Ten) => Ordering::Less,
            (P2Card::Number(s), P2Card::Number(o)) => s.cmp(o),
            (P2Card::Jack, P2Card::Number(_)) => Ordering::Less,
            (P2Card::Number(_), P2Card::Jack) => Ordering::Greater,
        }
    }
}

impl P2Card {
    fn new(c: char) -> P2Card {
        match c {
            '2'..='9' => P2Card::Number(c.to_digit(10).unwrap().try_into().unwrap()),
            'T' => P2Card::Ten,
            'J' => P2Card::Jack,
            'Q' => P2Card::Queen,
            'K' => P2Card::King,
            'A' => P2Card::Ace,
            _ => unreachable!("Should not happen!"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum P1Hand {
    FiveOfAKind { cards: Vec<P1Card> },
    FourOfAKind { cards: Vec<P1Card> },
    FullHouse { cards: Vec<P1Card> },
    ThreeOfAKind { cards: Vec<P1Card> },
    TwoPair { cards: Vec<P1Card> },
    OnePair { cards: Vec<P1Card> },
    HighCard { cards: Vec<P1Card> },
}

impl PartialOrd for P1Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for P1Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (P1Hand::FiveOfAKind { cards: c0 }, P1Hand::FiveOfAKind { cards: c1 })
            | (P1Hand::FourOfAKind { cards: c0 }, P1Hand::FourOfAKind { cards: c1 })
            | (P1Hand::FullHouse { cards: c0 }, P1Hand::FullHouse { cards: c1 })
            | (P1Hand::ThreeOfAKind { cards: c0 }, P1Hand::ThreeOfAKind { cards: c1 })
            | (P1Hand::TwoPair { cards: c0 }, P1Hand::TwoPair { cards: c1 })
            | (P1Hand::OnePair { cards: c0 }, P1Hand::OnePair { cards: c1 })
            | (P1Hand::HighCard { cards: c0 }, P1Hand::HighCard { cards: c1 }) => {
                compare_p1_hands(c0, c1)
            }
            (P1Hand::FiveOfAKind { .. }, _) => Ordering::Greater,
            (_, P1Hand::FiveOfAKind { .. }) => Ordering::Less,
            (P1Hand::FourOfAKind { .. }, _) => Ordering::Greater,
            (_, P1Hand::FourOfAKind { .. }) => Ordering::Less,
            (P1Hand::FullHouse { .. }, _) => Ordering::Greater,
            (_, P1Hand::FullHouse { .. }) => Ordering::Less,
            (P1Hand::ThreeOfAKind { .. }, _) => Ordering::Greater,
            (_, P1Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (P1Hand::TwoPair { .. }, _) => Ordering::Greater,
            (_, P1Hand::TwoPair { .. }) => Ordering::Less,
            (P1Hand::OnePair { .. }, _) => Ordering::Greater,
            (_, P1Hand::OnePair { .. }) => Ordering::Less,
        }
    }
}

impl P1Hand {
    fn new(cards: Vec<P1Card>, card_counts: HashMap<P1Card, u8>) -> P1Hand {
        let mut card_counts: Vec<_> = card_counts.into_iter().collect();
        card_counts.sort_by(|(_, c0), (_, c1)| c0.cmp(c1));
        card_counts.reverse();

        let mut iter = card_counts.into_iter();
        let (_, count) = iter.next().unwrap();

        match count {
            5 => P1Hand::FiveOfAKind { cards },
            4 => P1Hand::FourOfAKind { cards },
            2..=3 => {
                let (_, next_count) = iter.next().unwrap();
                match next_count {
                    2 => match count {
                        3 => P1Hand::FullHouse { cards },
                        2 => P1Hand::TwoPair { cards },
                        _ => unreachable!("Should not happen!"),
                    },
                    1 => match count {
                        3 => P1Hand::ThreeOfAKind { cards },
                        2 => P1Hand::OnePair { cards },
                        _ => unreachable!("Should not happen!"),
                    },
                    _ => unreachable!("Should not happen!"),
                }
            }
            1 => P1Hand::HighCard { cards },
            _ => unreachable!("Should not happen!"),
        }
    }
}

fn compare_p1_hands(first: &Vec<P1Card>, second: &Vec<P1Card>) -> Ordering {
    first
        .iter()
        .zip(second.iter())
        .find_map(|(f, s)| match f.cmp(s) {
            Ordering::Equal => None,
            order => Some(order),
        })
        .unwrap_or(Ordering::Equal)
}

#[derive(Debug, Eq, PartialEq)]
enum P2Hand {
    FiveOfAKind { cards: Vec<P2Card> },
    FourOfAKind { cards: Vec<P2Card> },
    FullHouse { cards: Vec<P2Card> },
    ThreeOfAKind { cards: Vec<P2Card> },
    TwoPair { cards: Vec<P2Card> },
    OnePair { cards: Vec<P2Card> },
    HighCard { cards: Vec<P2Card> },
}

impl PartialOrd for P2Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for P2Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (P2Hand::FiveOfAKind { cards: c0 }, P2Hand::FiveOfAKind { cards: c1 })
            | (P2Hand::FourOfAKind { cards: c0 }, P2Hand::FourOfAKind { cards: c1 })
            | (P2Hand::FullHouse { cards: c0 }, P2Hand::FullHouse { cards: c1 })
            | (P2Hand::ThreeOfAKind { cards: c0 }, P2Hand::ThreeOfAKind { cards: c1 })
            | (P2Hand::TwoPair { cards: c0 }, P2Hand::TwoPair { cards: c1 })
            | (P2Hand::OnePair { cards: c0 }, P2Hand::OnePair { cards: c1 })
            | (P2Hand::HighCard { cards: c0 }, P2Hand::HighCard { cards: c1 }) => {
                compare_p2_hands(c0, c1)
            }
            (P2Hand::FiveOfAKind { .. }, _) => Ordering::Greater,
            (_, P2Hand::FiveOfAKind { .. }) => Ordering::Less,
            (P2Hand::FourOfAKind { .. }, _) => Ordering::Greater,
            (_, P2Hand::FourOfAKind { .. }) => Ordering::Less,
            (P2Hand::FullHouse { .. }, _) => Ordering::Greater,
            (_, P2Hand::FullHouse { .. }) => Ordering::Less,
            (P2Hand::ThreeOfAKind { .. }, _) => Ordering::Greater,
            (_, P2Hand::ThreeOfAKind { .. }) => Ordering::Less,
            (P2Hand::TwoPair { .. }, _) => Ordering::Greater,
            (_, P2Hand::TwoPair { .. }) => Ordering::Less,
            (P2Hand::OnePair { .. }, _) => Ordering::Greater,
            (_, P2Hand::OnePair { .. }) => Ordering::Less,
        }
    }
}

impl P2Hand {
    fn new(cards: Vec<P2Card>, card_counts: HashMap<P2Card, u8>, num_jacks: u8) -> P2Hand {
        let mut card_counts: Vec<_> = card_counts.into_iter().collect();
        card_counts.sort_by(|(_, c0), (_, c1)| c0.cmp(c1));
        card_counts.reverse();

        let mut iter = card_counts.into_iter();
        let (top_card, count) = iter.next().unwrap();

        match count {
            5 => P2Hand::FiveOfAKind { cards },
            4 => match (top_card, num_jacks) {
                (P2Card::Jack, 4) | (_, 1) => P2Hand::FiveOfAKind { cards },
                (_, 0) => P2Hand::FourOfAKind { cards },
                _ => unreachable!("Should not happen!"),
            },
            3 => match (top_card, num_jacks) {
                (P2Card::Jack, 3) => {
                    let (_, next_count) = iter.next().unwrap();
                    match next_count {
                        2 => P2Hand::FiveOfAKind { cards },
                        1 => P2Hand::FourOfAKind { cards },
                        _ => unreachable!("Should not happen!"),
                    }
                }
                (_, 2) => P2Hand::FiveOfAKind { cards },
                (_, 1) => P2Hand::FourOfAKind { cards },
                (_, 0) => {
                    let (_, next_count) = iter.next().unwrap();
                    match next_count {
                        2 => P2Hand::FullHouse { cards },
                        1 => P2Hand::ThreeOfAKind { cards },
                        _ => unreachable!("Should not happen!"),
                    }
                }
                _ => unreachable!("Should not happen!"),
            },
            2 => match (top_card, num_jacks) {
                (P2Card::Jack, 2) => {
                    let (_, next_count) = iter.next().unwrap();
                    match next_count {
                        2 => P2Hand::FourOfAKind { cards },
                        1 => P2Hand::ThreeOfAKind { cards },
                        _ => unreachable!("Should not happen!"),
                    }
                }
                (_, 2) => P2Hand::FourOfAKind { cards },
                (_, 1) => {
                    let (_, next_count) = iter.next().unwrap();
                    match next_count {
                        2 => P2Hand::FullHouse { cards },
                        1 => P2Hand::ThreeOfAKind { cards },
                        _ => unreachable!("Should not happen!"),
                    }
                }
                (_, 0) => {
                    let (_, next_count) = iter.next().unwrap();
                    match next_count {
                        2 => P2Hand::TwoPair { cards },
                        1 => P2Hand::OnePair { cards },
                        _ => unreachable!("Should not happen!"),
                    }
                }
                _ => unreachable!("Should not happen!"),
            },
            1 => match num_jacks {
                1 => P2Hand::OnePair { cards },
                0 => P2Hand::HighCard { cards },
                _ => unreachable!("Should not happen!"),
            },
            _ => unreachable!("Should not happen!"),
        }
    }
}

fn compare_p2_hands(first: &Vec<P2Card>, second: &Vec<P2Card>) -> Ordering {
    first
        .iter()
        .zip(second.iter())
        .find_map(|(f, s)| match f.cmp(s) {
            Ordering::Equal => None,
            order => Some(order),
        })
        .unwrap_or(Ordering::Equal)
}

fn part1(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let mut lines = binding.trim().lines();
    let mut hands_and_bids = Vec::new();

    while let Some(line) = lines.next() {
        let (cards_str, bid_str) = line.split_at(5);
        let bid = bid_str.trim_start().parse::<u32>().unwrap();

        let mut cards = Vec::new();
        let mut card_map = HashMap::new();
        for c in cards_str.chars() {
            let card = P1Card::new(c);
            let count = card_map.get(&card).unwrap_or(&0) + 1;

            cards.push(card.clone());
            card_map.insert(card, count);
        }

        let hand = P1Hand::new(cards, card_map);
        hands_and_bids.push((hand, bid));
    }

    hands_and_bids.sort_by(|(h0, _), (h1, _)| h0.cmp(h1));
    let result = hands_and_bids
        .into_iter()
        .enumerate()
        .fold(0, |acc, (pos, (_, bid))| ((pos as u32) + 1) * bid + acc);

    println!("{result}");
    Ok(())
}

fn part2(full_path: PathBuf) -> std::io::Result<()> {
    let binding = read_to_string(full_path)?;
    let mut lines = binding.trim().lines();
    let mut hands_and_bids = Vec::new();

    while let Some(line) = lines.next() {
        let (cards_str, bid_str) = line.split_at(5);
        let bid = bid_str.trim_start().parse::<u32>().unwrap();

        let mut cards = Vec::new();
        let mut card_map = HashMap::new();
        let mut num_jacks = 0;
        for c in cards_str.chars() {
            let card = P2Card::new(c);
            let count = card_map.get(&card).unwrap_or(&0) + 1;

            if card == P2Card::Jack {
                num_jacks += 1;
            }

            cards.push(card.clone());
            card_map.insert(card, count);
        }

        let hand = P2Hand::new(cards, card_map, num_jacks);
        hands_and_bids.push((hand, bid));
    }

    hands_and_bids.sort_by(|(h0, _), (h1, _)| h0.cmp(h1));
    let result = hands_and_bids
        .into_iter()
        .enumerate()
        .fold(0, |acc, (pos, (_, bid))| ((pos as u32) + 1) * bid + acc);

    println!("{result}");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args = App::parse();
    let current_dir = current_dir()?;

    match args.parts {
        Parts::P1 { path } => {
            let full_path = current_dir.join(path);
            part1(full_path)?;
        }
        Parts::P2 { path } => {
            let full_path = current_dir.join(path);
            part2(full_path)?;
        }
    }

    Ok(())
}
