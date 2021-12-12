use aoc2021::files::get_input;
use itertools::Itertools;

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);

    println!("Answer 1 is {}", sol1(&commands));

    println!("Answer 2 is {}", sol2(&commands));
}

struct Coord {
    row: usize,
    col: usize,
}
fn mark_block(coord: &Coord, grid: &mut Vec<Vec<i32>>) -> i32 {
    if grid[coord.row][coord.col] == 9 {
        return 0;
    }
    if grid[coord.row][coord.col] > 0 {
        grid[coord.row][coord.col] = -1;
        return 1;
    }
    0
}
fn get_surrounding_blocks(coord: Coord, grid: Vec<Vec<i32>>) -> Vec<Coord> {
    let rowcount = grid.len();
    let colcount = &grid[0].len();
    let mut list = Vec::new();
    if coord.row > 0 {
        list.push(Coord {
            row: coord.row - 1,
            col: coord.col,
        });
    }
    if coord.row < rowcount - 1 {
        list.push(Coord {
            row: coord.row + 1,
            col: coord.col,
        });
    }
    if coord.col > 0 {
        list.push(Coord {
            row: coord.row,
            col: coord.col - 1,
        });
    }

    if coord.col < colcount - 1 {
        list.push(Coord {
            row: coord.row,
            col: coord.col + 1,
        });
    }
    list
}

fn mark_surrounding_blocks(coord: Coord, grid: &mut Vec<Vec<i32>>) -> i32 {
    let mut size = 0;
    let surrounding_blocks = get_surrounding_blocks(coord, grid.to_vec());
    for block in surrounding_blocks {
        if mark_block(&block, grid) == 1 {
            size += 1 + mark_surrounding_blocks(block, grid);
        }
    }
    size
}

fn sol2(commands: &[String]) -> i32 {
    let mut input: Vec<Vec<i32>> = commands
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let rowcount = input.len();
    let colcount = &input[0].len();
    let mut bottom_coords = Vec::new();
    for (row, rowdata) in input.iter().enumerate() {
        for (col, item) in rowdata.iter().enumerate() {
            let mut bottom = true;
            if row > 0 {
                if item >= &input[row - 1][col] {
                    bottom = false;
                }
                // Check less than the one above
            }
            if row < rowcount - 1 {
                if item >= &input[row + 1][col] {
                    bottom = false;
                }
                // check less than the one below
            }
            if col > 0 {
                if item >= &input[row][col - 1] {
                    bottom = false;
                }

                //check less than the one to left
            }
            if col < colcount - 1 {
                //check less than the one to right
                if item >= &input[row][col + 1] {
                    bottom = false;
                }
            }
            if bottom {
                bottom_coords.push(Coord { row, col });
            }
        }
    }
    let mut sizes = Vec::new();
    for coord in bottom_coords {
        mark_block(&coord, &mut input);
        sizes.push(1 + mark_surrounding_blocks(coord, &mut input));
    }
    println!("Input {:?}", input);
    println!("Sizes {:?}", sizes);
    sizes.sort();
    sizes.reverse();
    sizes[0..3].iter().product()
}

fn sol1(commands: &[String]) -> u32 {
    let input = commands
        .iter()
        .map(|x| {
            x.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let rowcount = input.len();
    let colcount = &input[0].len();
    let mut riskscore = 0;
    for (row, rowdata) in input.iter().enumerate() {
        for (col, item) in rowdata.iter().enumerate() {
            let mut bottom = true;
            if row > 0 {
                if item >= &input[row - 1][col] {
                    bottom = false;
                }
                // Check less than the one above
            }
            if row < rowcount - 1 {
                if item >= &input[row + 1][col] {
                    bottom = false;
                }
                // check less than the one below
            }
            if col > 0 {
                if item >= &input[row][col - 1] {
                    bottom = false;
                }

                //check less than the one to left
            }
            if col < colcount - 1 {
                //check less than the one to right
                if item >= &input[row][col + 1] {
                    bottom = false;
                }
            }
            if bottom {
                riskscore += item + 1;
            }
        }
    }
    println!("Answer 1 is {}", riskscore);
    riskscore
}
