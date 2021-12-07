use std::env;
use std::fs::File;
use std::io::{self, BufRead};

pub fn get_input(test: bool) -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let day_param = &args[0].split('/').last().unwrap();
    let day = day_param.split_once('-').unwrap_or((day_param, "")).0;
    let test = if !test && (args.get(1).is_none() || !args.get(1).unwrap().eq("test")) {
        ""
    } else {
        "_test"
    };
    println!("{}", format!("input/{}{}.txt", day, test));
    let file = File::open(format!("input/{}{}.txt", day, test)).unwrap();
    io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect()
}
