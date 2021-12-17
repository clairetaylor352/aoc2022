use aoc2021::files::get_input;

fn main() {
    let commands: Vec<String> = get_input(false);
    println!("{:?}", commands);
    let xmin = 155;
    let xmax = 215;
    let ymin = -132;
    let ymax = -72;
    let mut valid_yspeeds = Vec::new();
    //we'll come back to 0 at the same speed we go up, so we'll over shoot at more than -ymin speed
    for yspeed in ymin..=-ymin {
        let mut path = Path {
            xspeed: 0,
            yspeed,
            x: 0,
            y: 0,
        };
        if path.yspeedhits(ymin, ymax) {
            println!("Can hit target at yspeed {}", yspeed);
            valid_yspeeds.push(yspeed);
        }
    }
    let mut valid_xspeeds = Vec::new();
    //gonna overshoot if we go more than xmax
    for xspeed in 1..=xmax {
        let mut path = Path {
            xspeed,
            yspeed: 0,
            x: 0,
            y: 0,
        };
        if path.xspeedhits(xmin, xmax) {
            println!("Can hit target at xspeed {}", xspeed);
            valid_xspeeds.push(xspeed);
        }
    }
    let mut count = 0;
    for yspeed in &valid_yspeeds {
        for xspeed in &valid_xspeeds {
            let mut path = Path {
                xspeed: *xspeed,
                yspeed: *yspeed,
                x: 0,
                y: 0,
            };
            if path.hits(xmin, xmax, ymin, ymax) {
                println!(
                    "Can hit target at yspeed {}, xspeed {}, max_height",
                    yspeed, xspeed
                );
                count += 1;
            }
        }
    }

    println!("Valid number of paths {}", count);

    //hack, cos I can't be bothered
    let mut path = Path {
        xspeed: 18,
        yspeed: 131,
        x: 0,
        y: 0,
    };
    println!("Path max height {}", path.max_height());
}

struct Path {
    xspeed: i32,
    yspeed: i32,
    x: i32,
    y: i32,
}

impl Iterator for Path {
    // We can refer to this type using Self::Item
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.x = self.x + self.xspeed;
        self.y = self.y + self.yspeed;
        self.xspeed = if self.xspeed == 0 { 0 } else { self.xspeed - 1 };
        self.yspeed = self.yspeed - 1;

        Some((self.x, self.y))
    }
}

impl Path {
    fn hits(&mut self, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> bool {
        while let Some((x, y)) = self.next() {
            if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
                return true;
            }
            if x > xmax || y < ymin {
                return false;
            }
        }
        false
    }

    fn yspeedhits(&mut self, ymin: i32, ymax: i32) -> bool {
        while let Some((_, y)) = self.next() {
            if y >= ymin && y <= ymax {
                return true;
            }
            if y < ymin {
                return false;
            }
        }
        false
    }

    fn xspeedhits(&mut self, xmin: i32, xmax: i32) -> bool {
        while let Some((x, _)) = self.next() {
            if x >= xmin && x <= xmax {
                return true;
            }
            if x > xmax || self.xspeed == 0 {
                return false;
            }
        }
        false
    }

    fn max_height(&mut self) -> i32 {
        let mut last_y = 0;
        while let Some((_, y)) = self.next() {
            if y < last_y {
                return last_y;
            }
            last_y = y;
        }
        last_y
    }
}
