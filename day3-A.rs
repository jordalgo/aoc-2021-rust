use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut ones: [u32; 12] = [0; 12];
    let mut zeros: [u32; 12] = [0; 12];
    if let Ok(lines) = read_lines("./day3-input") {
        for line in lines {
            if let Ok(l) = line {
                // println!("line {}", l);
                let bits: Vec<char> = l.chars().collect();
                for n in 0..12 {
                  //println!("bit {}", bit);
                  if bits[n] == '0' {
                    //println!("zero");
                    zeros[n] = zeros[n] + 1;
                  } else {
                    ones[n] = ones[n] + 1;
                    //println!("one");
                  }
                }
            }
        }
    }
    let mut gamma = [""; 12];
    let mut epsilon = [""; 12];
    for n in 0..12 {
      if ones[n] > zeros[n] {
        gamma[n] = "1";
        epsilon[n] = "0";
      } else {
        gamma[n] = "0";
        epsilon[n] = "1";
      }
    }
    let gamma_decimal = isize::from_str_radix(&gamma.join(""), 2).unwrap();
    //println!("{}", gamma_decimal);
    let epsilon_decimal = isize::from_str_radix(&epsilon.join(""), 2).unwrap();
    //println!("{}", epsilon_decimal);
    println!("total {}", gamma_decimal * epsilon_decimal);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
