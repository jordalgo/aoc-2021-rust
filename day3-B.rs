use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn get_next_vec(lines: &Vec<String>, position: usize, most: bool) -> Vec<String> {
  let mut zeroes = Vec::new();
  let mut ones = Vec::new();
  for line in lines {
    let bits: Vec<char> = line.chars().collect();
    if bits[position] == '0' {
      zeroes.push(line.to_string());
    } else {
      ones.push(line.to_string());
    }
  }
  let ones_len = ones.len();
  let zeroes_len = zeroes.len();
  if ones_len >= zeroes_len {
    if most {
      if ones_len == 1 {
        return ones;
      } else {
        return get_next_vec(&ones, position + 1, most);
      }
    } else {
      if zeroes_len == 1 {
        return zeroes;
      } else {
        return get_next_vec(&zeroes, position + 1, most);
      }
    }
  } else {
    if most {
      if zeroes_len == 1 {
        return zeroes;
      } else {
        return get_next_vec(&zeroes, position + 1, most);
      }
    } else {
      if ones_len == 1 {
        return ones;
      } else {
        return get_next_vec(&ones, position + 1, most);
      }
    }
  }
}

fn main() {
    let mut vec = Vec::new();
    if let Ok(lines) = read_lines("./day3-input") {
        for line in lines {
            if let Ok(l) = line {
                vec.push(l);
            }
        }
    }

    let oxygen = get_next_vec(&vec, 0, true);
    let oxygen_decimal = isize::from_str_radix(oxygen.get(0).unwrap(), 2).unwrap();
    // println!("oxygen {}", oxygen_decimal);

    let scrubber = get_next_vec(&vec, 0, false);
    let scrubber_decimal = isize::from_str_radix(scrubber.get(0).unwrap(), 2).unwrap();
    // println!("scrubber {}", scrubber_decimal);

    println!("total {}", scrubber_decimal * oxygen_decimal);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
