use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use nom::Slice;
use std::collections::HashMap;
use std::ops::Index;

type Input = Vec<String>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

// #[aoc(day1, part1)]
pub fn d1_p1(lines: &Input) -> usize {
    return lines
        .iter()
        .map(|line| {
            (
                line.chars()
                    .nth(
                        line.find(|c| char::is_digit(c, 10))
                            .expect(&format!("1 - '{line}'")),
                    )
                    .expect("2"),
                line.chars()
                    .nth(line.rfind(|c| char::is_digit(c, 10)).expect("3"))
                    .expect("4"),
            )
        })
        .map(|(a, b)| str::parse::<usize>(&format!("{a}{b}")).expect("5"))
        .sum::<usize>();
}

#[derive(Debug)]
pub struct Scanner {
    buffer: String,
    candidates: Vec<String>,
    i: usize,
}

const NUMBERS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
    static ref INIT_NUMS: [Option<Vec<String>>; 255] = {
        let mut map: [Option<Vec<String>>; 255] = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None,
        ];

        map['o' as usize] = Some(vec!["ne".to_string()]);
        map['t' as usize] = Some(vec!["wo".to_string(), "hree".to_string()]);
        map['f' as usize] = Some(vec!["our".to_string(), "ive".to_string()]);
        map['s' as usize] = Some(vec!["ix".to_string(), "even".to_string()]);
        map['e' as usize] = Some(vec!["ight".to_string()]);
        map['n' as usize] = Some(vec!["ine".to_string()]);

        map
    };
}

fn filter_candidates(candidates: &Vec<String>, ch: char, i: usize) -> Vec<String> {
    candidates
        .iter()
        .filter_map(|number| {
            if number.as_bytes()[i] != ch as u8 {
                return None;
            }

            return Some(number.clone());
        })
        .collect()
}

impl Scanner {
    pub fn new(ch: char) -> Option<Self> {
        let buffer = ch.to_string();
        Some(Self {
            buffer,
            candidates: INIT_NUMS[ch as usize].clone()?,
            i: 0,
        })
    }

    pub fn scan(&mut self, ch: char) -> Option<u32> {
        self.buffer.push(ch);
        self.i += 1;

        let (found, candidates) = filter_candidates(&self.candidates, ch, self.i)
            .into_iter()
            .fold((false, vec![]), |(b, mut acc), cur| {
                if cur.len() == self.i {
                    (true, acc)
                } else {
                    acc.push(cur);
                    (b, acc)
                }
            });
        self.candidates = candidates;

        if found {
            Some(self.as_int())
        } else {
            None
        }
    }
    fn as_int(&self) -> u32 {
        match self.buffer.as_str() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!("Invalid buffer {}", self.buffer),
        }
    }
}

#[aoc(day1, part2)]
pub fn d1_p2(lines: &Input) -> u32 {
    let mut sum = 0;
    for line in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        let mut scanners: Vec<Scanner> = Vec::new();

        for ch in line.chars() {
            if let Some(dig) = ch.to_digit(10) {
                if first.is_none() {
                    first = Some(dig)
                } else {
                    last = Some(dig)
                }
            }

            for scanner in &mut scanners {
                let Some(result) = scanner.scan(ch) else {
                    continue;
                };
                if first.is_none() {
                    first = Some(result)
                } else {
                    last = Some(result)
                }
            }

            if let Some(new_scanner) = Scanner::new(ch) {
                scanners.push(new_scanner);
            }
            scanners = scanners
                .into_iter()
                .filter(|scanner| !scanner.candidates.is_empty())
                .collect();
        }

        assert!(first.is_some());

        last = last.or(first.clone());

        sum += first.unwrap() * 10 + last.unwrap();
    }

    sum
}

// #[aoc(day1, part2)]
pub fn d1_p2_old(lines: &Input) -> usize {
    fn find_numbers(line: &String) -> usize {
        fn parse(num: &str) -> usize {
            let n = num.to_lowercase();
            if n.ends_with("one") {
                return 1;
            }
            if n.ends_with("two") {
                return 2;
            }
            if n.ends_with("three") {
                return 3;
            }
            if n.ends_with("four") {
                return 4;
            }
            if n.ends_with("five") {
                return 5;
            }
            if n.ends_with("six") {
                return 6;
            }
            if n.ends_with("seven") {
                return 7;
            }
            if n.ends_with("eight") {
                return 8;
            }
            if n.ends_with("nine") {
                return 9;
            }
            if n.chars()
                .next()
                .map(|c| char::is_digit(c, 10))
                .unwrap_or_default()
            {
                return n.parse::<usize>().unwrap();
            }

            panic!("{n}");
        }
        let re = Regex::new(r"(?=(one))|(?=(two))|(?=(three))|(?=(four))|(?=(five))|(?=(six))|(?=(seven))|(?=(eight))|(?=(nine))|(?=(\d))").unwrap();
        let caps = re.captures_iter(line);
        let caps = caps
            .filter_map(|cap| cap.ok())
            .flat_map(|cap| {
                cap.iter()
                    .map(|c| c.map(|c2| c2.as_str().to_owned()))
                    .filter(|c| c.as_ref().map(|i| i.len() != 0).unwrap_or_default())
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();

        let start = parse(caps.first().expect("1").as_str());
        let end = parse(caps.last().expect("3").as_str());

        return start * 10 + end;
    }

    return lines.iter().map(find_numbers).sum::<usize>();
}
