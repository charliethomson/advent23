use std::cmp::Ordering;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::multi::count;

#[derive(Clone, Copy, Debug)]
struct Card {
    sign: char,
    value: usize,
}

impl Card {
    pub fn from(value: char) -> Self {
        match value {
            'A' => Self { sign: 'A', value: 14 },
            'K' => Self { sign: 'K', value: 13 },
            'Q' => Self { sign: 'Q', value: 12 },
            'J' => Self { sign: 'J', value: 11 },
            'T' => Self { sign: 'T', value: 10 },
            '9' => Self { sign: '9', value: 9 },
            '8' => Self { sign: '8', value: 8 },
            '7' => Self { sign: '7', value: 7 },
            '6' => Self { sign: '6', value: 6 },
            '5' => Self { sign: '5', value: 5 },
            '4' => Self { sign: '4', value: 4 },
            '3' => Self { sign: '3', value: 3 },
            '2' => Self { sign: '2', value: 2 },
            _ => panic!("{}", value),
        }
    }
    pub fn from2(value: char) -> Self {
        match value {
            'A' => Self { sign: 'A', value: 14 },
            'K' => Self { sign: 'K', value: 13 },
            'Q' => Self { sign: 'Q', value: 12 },
            'J' => Self { sign: 'J', value: 1 },
            'T' => Self { sign: 'T', value: 10 },
            '9' => Self { sign: '9', value: 9 },
            '8' => Self { sign: '8', value: 8 },
            '7' => Self { sign: '7', value: 7 },
            '6' => Self { sign: '6', value: 6 },
            '5' => Self { sign: '5', value: 5 },
            '4' => Self { sign: '4', value: 4 },
            '3' => Self { sign: '3', value: 3 },
            '2' => Self { sign: '2', value: 2 },
            _ => panic!("{}", value),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    Explicit {
        counts: Vec<usize>,
        jokers: usize,
    },
}

impl HandKind {
    #[inline(always)]
    pub fn strength(&self) -> usize {
        match self {
            Self::FiveOfAKind => 7,
            Self::FourOfAKind => 6,
            Self::FullHouse => 5,
            Self::ThreeOfAKind => 4,
            Self::TwoPair => 3,
            Self::OnePair => 2,
            Self::HighCard => 1,
            Self::Explicit { counts, jokers } => {

                let mut counts = counts.iter().sorted().rev().copied().collect::<Vec<_>>();
                println!("{:?}", counts);

                match counts.get(0).copied().unwrap_or_default() + jokers {
                    2 => if counts.get(1).copied().unwrap_or_default() == 2 { 3 } else { 2 },
                    3 => if counts.get(1).copied().unwrap_or_default() == 2 { 5 } else { 4 },
                    4 => 6,
                    5 => 7,
                    e => e
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Hand {
    cards: [Card; 5],
    kind: HandKind,
}

impl Hand {
    pub fn new<S: ToString>(hand: S) -> Self {
        let hand = hand.to_string();
        let mut cards = hand.chars().map(Card::from).collect::<Vec<Card>>().try_into().unwrap();

        Self {
            kind: Self::categorize(&cards),
            cards,
        }
    }
    fn categorize(cards: &[Card; 5]) -> HandKind {
        // TODO(opt): maybe map count -> count of count if that makes sense
        let mut counts: HashMap<char, usize> = HashMap::new();
        for card in cards {
            let mut entry = counts.entry(card.sign).or_default();
            *entry += 1;
        }

        let count_twos = counts.values().filter(|c| **c == 2).count();
        let has_three = counts.values().any(|c| *c == 3);
        let has_four = counts.values().any(|c| *c == 4);
        let has_five = counts.values().any(|c| *c == 5);


        if count_twos == 1 {
            if has_three { HandKind::FullHouse } else { HandKind::OnePair }
        } else if count_twos == 2 {
            HandKind::TwoPair
        } else if has_three {
            HandKind::ThreeOfAKind
        } else if has_four {
            HandKind::FourOfAKind
        } else if has_five {
            HandKind::FiveOfAKind
        } else {
            HandKind::HighCard
        }
    }
    pub fn new2<S: ToString>(hand: S) -> Self {
        let hand = hand.to_string();
        let mut cards = hand.chars().map(Card::from2).collect::<Vec<Card>>().try_into().unwrap();

        Self {
            kind: Self::categorize2(&cards),
            cards,
        }
    }
    fn categorize2(cards: &[Card; 5]) -> HandKind {
        let mut counts: HashMap<char, usize> = HashMap::new();
        let mut jokers = 0;
        for card in cards {
            if card.sign == 'J' {
                jokers += 1;
            } else {
                *counts.entry(card.sign).or_default() += 1;
            }
        }

        HandKind::Explicit {
            counts: counts.into_values().collect(),
            jokers,
        }
    }
}

type Input = Vec<(Hand, usize)>;

#[aoc_generator(day7, part1)]
pub fn input_generator_p1(input: &str) -> Input {
    input.lines().map(|s| {
        let mut parts = s.split(" ");
        let hand_raw = parts.next().unwrap();
        let bid = parts.next().unwrap().parse().unwrap();

        (Hand::new(hand_raw), bid)
    }).collect()
}

#[aoc_generator(day7, part2)]
pub fn input_generator_p2(input: &str) -> Input {
    input.lines().map(|s| {
        let mut parts = s.split(" ");
        let hand_raw = parts.next().unwrap();
        let bid = parts.next().unwrap().parse().unwrap();

        (Hand::new2(hand_raw), bid)
    }).collect()
}

fn print_hands(hands: &Input) {
    for (index, (hand, bet)) in hands.iter().enumerate() {
        let handstr = hand.cards.iter().map(|card| card.sign).join("");
        println!("{:<04} - {handstr} [Kind={:?}][Bet={bet}]", index + 1, hand.kind)
    }
}

#[aoc(day7, part1)]
pub fn d7_p1(input: &Input) -> usize {
    // println!("{:#?}", input);

    let mut sorted = input.clone();
    sorted.sort_by(|(hand, _), (ohand, _)| {
        match hand.kind.strength().cmp(&ohand.kind.strength()) {
            Ordering::Equal => {
                hand.cards.iter().map(|c| c.value).cmp(ohand.cards.iter().map(|c| c.value))
            }
            ord => ord
        }
    });

    sorted.into_iter().enumerate().inspect(|(index, (hand, bet))| {
        // let cards = hand.cards.iter().map(|c| c.value).collect::<Vec<_>>();
        // println!("({index}, Hand {{ cards: {cards:?}, bid: {bet} }})")
        let handstr = hand.cards.iter().map(|card| card.sign).join("");
        println!("{:<04} - {handstr} [Kind={:?}][Bet={bet}]", index + 1, hand.kind);
    }).map(|(i, (_, bet))| bet * (i + 1)).sum()
}

#[aoc(day7, part2)]
pub fn d7_p2(input: &Input) -> usize {
    let mut sorted = input.clone();
    sorted.sort_by(|(hand, _), (ohand, _)| {
        match hand.kind.strength().cmp(&ohand.kind.strength()) {
            Ordering::Equal => {
                hand.cards.iter().map(|c| c.value).cmp(ohand.cards.iter().map(|c| c.value))
            }
            ord => ord
        }
    });

    // println!("{:#?}", sorted);


    sorted.into_iter().enumerate().inspect(|(index, (hand, bet))| {
        // let cards = hand.cards.iter().map(|c| c.value).collect::<Vec<_>>();
        // println!("({index}, Hand {{ cards: {cards:?}, bid: {bet} }})");
        let handstr = hand.cards.iter().map(|card| card.sign).join("");
        println!("{:<04} - {handstr} [Kind={:?}][Strength={}][Bet={bet}]", index, hand.kind, hand.kind.strength());
    }).map(|(i, (_, bet))| bet * (i + 1)).sum()
}

