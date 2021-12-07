use crate::files::get_input;
use std::collections::HashMap;
mod files;

fn main() {
    let commands: Vec<String> = get_input();

    println!("Solution 1 is: {} ", fish_after_days(&commands[0], 80));
    println!("Solution 2 is: {} ", fish_after_days(&commands[0], 256));
}
fn fish_after_days(command: &String, days: i32) -> i64 {
    let fish: Vec<i32> = command
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut fish_count: HashMap<i32, i64> = HashMap::new();
    for f in fish {
        let f_n = fish_count.entry(f).or_insert(0);
        *f_n += 1;
    }
    println!("Starting fish {:?}", fish_count);
    for i in 1..=days {
        let f_zero = *fish_count.get(&0).unwrap_or(&0_i64);
        for day in 1..=8 {
            *fish_count.entry(day - 1).or_default() = *fish_count.get(&day).unwrap_or(&0_i64);
        }
        *fish_count.entry(8).or_default() = f_zero;
        *fish_count.entry(6).or_default() += f_zero;
    }
    fish_count.into_values().sum::<i64>()
}
