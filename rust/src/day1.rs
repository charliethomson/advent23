use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;

type Input = Vec<String>;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

#[aoc(day1, part1)]
pub fn day1(lines: &Input) -> usize {
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

#[aoc(day1, part2)]
pub fn day1_part2(lines: &Input) -> usize {
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
        println!("{line}: {start}, {end} => {}", start * 10 + end);
        
        return start * 10 + end;
    }

    return lines.iter().map(find_numbers).sum::<usize>();
}
