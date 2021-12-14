use std::collections::HashMap;

use aoc2021::files::get_input;
use itertools::{min, Itertools};

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);

    let template = commands[0].to_string();

    let rules = &commands[2..]
        .iter()
        .map(|x| x.split_once(" -> ").unwrap())
        .map(|(x, y)| {
            (
                (x.chars().next().unwrap(), x.chars().nth(1).unwrap()),
                y.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();

    println!("Template : {}", template);
    println!("Rules: {:?}", rules);

    println!(
        "Sol 1: {}",
        count_difference_after_n(template.clone(), rules, 10)
    );
    println!(
        "Sol 2: {}",
        count_difference_after_n(template.clone(), rules, 40)
    );
}

fn count_difference_after_n(
    template: String,
    rules: &HashMap<(char, char), char>,
    iterations: i32,
) -> usize {
    let mut pair_counts = HashMap::new();
    for (a, b) in template.chars().tuple_windows() {
        let new_count = pair_counts.get(&(a, b)).unwrap_or(&0) + 1;
        pair_counts.insert((a, b), new_count);
    }
    let last_char = template.chars().last().unwrap();

    for _ in 1..=iterations {
        let mut new_counts = HashMap::new();
        for ((a, b), count) in &pair_counts {
            let key = rules.get(&(*a, *b)).unwrap();
            let value1 = new_counts.get(&(*a, *key)).unwrap_or(&0) + count;

            new_counts.insert((*a, *key), value1);
            let value2 = new_counts.get(&(*key, *b)).unwrap_or(&0) + count;

            new_counts.insert((*key, *b), value2);
        }
        pair_counts = new_counts;
    }
    let mut final_count: HashMap<char, usize> = HashMap::new();
    for ((char1, _), count) in pair_counts {
        let new_count = final_count.get(&char1).unwrap_or(&0) + count;
        final_count.insert(char1, new_count);
    }
    let new_count = final_count.get(&last_char).unwrap_or(&0) + 1;
    final_count.insert(last_char, new_count);
    println!("Final Count {:?}", final_count);
    println!(
        "Final count {:?}",
        final_count.values().max().unwrap() - min(final_count.values()).unwrap()
    );
    final_count.values().max().unwrap() - min(final_count.values()).unwrap()
}
