use aoc2021::files::get_input;
use counter::Counter;
use itertools::Itertools;
fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);

    let coords = commands
        .iter()
        .filter_map(|coord| coord.split_once(','))
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .collect_vec();

    let folds = commands
        .iter()
        .filter_map(|fold| fold.split_once('='))
        .map(|(x, y)| (x.chars().last().unwrap(), y.parse::<i32>().unwrap()))
        .collect_vec();
    println!("coords {:?}", coords.len());
    println!("folds {:?}", folds);
    let mut new_coords = coords;
    for first_fold in folds {
        new_coords = new_coords
            .iter()
            .map(|(x, y)| {
                if first_fold.0 == 'x' {
                    if *x > first_fold.1 {
                        (first_fold.1 - (x - first_fold.1), *y)
                    } else {
                        (*x, *y)
                    }
                } else {
                    if *y > first_fold.1 {
                        (*x, first_fold.1 - (y - first_fold.1))
                    } else {
                        (*x, *y)
                    }
                }
            })
            .collect_vec();
        let coord_counts = new_coords.iter().collect::<Counter<_>>();
        println!("{}", coord_counts.len());
    }
    let max_x = new_coords.iter().map(|(x, _)| x).max().unwrap();
    let max_y = new_coords.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=*max_y {
        let mut line: String = "".to_string();
        for x in 0..=*max_x {
            if new_coords.contains(&(x, y)) {
                line.push_str("*");
            } else {
                line.push_str(" ");
            }
        }
        println!("{}", line);
    }
}
