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

fn sol1(commands: &Vec<String>) -> i32 {
    let mut vec_count: Vec<usize> = vec![0; commands[0].len()];

    for command in commands {
        for charen in command.chars().enumerate() {
            if charen.1 == '1' {
                vec_count[charen.0] += 1;
            }
        }
    }
    println!("{:?}", vec_count);
    let threshold = commands.len() / 2;
    let mut epsilon = 0;
    let mut gamma = 0;
    let string_size = vec_count.len();
    for en in vec_count.iter().enumerate() {
        if vec_count[en.0] > threshold {
            gamma += 2_i32.pow(string_size as u32 - en.0 as u32 - 1);
        } else {
            epsilon += 2_i32.pow(string_size as u32 - en.0 as u32 - 1);
        }
    }

    println!("Gamma {}", gamma);
    println!("Epsilon {}", epsilon);
    gamma * epsilon
}
fn sol2(commands: &Vec<String>) -> i64 {
    let mut command_clone = commands.clone();
    let gamma = filter_vector(&mut command_clone, 0, true);
    let epsilon = filter_vector(&mut commands.to_vec(), 0, false);
    println!("Gamma {}", gamma);
    println!("Epsilon {}", epsilon);
    gamma * epsilon
}
fn filter_vector(commands: &mut Vec<String>, index: usize, gamma: bool) -> i64 {
    let mut ones = 0;
    let command_size = commands.len();
    for command in &*commands {
        if command.chars().nth(index).unwrap() == '1' {
            ones += 1;
        }
    }
    let mut comp_char = '0';
    let zeros = command_size - ones;
    if ones >= zeros && gamma {
        comp_char = '1';
    }
    if ones < zeros && !gamma {
        comp_char = '1';
    }
    commands.retain(|c| c.chars().nth(index).unwrap() == comp_char);
    println!(
        "Gamma {}, index {}, new commandsize {}, comparison char {}",
        gamma,
        index,
        commands.len(),
        comp_char
    );
    println!("{:?}", commands);
    if commands.len() == 1 {
        println!("Remaining command {}", &commands[0]);
        println!(
            "Integer value {}",
            i64::from_str_radix(&commands[0], 2).unwrap()
        );
        return i64::from_str_radix(&commands[0], 2).unwrap();
    }
    filter_vector(commands, index + 1, gamma)
}
