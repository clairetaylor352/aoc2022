use aoc2021::files::get_input;
use itertools::Itertools;
/* 0:      1:      2:      3:      4:
aaaa    ....    aaaa    aaaa    ....
b    c  .    c  .    c  .    c  b    c
b    c  .    c  .    c  .    c  b    c
....    ....    dddd    dddd    dddd
e    f  .    f  e    .  .    f  .    f
e    f  .    f  e    .  .    f  .    f
gggg    ....    gggg    gggg    ....

 5:      6:      7:      8:      9:
aaaa    aaaa    aaaa    aaaa    aaaa
b    .  b    .  .    c  b    c  b    c
b    .  b    .  .    c  b    c  b    c
dddd    dddd    ....    dddd    dddd
.    f  e    f  .    f  e    f  .    f
.    f  e    f  .    f  e    f  .    f
gggg    gggg    ....    gggg    gggg */
fn main() {
    let commands: Vec<String> = get_input(false);

    let mut grid: Vec<Vec<(i32, bool)>> = commands
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| (x.to_digit(10).unwrap() as i32, false))
                .collect_vec()
        })
        .collect_vec();
    let mut flash_count = 0;
    for i in 1..=1000000 {
        for (_, row) in grid.iter_mut().enumerate() {
            for (_, (value, flashed)) in row.iter_mut().enumerate() {
                *value = *value + 1;
                *flashed = false;
            }
        }
        //Check any is > 9
        while grid
            .iter()
            .flatten()
            .filter(|x| x.0 > 9 && x.1 == false)
            .count()
            > 0
        {
            let directions = vec![
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];
            let mut just_flashed = Vec::new();
            for (rownum, row) in grid.iter_mut().enumerate() {
                for (colnum, (value, flashed)) in row.iter_mut().enumerate() {
                    if value > &mut 9 && !*flashed {
                        *value += 1;
                        *flashed = true;
                        flash_count += 1;
                        just_flashed.push((rownum, colnum));
                    }
                }
            }
            for (directionx, directiony) in directions {
                for (flashedx, flashedy) in &just_flashed {
                    let flashedxtoinc = *flashedx as i32 + directionx;
                    let flashedytoinc = *flashedy as i32 + directiony;
                    if 0 <= flashedxtoinc
                        && flashedxtoinc < 10
                        && 0 <= flashedytoinc
                        && flashedytoinc < 10
                    {
                        grid[flashedxtoinc as usize][flashedytoinc as usize].0 += 1;
                    }
                }
            }
        }
        for (_, row) in grid.iter_mut().enumerate() {
            for (_, (value, flashed)) in row.iter_mut().enumerate() {
                if *value > 9 {
                    *value = 0;
                }
            }
        }
        //println!("After Iteration {}, grid {:?}", i, grid);
        if grid.iter().flatten().filter(|x| x.1 == true).count() == 100 {
            println!("Iteration {} ", i);
            break;
        }
    }
    println!("Answer is {}", flash_count);
    println!("{:?}", commands);
}
