use std::collections::HashMap;

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
    println!("Answer 1 is {}", sol1(&commands));
    println!("Answer 2 is {}", sol2(&commands));
}
fn sol2(commands: &[String]) -> i64 {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');
    let mut syntax_pen = HashMap::new();
    syntax_pen.insert('(', 1);
    syntax_pen.insert('[', 2);
    syntax_pen.insert('{', 3);
    syntax_pen.insert('<', 4);

    let mut result = Vec::new();
    for command in commands {
        let mut vec = Vec::new();
        let mut invalid = false;
        for chr in command.chars() {
            if map.get(&chr).is_some() {
                vec.push(chr);
            } else {
                let bracket = vec.pop();
                if bracket.is_some() {
                    if chr != *map.get(&bracket.unwrap()).unwrap() {
                        invalid = true;
                        break;
                    }
                }
            }
        }
        let mut this_result = 0;
        if !invalid {
            while let Some(x) = vec.pop() {
                this_result *= 5;
                this_result += syntax_pen.get(&x).unwrap();
            }
            result.push(this_result);
        }
    }
    println!("Results {:?} ", result);
    result.sort();
    result[result.len() / 2]
}

fn sol1(commands: &[String]) -> i32 {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');
    map.insert('<', '>');
    let mut syntax_pen = HashMap::new();
    syntax_pen.insert(')', 3);
    syntax_pen.insert(']', 57);
    syntax_pen.insert('}', 1197);
    syntax_pen.insert('>', 25137);

    let mut result = 0;
    for command in commands {
        let mut vec = Vec::new();
        for chr in command.chars() {
            if map.get(&chr).is_some() {
                vec.push(chr);
            } else {
                let bracket = vec.pop();
                if bracket.is_some() {
                    if chr != *map.get(&bracket.unwrap()).unwrap() {
                        result += syntax_pen.get(&chr).unwrap();
                    }
                }
            }
        }
    }
    result
}
