use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[0].split('/').last().unwrap();
    let test = if args.get(1).is_none() || !args.get(1).unwrap().eq("test") {
        ""
    } else {
        "_test"
    };
    let file = File::open(format!("input/{}{}.txt", day, test)).unwrap();
    let commands: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();

    println!("Part 1 result is {}", sol1(&commands));
    println!("Part 2 result is {}", sol2(&commands));
}

struct Field {
    found: bool,
    num: i32,
}

struct Row {
    row: Vec<Field>,
}

struct Board {
    rows: Vec<Row>,
    won: bool,
}
impl Board {
    fn mark_numbers_found(&mut self, num: i32) {
        for row in &mut self.rows {
            for field in &mut row.row {
                if field.num == num {
                    field.found = true;
                    println!("Found an instance of {}", num);
                }
            }
        }
    }

    fn find_rows_or_columns(&mut self, num: i32) -> i32 {
        let mut cols_found = vec![true; self.rows.len()];
        let mut non_selected_sum = 0;
        let mut this_row_found;
        let mut a_row_found = false;
        for (_, row) in self.rows.iter_mut().enumerate() {
            this_row_found = true;
            for (colnum, field) in row.row.iter_mut().enumerate() {
                //println!("Checking row{}, col{}", rownum, colnum);

                if !field.found {
                    this_row_found = false;
                    cols_found[colnum] = false;

                    non_selected_sum += field.num;
                }
            }
            if this_row_found {
                a_row_found = true;
            }
        }
        if a_row_found || cols_found.contains(&true) {
            println!("Found num{} sum is{}", num, non_selected_sum);
            return non_selected_sum * num;
        }
        0
    }
}

fn sol1(commands: &[String]) -> i32 {
    let (numbers, mut boards) = process_input(commands);
    for num in numbers {
        println!("Setting numbers {}", num);
        for board in &mut boards {
            board.mark_numbers_found(num);
            let find = board.find_rows_or_columns(num);
            if find != 0 {
                return find;
            }
        }
    }
    0
}
fn process_input(commands: &[String]) -> (Vec<i32>, Vec<Board>) {
    println!("{}", commands[0]);
    let numbers = commands[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut boards = Vec::new();
    let mut board = Board {
        rows: Vec::new(),
        won: false,
    };
    for command in &commands[2..] {
        //Make the boards
        if command.len() == 0 {
            boards.push(board);
            board = Board {
                rows: Vec::new(),
                won: false,
            };
        } else {
            let mut row_struct = Row { row: Vec::new() };
            row_struct.row = command
                .split_whitespace()
                .map(|x| Field {
                    num: x.parse::<i32>().unwrap(),
                    found: false,
                })
                .collect::<Vec<_>>();
            board.rows.push(row_struct);
        }
    }
    boards.push(board);
    (numbers, boards)
}

fn sol2(commands: &[String]) -> i32 {
    let (numbers, mut boards) = process_input(commands);
    for num in numbers {
        let num_boards = boards.iter().filter(|b| !b.won).count();
        println!("Number of boards left {}", num_boards);
        println!("Setting numbers {}", num);
        for board in &mut boards {
            if !board.won {
                board.mark_numbers_found(num);
                let find = board.find_rows_or_columns(num);
                if find != 0 {
                    if num_boards == 1 {
                        return find;
                    }
                    board.won = true;
                }
            }
        }
    }
    0
}
