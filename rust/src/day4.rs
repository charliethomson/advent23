use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use num::Zero;

#[derive(Debug)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}
type Input = Vec<Card>;

//Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let id = line
                .split("Card ")
                .nth(1)
                .and_then(|l| l.split(":").nth(0))
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();
            let winning_numbers = line
                .split(":")
                .nth(1)
                .and_then(|l| l.split("|").nth(0))
                .unwrap()
                .split(" ")
                .filter_map(|n| {
                    if n.trim().is_empty() {
                        None
                    } else {
                        Some(n.trim())
                    }
                })
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();
            let numbers = line
                .split("|")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter_map(|n| {
                    if n.trim().is_empty() {
                        None
                    } else {
                        Some(n.trim())
                    }
                })
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();

            Card {
                id,
                winning_numbers,
                numbers,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn d4_p1(input: &Input) -> usize {
    let mut total = 0;

    for card in input {
        let win_count = card
            .numbers
            .iter()
            .filter(|num| card.winning_numbers.contains(num))
            .count();

        let mut value = if win_count.is_zero() { 0 } else { 1 };
        for _ in 1..win_count {
            value *= 2;
        }
        total += value;
    }

    total
}

#[aoc(day4, part2)]
pub fn d4_p2(input: &Input) -> usize {
    let mut scales = HashMap::<usize, usize>::new();

    for card in input {
        let scale = *scales.entry(card.id).or_insert(1);
        let win_count = card
            .numbers
            .iter()
            .filter(|num| card.winning_numbers.contains(num))
            .count();

        for i in card.id + 1..card.id + win_count + 1 {
            let scl = scales.get(&i).copied().unwrap_or(1);
            scales.insert(i, scl + scale);
        }
    }

    return scales.values().sum::<usize>();
}
