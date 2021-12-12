use std::collections::HashSet;

use aoc2021::files::get_input;
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
    println!("{:?}", commands);
    println!("Sol 1 is {}", sol1(&commands));
    println!("Sol 2 is {}", sol2(&commands));
}
fn sol2(commands: &[String]) -> i32 {
    let split_commands = commands
        .iter()
        .map(|x| x.split_once('|').unwrap())
        .collect::<Vec<_>>();

    let split_again = split_commands
        .iter()
        .map(|x| {
            (
                x.0.trim().split(' ').collect::<Vec<_>>(),
                x.1.trim().split(' ').collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut count = 0;
    for row in split_again {
        count += calculate_row(row);
    }
    count
}
#[derive(Debug)]
struct Digit {
    digit: HashSet<char>,
    value: i32,
}
fn calculate_row(row: (Vec<&str>, Vec<&str>)) -> i32 {
    //Do 1, 8, 7 and 4
    let mut answers: Vec<Digit> = Vec::new();
    //Find 8, 1, 2, 4

    let mut digit_list: Vec<HashSet<char>> = Vec::new();
    println!("Row {:?}", row);
    for item in row.0 {
        digit_list.push(item.chars().collect());
    }
    println!("Digit list {:?}", digit_list);
    let one = digit_list
        .iter()
        .filter(|x| x.len() == 2)
        .next()
        .unwrap()
        .clone();

    let eight = digit_list
        .iter()
        .filter(|x| x.len() == 7)
        .next()
        .unwrap()
        .clone();

    let seven = digit_list
        .iter()
        .filter(|x| x.len() == 3)
        .next()
        .unwrap()
        .clone();

    let four = digit_list
        .iter()
        .filter(|x| x.len() == 4)
        .next()
        .unwrap()
        .clone();

    let three = digit_list
        .iter()
        .filter(|x| x.len() == 5 && seven.intersection(x).count() == 3)
        .next()
        .unwrap()
        .clone();

    let five = digit_list
        .iter()
        .filter(|x| x.len() == 5 && four.intersection(x).count() == 3 && !(**x == three))
        .next()
        .unwrap()
        .clone();

    let two = digit_list
        .iter()
        .filter(|x| x.len() == 5 && !(**x == three) && !(**x == five))
        .next()
        .unwrap()
        .clone();

    let nine = digit_list
        .iter()
        .filter(|x| x.len() == 6 && four.intersection(x).count() == 4)
        .next()
        .unwrap()
        .clone();

    let six = digit_list
        .iter()
        .filter(|x| x.len() == 6 && five.intersection(x).count() == 5 && !(**x == nine))
        .next()
        .unwrap()
        .clone();

    let zero = digit_list
        .iter()
        .filter(|x| x.len() == 6 && **x != nine && **x != six)
        .next()
        .unwrap()
        .clone();
    answers.push(Digit {
        digit: one,
        value: 1,
    });
    answers.push(Digit {
        digit: eight,
        value: 8,
    });
    answers.push(Digit {
        digit: seven,
        value: 7,
    });
    answers.push(Digit {
        digit: four,
        value: 4,
    });
    answers.push(Digit {
        digit: three,
        value: 3,
    });
    answers.push(Digit {
        digit: five,
        value: 5,
    });
    answers.push(Digit {
        digit: six,
        value: 6,
    });
    answers.push(Digit {
        digit: zero,
        value: 0,
    });
    answers.push(Digit {
        digit: two,
        value: 2,
    });
    answers.push(Digit {
        digit: nine,
        value: 9,
    });

    let mut ten_power = 4;
    let mut answer = 0;
    println!("Digits {:?} ", answers);

    for item in row.1 {
        let chars: HashSet<char> = item.chars().collect();
        println!("Looking for {:?}", chars);
        ten_power -= 1;
        answer += 10_i32.pow(ten_power)
            * answers
                .iter()
                .filter(|x| x.digit == chars)
                .next()
                .unwrap()
                .value;
    }
    println!("Answer is {}", answer);
    answer
}
//println!("Solution 1 is: {} ", minimum_fuel(&commands[0], false));
//println!("Solution 2 is: {} ", minimum_fuel(&commands[0], true));
fn sol1(commands: &[String]) -> i32 {
    println!("{:?}", commands);
    let split_commands = commands
        .iter()
        .map(|x| x.split_once('|').unwrap())
        .collect::<Vec<_>>();
    let split_second_commands = split_commands
        .iter()
        .map(|x| x.1.trim().split(' '))
        .flatten()
        .collect::<Vec<_>>();

    split_second_commands
        .iter()
        .map(|cmd| {
            if cmd.len() == 2 || cmd.len() == 3 || cmd.len() == 7 || cmd.len() == 4 {
                return 1;
            } else {
                return 0;
            }
        })
        .sum::<i32>()
}
