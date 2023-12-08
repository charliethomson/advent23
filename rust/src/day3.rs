use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::net;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Number { value: usize, span: (usize, usize) },
    Symbol { value: char, span: usize },
}

impl Value {
    fn number(&self) -> usize {
        match self {
            Value::Number { value, .. } => *value,
            _ => panic!(),
        }
    }
    fn number_span(&self) -> (usize, usize) {
        match self {
            Value::Number { span, .. } => *span,
            _ => panic!(),
        }
    }
    fn symbol(&self) -> char {
        match self {
            Value::Symbol { value, .. } => *value,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    cols: usize,
    rows: usize,
    values: Vec<Value>,
}

fn get_neighbors(center: usize, cols: usize, rows: usize) -> Vec<usize> {
    let (left, right) = match center % cols {
        0 => (-1, 0),
        1 => (0, 1),
        _ => (-1, 1),
    };

    let (top, bottom) = match center {
        c if c > (cols * (rows - 1)) => (-(cols as i32), 0),
        c if c < cols => (0, cols as i32),
        _ => (-(cols as i32), (cols as i32)),
    };

    // let mut neighbors = HashSet::new();
    // neighbors.insert(((center as i32) + top + left) as usize);
    // neighbors.insert(((center as i32) + top) as usize);
    // neighbors.insert(((center as i32) + top + right) as usize);
    // neighbors.insert(((center as i32) + left) as usize);
    // //
    // neighbors.insert(((center as i32) + right) as usize);
    // neighbors.insert(((center as i32) + bottom + left) as usize);
    // neighbors.insert(((center as i32) + bottom) as usize);
    // neighbors.insert(((center as i32) + bottom + right) as usize);
    let mut neighbors = vec![
        ((center as i32) + top + left) as usize,
        ((center as i32) + top) as usize,
        ((center as i32) + top + right) as usize,
        ((center as i32) + left) as usize,
        //
        ((center as i32) + right) as usize,
        ((center as i32) + bottom + left) as usize,
        ((center as i32) + bottom) as usize,
        ((center as i32) + bottom + right) as usize,
    ];
    neighbors.sort();
    neighbors.dedup();
    neighbors.retain(|e| *e != center);
    return neighbors;
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    let mut buffer = String::new();
    let mut buffer_start = 0;
    let mut values = Vec::new();

    let mut i = 0;
    for ch in input.trim().chars() {
        if ch == '\n' {
            if !buffer.is_empty() && buffer.chars().all(|c| c.is_digit(10)) {
                let value = buffer.parse::<usize>().expect("invalid int");
                values.push(Value::Number {
                    value,
                    span: (buffer_start, buffer_start + buffer.len()),
                });
            }
            buffer.clear();
            continue;
        } else {
            i += 1;
        }

        if ch.is_digit(10) {
            if buffer.is_empty() {
                buffer_start = i;
            }
            buffer.push(ch);
            continue;
        }
        if !buffer.is_empty() && buffer.chars().all(|c| c.is_digit(10)) {
            let value = buffer.parse::<usize>().expect("invalid int");
            values.push(Value::Number {
                value,
                span: (buffer_start, buffer_start + buffer.len()),
            });
        }
        buffer.clear();

        if ch == '.' {
            continue;
        } else {
            values.push(Value::Symbol { value: ch, span: i });
        }
    }

    return Input {
        cols: input.split("\n").nth(0).unwrap().len(),
        rows: input.chars().filter(|c| *c == '\n').count() + 1,
        values,
    };
}

// TODO: Perf :/

#[aoc(day3, part1)]
pub fn d3_p1(input: &Input) -> usize {
    let mut total = 0;

    let (symbols, mut numbers): (Vec<Value>, Vec<Value>) = input
        .values
        .clone()
        .into_iter()
        .partition(|value| matches!(value, Value::Symbol { .. }));

    let mut removed_nums = vec![];

    for symbol in symbols.clone() {
        let Value::Symbol { span, .. } = symbol else {
            continue;
        };

        let neighbors = get_neighbors(span, input.cols, input.rows);

        let mut removed: Vec<Value> = vec![];
        let mut nums: Vec<Value> = vec![];
        'outer: for num in numbers {
            let (min, max) = num.number_span();
            for i in min..max {
                if (neighbors.contains(&i)) {
                    removed.push(num);
                    continue 'outer;
                }
            }
            nums.push(num);
        }

        for v in removed {
            total += v.number();
            removed_nums.push(v);
        }

        numbers = nums;
    }

    total
}

#[aoc(day3, part2)]
pub fn d3_p2(input: &Input) -> usize {
    let mut total = 0;

    let (symbols, numbers): (Vec<Value>, Vec<Value>) =
        input.values.clone().into_iter().partition(|value| {
            if matches!(value, Value::Symbol { .. }) {
                true
            } else {
                false
            }
        });

    for gear in symbols.clone().into_iter().filter(|s| s.symbol() == '*') {
        let Value::Symbol { span, .. } = gear else {
            continue;
        };

        let neighbors = get_neighbors(span, input.cols, input.rows);

        let removed = numbers
            .iter()
            .filter(|num| {
                let (min, max) = num.number_span();
                return neighbors
                    .iter()
                    .any(|neighbor| *neighbor < max && *neighbor >= min);
            })
            .collect::<Vec<_>>();

        if removed.len() != 2 {
            continue;
        }

        let a = removed.get(0).unwrap();
        let b = removed.get(1).unwrap();

        total += a.number() * b.number();
    }

    // for symbol in symbols.clone() {
    //     let Value::Symbol { value, span } = symbol else {
    //         continue;
    //     };

    //     let neighbors = get_neighbors(span, input.cols, input.rows);

    //     let (removed, nums): (Vec<Value>, Vec<Value>) = numbers.iter().partition(|num| {
    //         let (min, max) = num.number_span();
    //         return neighbors
    //             .iter()
    //             .any(|neighbor| *neighbor < max && *neighbor >= min);
    //     });

    //     for v in removed {
    //         total += v.number();
    //         removed_nums.push(v);
    //     }

    //     numbers = nums;
    // }

    total
}
