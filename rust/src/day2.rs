use std::collections::HashSet;
use aoc_runner_derive::{aoc, aoc_generator};
use fancy_regex::Regex;

#[derive(Debug)]
struct Game {
    id: usize,
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

type Input = Vec<Game>;


#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Input {
    input.lines()
        .map(|s| s.trim().to_string())
        .map(|s| {
            let id = s.split("Game ").nth(1).expect("1").split(":").next().expect("2");
            let parts = s.split(": ").nth(1).expect("3").split(";").map(|p| p.split(", ").map(|s| s.split(" ").filter(|f| !f.is_empty()).collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();
            // let parts = s.split(": ").nth(1).expect("3").split(", ").map(|p| p.split(" ").collect::<Vec<_>>()).map(|i| (i[0], i[1])).collect::<Vec<_>>();

            let mut hands = Vec::new();

            for hand in parts {
                let mut hred = 0;
                let mut hgreen = 0;
                let mut hblue = 0;

                for part in hand {
                    let ct = part.iter().nth(0).unwrap().parse::<usize>().expect("parseusize");

                    match *part.iter().nth(1).unwrap() {
                        "red" => hred += ct,
                        "green" => hgreen += ct,
                        "blue" => hblue += ct,
                        a => panic!("{}", a),
                    }
                }

                hands.push(Hand {
                    blue: hblue,
                    red: hred,
                    green: hgreen,
                });
                // println!("Hand: {hred} red, {hgreen} green, {hblue} blue");
            }

            let game = Game { id: id.parse().expect("parseid"), hands };
            return game;
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn day2(lines: &Input) -> usize {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    return lines.iter().filter_map(|game| {
        let (
            red,
            green,
            blue,
        ) = game.hands.iter().fold((0, 0, 0), |(gr, gg, gb), hand|
            (gr.max(hand.red)
             , gg.max(hand.green)
             , gb.max(hand.blue)),
        );

        if red > max_red { return None; }
        if green > max_green { return None; }
        if blue > max_blue { return None; }

        return Some(game.id);
    }).sum::<usize>();
}

#[aoc(day2, part2)]
pub fn day2_p2(lines: &Input) -> usize {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    return lines.iter().map(|game| {
        let (
            mut red,
            mut green,
            mut blue,
        ) = (0, 0, 0);

        for hand in &game.hands {
            red = red.max(hand.red);
            green = green.max(hand.green);
            blue = blue.max(hand.blue);
        }

        return red * green * blue;
    }).sum::<usize>();
}
