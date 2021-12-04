use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut horizontal: u32 = 0;
    let mut vertical: u32 = 0;
    let mut aim: u32 = 0;
    if let Ok(lines) = read_lines("./day2-input") {
        for line in lines {
            if let Ok(l) = line {
                // println!("line {}", l);
                let v: Vec<&str> = l.split(' ').collect();
                // println!("part1 {}", v[0]);
                // println!("part2 {}", v[1]);
                let val: u32 = v[1].parse::<u32>().unwrap();
                match v[0] {
                    "forward" => {
                        horizontal = horizontal + val;
                        vertical = vertical + (aim * val);
                    },
                    "down" => aim = aim + val,
                    "up" => aim = aim - val,
                    _ => println!("Input does not equal any value"),
                }
            }
        }
    }
    println!("total {}", horizontal * vertical);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
