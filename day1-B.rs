use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn sum_arr(arr: &[u32; 3]) -> u32 {
    arr.iter().fold(0, |acc, x| acc + x)
}

fn main() {
    if let Ok(lines) = read_lines("./day1-input") {

        let mut a: [u32; 3] = [0, 0, 0];
        let mut b: [u32; 3] = [0, 0, 0];
        let mut c: [u32; 3] = [0, 0, 0];
        let mut d: [u32; 3] = [0, 0, 0];

        let mut idx: u32 = 0;
        let mut total: u32 = 0;
        let mut start = false;

        for line in lines {
            if let Ok(ip) = line {
                let next: u32 = ip.parse::<u32>().unwrap();
                if idx == 0 {
                    a[0] = next;
                    c[2] = next;
                    d[1] = next;

                    if start && sum_arr(&b) < sum_arr(&c){
                        total = total + 1;
                    }
                } else if idx == 1 {
                    a[1] = next;
                    b[0] = next;
                    d[2] = next;

                    if start && sum_arr(&c) < sum_arr(&d) {
                        total = total + 1;
                    }
                }  else if idx == 2 {
                    a[2] = next;
                    b[1] = next;
                    c[0] = next;

                    if start && sum_arr(&d) < sum_arr(&a) {
                        total = total + 1;
                    }
                } else if idx == 3 {
                    b[2] = next;
                    c[1] = next;
                    d[0] = next;
                    if sum_arr(&a) < sum_arr(&b) {
                        total = total + 1;
                    }
                }

                if idx == 3 {
                    idx = 0;
                    start = true;
                } else {
                    idx = idx + 1;
                }
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
