use aoc2021::files::get_input;
use itertools::Itertools;

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);
    //DJikistra, original risk, in set, distance (none is infinity), previous node
    /*     let mut grid: Vec<Vec<(i32, bool, Option<i32>, Option<(i32, i32)>)>> = commands
    .iter()
    .map(|x| {
        x.chars()
            .map(|x| (x.to_digit(10).unwrap() as i32, true, None, None))
            .collect_vec()
    })
    .collect_vec(); */
    let mut grid = generate_big_grid(commands, 5);
    let ylen = grid.len() as i32;
    let xlen = grid[0].len() as i32;
    println!("xlen {}, ylen{}", &xlen, &ylen);

    grid[0][0].2 = Some(0);
    grid[0][0].1 = false;

    let directions = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let (mut x, mut y) = (0 as i32, 0 as i32);
    while (true) {
        for (i, j) in &directions {
            //Can we go in this direction?
            if 0 <= x + i && x + i < xlen && 0 <= y + j && y + j < ylen {
                let item = grid[(x + i) as usize][(y + j) as usize];
                //is this item still in the set?
                if item.1 {
                    let alt = grid[x as usize][y as usize].2.unwrap() + item.0;
                    if let Some(dist) = item.2 {
                        if alt < dist {
                            grid[(x + i) as usize][(y + j) as usize].2 = Some(alt);
                            grid[(x + i) as usize][(y + j) as usize].3 = Some((x, y));
                        }
                    } else {
                        grid[(x + i) as usize][(y + j) as usize].2 = Some(alt);
                        grid[(x + i) as usize][(y + j) as usize].3 = Some((x, y));
                    }
                }
            }
        }
        let mut min_dist = i32::MAX;
        let mut min_coords = (i32::MAX, i32::MAX);
        for i in 0..xlen {
            for j in 0..ylen {
                if grid[i as usize][j as usize].2.unwrap_or(i32::MAX) < min_dist
                    && grid[i as usize][j as usize].1
                {
                    min_dist = grid[i as usize][j as usize].2.unwrap();
                    min_coords = (i, j);
                }
            }
        }
        grid[min_coords.0 as usize][min_coords.1 as usize].1 = false;
        x = min_coords.0;
        y = min_coords.1;

        if !grid[(xlen - 1) as usize][(ylen - 1) as usize].1 {
            println!(
                "Now looking from x {}, y {}, which has a min dist of {}",
                x, y, min_dist
            );
            break;
        }
    }
    x = xlen - 1;
    y = ylen - 1;
    let mut min_dist = 0;

    while (true) {
        min_dist += grid[x as usize][y as usize].0;
        x = grid[x as usize][y as usize].3.unwrap().0;
        y = grid[x as usize][y as usize].3.unwrap().1;
        if x == 0 && y == 0 {
            break;
        }
    }
    println!("Answer is {}", min_dist);
}
fn duplicate_row(x: String, times: i32) -> Vec<i32> {
    let mut row = x
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect_vec();
    let orig_row = row.clone();
    for i in 0..times - 1 {
        let mut new_row = orig_row
            .clone()
            .iter()
            .map(|x| (x + &i) % 9 + 1)
            .collect_vec();
        row.append(&mut new_row);
    }
    row
}

fn generate_big_grid(
    commands: Vec<String>,
    times: i32,
) -> Vec<Vec<(i32, bool, Option<i32>, Option<(i32, i32)>)>> {
    let mut top_rows = commands
        .iter()
        .map(|x| duplicate_row(x.to_string(), times))
        .collect_vec();
    let orig_top_rows = top_rows.clone();
    println!("Top rows len {} len {}", top_rows.len(), top_rows[0].len());
    println!("{:?}", top_rows);
    for i in 0..times - 1 {
        let mut new_rows = orig_top_rows
            .clone()
            .iter()
            .map(|x| x.iter().map(|x| (x + &i) % 9 + 1).collect_vec())
            .collect_vec();
        top_rows.append(&mut new_rows);
    }
    println!("Top rows {:?}", top_rows);
    top_rows
        .iter()
        .map(|x| x.iter().map(|x| (*x, true, None, None)).collect_vec())
        .collect_vec()
}
