use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let depths: Vec<i32> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().parse::<i32>().unwrap())
        .collect();
    println!("Solution 1 is {}", sol1(&depths));
    println!("Solution 2 is {}", sol2(&depths))
}

fn sol1(depths: &[i32]) -> i32 {
    sol_w_window(depths, 1)
}
fn sol2(depths: &[i32]) -> i32 {
    sol_w_window(depths, 3)
}
fn sol_w_window(depths: &[i32], window_size: usize) -> i32 {
    let mut inc_count = 0;
    let mut last_num: i32 = depths[0..window_size].iter().sum();

    for n in 1..depths.len() - window_size + 1 {
        let current_num = depths[n..(n + window_size)].iter().sum();
        if current_num > last_num {
            inc_count += 1;
        }
        last_num = current_num;
    }
    inc_count
}
