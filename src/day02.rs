use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input/day02.txt").unwrap();
    let commands: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();

    println!("Part 1 result is {}", sol1(&commands));
    println!("Part 2 result is {}", sol2(&commands));
}

fn sol1(commands: &[String]) -> i64 {
    let mut x = 0i64;
    let mut y = 0i64;
    for command in commands {
        let amount = command.split_once(" ").unwrap().1.parse::<i64>().unwrap();
        match &command[..1] {
            "f" => {
                x += amount;
            }
            "u" => {
                y -= amount;
            }
            "d" => {
                y += amount;
            }
            _ => {}
        }
    }
    x * y
}
fn sol2(commands: &[String]) -> i64 {
    let mut x = 0i64;
    let mut y = 0i64;
    let mut aim = 0i64;
    for command in commands {
        let amount = command.split_once(" ").unwrap().1.parse::<i64>().unwrap();
        match &command[..1] {
            "f" => {
                y += amount * aim;
                x += amount;
            }
            "u" => {
                aim -= amount;
            }
            "d" => {
                aim += amount;
            }
            _ => {}
        }
    }
    x * y
}
