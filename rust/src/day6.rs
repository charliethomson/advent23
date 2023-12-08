/*
Time:      7  15   30
Distance:  9  40  200
*/

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    dist: usize,
}

type Input = Vec<String>;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Input {
    input.lines().map(|s| s.to_string()).collect()
}

fn can_win_race(race: Race, speed: usize) -> bool {
    let dist = speed * (race.time - speed);
    return dist > race.dist;
}

#[aoc(day6, part1)]
pub fn d6_p1(input: &Input) -> usize {
    let input: Vec<Race> = {
        let mut map = input.iter().map(|line| line.split(":").nth(1).unwrap().trim().split(" ").filter_map(|s| if s.trim().is_empty() { None } else { Some(s.trim()) }).collect::<Vec<_>>());

        let times = map.next().unwrap().into_iter();
        let dists = map.next().unwrap().into_iter();

        times.zip(dists).map(|(time, dist)| Race { time: time.parse().unwrap(), dist: dist.parse().unwrap() }).collect()
    };
    // println!("{:#?}", input);
    input.iter().map(|race| (1..race.time).filter(|i| can_win_race(*race, *i)).count()).product()
    // for race in input {
    //     let winners = (1..race.time).filter(|i| can_win_race(*race, *i)).collect::<Vec<_>>();
    //     println!("{:?} => {:?}",*race, winners.len())
    // }
    // 0
}

#[aoc(day6, part2)]
pub fn d6_p2(input: &Input) -> usize {
    let input = {
        let mut map = input.iter().map(|line| line.replace(" ", "")).map(|line| line.split(":").nth(1).unwrap().to_string());

        let time = map.next().unwrap().parse::<usize>().unwrap();
        let dist = map.next().unwrap().parse::<usize>().unwrap();

        Race { time, dist }
    };

    (1..input.time).filter(|i| can_win_race(input, *i)).count()
}

