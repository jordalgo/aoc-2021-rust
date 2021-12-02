use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./day1-input") {
        let mut prev: u32 = 0;
        let mut start = false;
        let mut total: u32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let my_u32: u32 = ip.parse::<u32>().unwrap();
                if start && prev < my_u32 {
                  total = total + 1;
                }
                start = true;
                prev = my_u32;
            }
        }
        println!("total {}", total);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
