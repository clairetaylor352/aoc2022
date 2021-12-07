use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn get_input() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let day = &args[0].split('/').last().unwrap();
    let test = if args.get(1).is_none() || !args.get(1).unwrap().eq("test") {
        ""
    } else {
        "_test"
    };
    let file = File::open(format!("input/{}{}.txt", day, test)).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect()
}
