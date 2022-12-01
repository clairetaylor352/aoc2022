use aoc2021::files::get_input;

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);
    let mut it1 = commands.iter();

    let mut calorie_last: i32 = it1.by_ref().take_while(|x| !x.is_empty()).map(|s| s.parse::<i32>().unwrap()).sum();
    let mut all_totals: Vec<i32> = Vec::new();
    while (calorie_last != 0) {
        calorie_last = it1.by_ref().take_while(|x| !x.is_empty()).map(|s| s.parse::<i32>().unwrap()).sum();
        all_totals.push(calorie_last);

    }
    all_totals.sort_by(|a, b| b.cmp(a));
    println!("top total {}", all_totals[0]);
    println!("All total {}", all_totals[0..3].iter().sum::<i32>());

}

fn part1() {

}
