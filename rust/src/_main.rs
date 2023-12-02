use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Game {
    id: usize,
    red: usize,
    green: usize,
    blue: usize,
}

fn main() {
    let mut buffer = String::new();
    <_ as Read>::read_to_string(&mut File::open("./input/2023/day2.txt"), &mut buffer).unwrap();
    println!("{:#?}", buffer
        .lines()
        .map(|s| s.trim().to_string())
        .map(|s| {
            let id = s.split("Game ").nth(1).expect("1").split(":").next().expect("2");
            let hands = s.split(": ").nth(1).expect("3").split(";").map(|p| p.split(", ").map(|s| s.split(" ")).map(|mut s| (s.nth(0).unwrap(), s.nth(1).unwrap())).collect::<Vec<_>>()).collect::<Vec<_>>();
            // let parts = s.split(": ").nth(1).expect("3").split(", ").map(|p| p.split(" ").collect::<Vec<_>>()).map(|i| (i[0], i[1])).collect::<Vec<_>>();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for hand in hands {
                let mut hred = 0;
                let mut hgreen = 0;
                let mut hblue = 0;

                for part in hand {
                    let ct = part.1.parse::<usize>().expect("parseusize");

                    match part.1 {
                        "red" => red = red.max(ct),
                        "green" => green = green.max(ct),
                        "blue" => blue = blue.max(ct),
                        _ => panic!("{}", part.1),
                    }
                }


                red = red.max(hred);
                green = green.max(hgreen);
                blue = blue.max(hblue);
            }

            let game = Game { id: id.parse().expect("parseid"), red, green, blue };
            return game;
        })
        .collect());
}