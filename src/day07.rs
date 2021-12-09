use aoc2021::files::get_input;

fn main() {
    let commands: Vec<String> = get_input(false);

    println!("Solution 1 is: {} ", minimum_fuel(&commands[0], false));
    println!("Solution 2 is: {} ", minimum_fuel(&commands[0], true));
}
fn minimum_fuel(command: &String, expensive: bool) -> i32 {
    let crabs: Vec<i32> = command
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min_value = *crabs.iter().min().unwrap();
    let max_value = *crabs.iter().max().unwrap();
    let mut min = crabs
        .iter()
        .map(|x| calculate_individual_fuel((x - min_value).abs(), expensive))
        .sum::<i32>();
    for i in min_value + 1..=max_value {
        if crabs
            .iter()
            .map(|x| calculate_individual_fuel((x - i).abs(), expensive))
            .sum::<i32>()
            < min
        {
            min = crabs
                .iter()
                .map(|x| calculate_individual_fuel((x - i).abs(), expensive))
                .sum::<i32>();
        }
    }
    min
}

fn calculate_individual_fuel(distance: i32, expensive: bool) -> i32 {
    if expensive {
        return (distance * (distance + 1)) / 2;
    }
    distance
}

#[test]
fn test_day07() {
    let commands: Vec<String> = get_input(true);

    assert_eq!(minimum_fuel(&commands[0], false), 37);
    assert_eq!(minimum_fuel(&commands[0], true), 168);
}
