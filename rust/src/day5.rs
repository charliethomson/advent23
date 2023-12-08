use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;
use chrono::Utc;

#[derive(Debug)]
struct SeedMap {
    src: usize,
    dst: usize,
    sz: usize,
}

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    maps: HashMap<String /* src */, (String, Vec<SeedMap>)>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let seeds_raw = parts.next().unwrap();
    let seeds = seeds_raw
        .split(":")
        .nth(1)
        .map(|s| {
            s.split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .unwrap();
    let mut maps: HashMap<String /* src */, (String, Vec<SeedMap>)> = HashMap::new();

    for part in parts {
        let mut lines = part.lines();
        let top = lines.next().unwrap();
        let src = top.split("-to-").next().unwrap().to_string();
        let dst = top
            .split("-to-")
            .nth(1)
            .unwrap()
            .split(" map:")
            .next()
            .unwrap()
            .to_string();

        let rows = lines
            .map(|line| {
                line.split(" ")
                    .filter_map(|s| {
                        if s.trim().is_empty() {
                            None
                        } else {
                            Some(s.trim().parse::<usize>().unwrap())
                        }
                    })
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let mut entry = maps.entry(src).or_insert((dst, Vec::new()));

        for row in rows {
            let dst_start = *row.get(0).unwrap();
            let src_start = *row.get(1).unwrap();
            let sz = *row.get(2).unwrap();
            entry.1.push(SeedMap {
                dst: dst_start,
                src: src_start,
                sz,
            })
        }
    }

    return Input { maps, seeds };
}

fn process_seed(seed: usize, maps: &Vec<SeedMap>) -> usize {
    maps.iter()
        .find_map(|map| {
            if seed >= map.src && seed < map.src + map.sz {
                Some(map.dst + (seed - map.src))
            } else {
                None
            }
        })
        .unwrap_or(seed)
}

#[aoc(day5, part1)]
pub fn d5_p1(input: &Input) -> usize {
    // println!("{:#?}", input);

    // println!("Initial seeds: {}", input.seeds.len());
    let mut seeds = input.seeds.clone(); //.into_iter().chunks(2).into_iter().flat_map(|mut range| {
    //     let start = range.next().unwrap();
    //     let sz = range.next().unwrap();
    //     (0..sz).map(move |i| i + start)
    // }).collect::<Vec<usize>>();
    // println!("Expanded seeds: {}", seeds.len());

    let mut stage = "seed".to_string();
    loop {
        let Some((next, maps)) = input.maps.get(&stage) else {
            break;
        };

        for seed in &mut seeds {
            *seed = process_seed(*seed, maps);
        }

        stage = next.clone();
    }

    seeds.into_iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn d5_p2(input: &Input) -> usize {
    let seeds_start = Instant::now();

    println!("[GetSeeds][Start]");
    let mut seeds = input
        .seeds
        .clone()
        .into_iter()
        .chunks(2)
        .into_iter()
        .flat_map(|mut range| {
            let start = range.next().unwrap();
            let sz = range.next().unwrap();
            (0..sz).map(move |i| i + start)
        })
        .collect::<Vec<usize>>();
    println!("[GetSeeds][End][Runtime='{}ms']", seeds_start.elapsed().as_millis());


    let start = Instant::now();
    let mut stage = "seed".to_string();
    while let Some((next, maps)) = input.maps.get(&stage) {
        let stage_start = Instant::now();
        println!("[Stage][Name='{}'][Start]", stage);

        seeds.iter_mut().for_each(|seed| {
            *seed =
                maps.iter()
                    .find_map(|map| {
                        if *seed >= map.src && *seed < map.src + map.sz {
                            Some(map.dst + (*seed - map.src))
                        } else {
                            None
                        }
                    })
                    .unwrap_or(*seed);
        });

        println!("[Stage][Name='{}'][End][Runtime='{}ms']", stage, stage_start.elapsed().as_millis());
        stage = next.clone();
    }
    println!("[Process][End][Runtime={}ms]", start.elapsed().as_millis());


    seeds.into_iter().min().unwrap()
}
