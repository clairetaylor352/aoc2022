use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[0].split('/').last().unwrap();
    let test = if args.get(1).is_none() || !args.get(1).unwrap().eq("test") {
        ""
    } else {
        "_test"
    };
    let file = File::open(format!("input/{}{}.txt", day, test)).unwrap();
    let commands: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();

    println!("Part 1 result is {}", sol1(&commands));
    println!("Part 2 result is {}", sol2(&commands));
}
fn parse_coordinates(coords: &str) -> (i32, i32) {
    let (x, y) = coords.split_once(",").unwrap();

    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
}
fn process_commands(commands: &[String], include_diagonals: bool) -> usize {
    let coordinates = commands
        .iter()
        .map(|x| {
            let (a, b) = x.split_once("->").unwrap();
            (parse_coordinates(a.trim()), parse_coordinates(b.trim()))
        })
        .collect::<Vec<_>>();
    println!("Coordinates {:?}", coordinates);
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    for ((x1, y1), (x2, y2)) in coordinates {
        if y1 == y2 {
            for x in x1..=x2 {
                let count = points.entry((x, y1)).or_insert(0);
                *count += 1;
            }
            for x in x2..=x1 {
                let count = points.entry((x, y1)).or_insert(0);
                *count += 1;
            }
        }
        if x1 == x2 {
            for y in y1..=y2 {
                let count = points.entry((x1, y)).or_insert(0);
                *count += 1;
            }
            for y in y2..=y1 {
                let count = points.entry((x1, y)).or_insert(0);
                *count += 1;
            }
        }

        if include_diagonals && (x1 - x2).abs() == (y1 - y2).abs() {
            let top = if y1 > y2 { (x1, y1) } else { (x2, y2) };
            let bottom = if y1 > y2 { (x2, y2) } else { (x1, y1) };
            if top.0 > bottom.0 {
                let mut idx = 0;
                for x in bottom.0..=top.0 {
                    let count = points.entry((x, bottom.1 + idx)).or_insert(0);
                    *count += 1;
                    idx += 1;
                }
            }
            if top.0 < bottom.0 {
                let mut idx = 0;
                for x in top.0..=bottom.0 {
                    let count = points.entry((x, top.1 - idx)).or_insert(0);
                    *count += 1;
                    idx += 1;
                }
            }
        }
    }
    println!("Points are {:?}", points);
    return points.values().into_iter().filter(|x| **x > 1).count();
}
fn sol1(commands: &[String]) -> usize {
    process_commands(commands, false)
}

fn sol2(commands: &[String]) -> usize {
    process_commands(commands, true)
}
