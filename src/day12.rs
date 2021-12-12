use aoc2021::files::get_input;
use itertools::Itertools;

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);
    println!("{:?}", parse_input(&commands));
    let paths = parse_input(&commands);

    println!("pt 1 {}", explore_paths(&paths, false));
    println!("pt 2 {}", explore_paths(&paths, true));
}

fn explore_paths(paths: &Vec<Path>, pt2: bool) -> usize {
    let mut explored_paths: Vec<Vec<String>> = Vec::new();
    let mut valid_paths: Vec<Vec<String>> = Vec::new();
    //Starting caves
    for cave in paths
        .iter()
        .filter_map(|x| x.return_other_cave("start".to_string()))
        .collect_vec()
    {
        explored_paths.push(vec!["start".to_string(), cave]);
    }
    // println!("Explored paths at start is {:?}", explored_paths);
    while let Some(x) = &explored_paths.pop() {
        // println!("Exploring from {:?}", x);
        let caves_from_last = paths
            .iter()
            .filter_map(|p| p.return_other_cave(x.last().unwrap().clone()))
            .collect_vec();
        // println!("Found caves to explore {:?}", caves_from_last);
        for cave in &caves_from_last {
            let mut copy_path = x.clone();
            copy_path.push(cave.to_string());
            // println!("new path to check {:?}", copy_path);
            if cave == "end" {
                valid_paths.push(copy_path);
            } else {
                if pt2 && path_valid_pt_2(&copy_path) || !pt2 && path_valid(&copy_path) {
                    //      println!("Adding on  {:?} to explored paths", copy_path);
                    explored_paths.push(copy_path);
                }
            }
        }
    }

    valid_paths.len()
}
fn path_valid(path: &Vec<String>) -> bool {
    let mut last = "";
    for s in path.iter().sorted() {
        if s == last && (s == "start" || s.to_ascii_lowercase() == s.to_string()) {
            return false;
        }
        last = s;
    }
    true
}

fn path_valid_pt_2(path: &Vec<String>) -> bool {
    let mut last = "";
    let mut small_twice = false;
    for s in path.iter().sorted() {
        if s == last && s == "start" {
            return false;
        }
        if s == last && s.to_ascii_lowercase() == s.to_string() {
            if small_twice {
                return false;
            }
            small_twice = true;
        }

        last = s;
    }
    true
}

#[derive(PartialEq, Debug)]
struct Path {
    cave1: String,
    cave2: String,
}

impl Path {
    fn return_other_cave(&self, name: String) -> Option<String> {
        if self.cave1 == name {
            return Some(self.cave2.clone());
        }
        if self.cave2 == name {
            return Some(self.cave1.clone());
        }
        None
    }
}

fn parse_input(commands: &[String]) -> Vec<Path> {
    commands
        .iter()
        .map(|x| create_path(x.split_once('-').unwrap()))
        .collect_vec()
}

fn create_path((path1, path2): (&str, &str)) -> Path {
    Path {
        cave1: path1.to_string(),
        cave2: path2.to_string(),
    }
}
